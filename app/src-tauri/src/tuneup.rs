//! Tune-Up: free, automatic health checks (no AI). Scans each game's Luau for
//! bug and lag patterns, reviews the built map for heavy content, applies
//! safe fixes on request, and clears old temp files from the studio folder.
use regex::Regex;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

fn luau_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        if p.is_dir() {
            luau_files(&p, out);
        } else if matches!(p.extension().and_then(|x| x.to_str()), Some("luau") | Some("lua")) {
            out.push(p);
        }
    }
    out.sort();
}

fn f(sev: &str, file: &str, line: usize, what: &str, fix: &str) -> Value {
    json!({ "severity": sev, "file": file, "line": line, "what": what, "fix": fix })
}

/// Line-level patterns: (regex, severity, what, fix).
fn rules() -> Vec<(Regex, &'static str, &'static str, &'static str)> {
    let r = |s: &str| Regex::new(s).unwrap();
    vec![
        (r(r"(^|[^.:\w])wait\s*\("), "minor", "wait() is deprecated and slower", "use task.wait() (Fix Safe Items does this)"),
        (r(r"(^|[^.:\w])spawn\s*\("), "minor", "spawn() is deprecated and can delay", "use task.spawn() (Fix Safe Items does this)"),
        (r(r"(^|[^.:\w])delay\s*\("), "minor", "delay() is deprecated", "use task.delay() (Fix Safe Items does this)"),
        (r(r"while\s+(task\.)?wait\s*\(\s*\)\s+do"), "major", "busy loop that runs every frame forever", "use an event, or wait a real interval like task.wait(0.5)"),
        (r(r#"Instance\.new\(\s*["']\w+["']\s*,"#), "minor", "Instance.new with a parent argument is slow", "set properties first, then set Parent last"),
        (r(r"\bloadstring\s*\("), "blocker", "loadstring runs arbitrary code", "remove it"),
        (r(r":InvokeClient\s*\("), "major", "InvokeClient can hang the server if the player leaves or cheats", "use a RemoteEvent and a timeout"),
        (r(r"\bgetfenv\b|\bsetfenv\b"), "major", "getfenv/setfenv turn off Luau optimisations", "remove them"),
        (r(r"\.Changed:Connect"), "minor", "Changed fires for every property", "use GetPropertyChangedSignal for the one property"),
    ]
}

/// Scan one project's Luau. Returns findings and a small summary.
pub fn scan_code(proj: &Path) -> (Vec<Value>, Value) {
    let mut files = vec![];
    luau_files(&proj.join("src"), &mut files);
    let rules = rules();
    let per_frame = Regex::new(r"(RenderStepped|Heartbeat|Stepped|PreRender|PreSimulation|PostSimulation):Connect").unwrap();
    let heavy = Regex::new(r":GetDescendants\(\)|:GetChildren\(\)|:FindFirstChild\(|Instance\.new\(|:Clone\(\)").unwrap();
    let mut out = vec![];
    let mut hashes: HashMap<String, String> = HashMap::new();
    let mut names: Vec<(String, String)> = vec![]; // (module name, rel path)
    let mut all_text = String::new();
    let (mut lines_total, mut prints) = (0, 0);
    for p in &files {
        let rel = p.strip_prefix(proj).unwrap_or(p).to_string_lossy().replace('\\', "/");
        let Ok(t) = fs::read_to_string(p) else { continue };
        if t.trim().is_empty() {
            out.push(f("minor", &rel, 1, "empty file", "delete it"));
            continue;
        }
        if let Some(first) = hashes.insert(t.clone(), rel.clone()) {
            out.push(f("minor", &rel, 1, &format!("same content as {first}"), "keep one copy and require it from both places"));
        }
        let n = t.lines().count();
        lines_total += n;
        if n > 800 {
            out.push(f("minor", &rel, 1, &format!("{n} lines in one file"), "split it into modules by job"));
        }
        let (mut connects, mut disconnects, mut frame_depth) = (0, 0, 0usize);
        for (i, raw) in t.lines().enumerate() {
            let l = raw.split("--").next().unwrap_or("");
            if l.trim().is_empty() {
                continue;
            }
            for (re, sev, what, fix) in &rules {
                if re.is_match(l) {
                    out.push(f(sev, &rel, i + 1, what, fix));
                }
            }
            if l.contains("print(") && !rel.contains("test") {
                prints += 1;
            }
            connects += l.matches(":Connect(").count();
            disconnects += l.matches(":Disconnect(").count() + l.matches("Destroy()").count() + l.matches(":Once(").count();
            // Work done inside a per-frame callback (rough: the next 25 lines).
            if per_frame.is_match(l) {
                frame_depth = 25;
            } else if frame_depth > 0 {
                frame_depth -= 1;
                if heavy.is_match(l) {
                    out.push(f("major", &rel, i + 1, "searching or creating instances every frame", "cache the result outside the loop, or do it on an event"));
                }
                if l.trim_start().starts_with("end)") {
                    frame_depth = 0;
                }
            }
        }
        if rel.contains("client") && connects >= 6 && disconnects == 0 {
            out.push(f("major", &rel, 1, &format!("{connects} event connections and none are ever disconnected"), "disconnect on respawn or cleanup, or they pile up and leak memory"));
        }
        if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
            let stem = stem.split('.').next().unwrap_or(stem).to_string();
            if !["init", "main"].contains(&stem.as_str()) && !rel.contains(".server.") && !rel.contains(".client.") {
                names.push((stem, rel.clone()));
            }
        }
        all_text.push_str(&t);
        all_text.push('\n');
    }
    for (name, rel) in names {
        if all_text.matches(&name).count() <= 1 && !rel.contains("test") {
            out.push(f("minor", &rel, 1, "module is never required anywhere", "delete it if it is unused"));
        }
    }
    if prints > 25 {
        out.push(f("minor", "src", 1, &format!("{prints} print() calls"), "remove debug prints; they slow the server log"));
    }
    (out, json!({ "files": files.len(), "lines": lines_total }))
}

/// Replace deprecated wait/spawn/delay with task.* in one text.
pub fn safe_fix(t: &str) -> String {
    let re = Regex::new(r"(^|[^.:\w])(wait|spawn|delay)(\s*\()").unwrap();
    t.lines()
        .map(|l| {
            let (code, comment) = match l.find("--") {
                Some(i) => l.split_at(i),
                None => (l, ""),
            };
            // A local function named wait/spawn/delay is the game's own: leave it.
            if code.contains("function wait") || code.contains("function spawn") || code.contains("function delay") || code.contains('"') || code.contains('\'') {
                return l.to_string();
            }
            format!("{}{}", re.replace_all(code, "${1}task.${2}${3}"), comment)
        })
        .collect::<Vec<_>>()
        .join("\n")
        + if t.ends_with('\n') { "\n" } else { "" }
}

fn count(xml: &str, class: &str) -> usize {
    xml.matches(&format!("<Item class=\"{class}\"")).count()
}

/// Review the built place for content that makes phones lag.
pub fn scan_map(root: &Path, proj: &Path) -> (Vec<Value>, Value) {
    let pj = proj.join("default.project.json");
    let out_file = std::env::temp_dir().join(format!("tuneup-{}.rbxlx", std::process::id()));
    let ok = crate::paths::output_timeout(crate::paths::command(crate::gates::tool("rojo")).args(["build", &pj.to_string_lossy(), "-o", &out_file.to_string_lossy()]).current_dir(root), 180).map(|o| o.status.success()).unwrap_or(false);
    let xml = fs::read_to_string(&out_file).unwrap_or_default();
    let _ = fs::remove_file(&out_file);
    if !ok || xml.is_empty() {
        return (vec![f("major", "default.project.json", 1, "the map could not be built for review", "open Build and fix the rojo build first")], json!({}));
    }
    let parts: usize = ["Part", "MeshPart", "WedgePart", "CornerWedgePart", "TrussPart", "UnionOperation"].iter().map(|c| count(&xml, c)).sum();
    let anchored = xml.matches("<bool name=\"Anchored\">true</bool>").count();
    let lights: usize = ["PointLight", "SpotLight", "SurfaceLight"].iter().map(|c| count(&xml, c)).sum();
    let (emitters, unions, meshes, scripts) = (count(&xml, "ParticleEmitter"), count(&xml, "UnionOperation"), count(&xml, "MeshPart"), count(&xml, "Script") + count(&xml, "LocalScript") + count(&xml, "ModuleScript"));
    let instances = xml.matches("<Item class=").count();
    let mut out = vec![];
    let m = |sev: &str, what: String, fix: &str| f(sev, "map", 0, &what, fix);
    if parts > 20_000 {
        out.push(m("major", format!("{parts} parts in the place"), "merge repeated props into meshes, or turn on StreamingEnabled; aim under 20,000 on phones"));
    }
    if parts > 50 && parts.saturating_sub(anchored) > parts / 4 {
        out.push(m("major", format!("about {} of {parts} parts are not anchored", parts - anchored), "anchor everything that does not need to move; loose parts cost physics every frame"));
    }
    if lights > 60 {
        out.push(m("major", format!("{lights} lights"), "keep dynamic lights under about 60, and turn Shadows off on small ones"));
    }
    if emitters > 80 {
        out.push(m("minor", format!("{emitters} particle emitters"), "disable emitters far from players and lower their Rate"));
    }
    if unions > 200 {
        out.push(m("minor", format!("{unions} unions"), "export big unions as meshes; unions are heavier to load and collide"));
    }
    let streaming = xml.contains("<bool name=\"StreamingEnabled\">true</bool>");
    if parts > 5_000 && !streaming {
        out.push(m("minor", "StreamingEnabled is off on a large map".into(), "turn it on in Workspace so phones load only what is near"));
    }
    (out, json!({ "instances": instances, "parts": parts, "anchored": anchored, "lights": lights, "emitters": emitters, "unions": unions, "meshes": meshes, "scripts": scripts, "streaming": streaming }))
}

fn dir_size(p: &Path) -> u64 {
    if p.is_file() {
        return fs::metadata(p).map(|m| m.len()).unwrap_or(0);
    }
    fs::read_dir(p).into_iter().flatten().flatten().map(|e| dir_size(&e.path())).sum()
}

fn age_days(p: &Path) -> f64 {
    fs::metadata(p).and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok()).map(|d| d.as_secs_f64() / 86400.0).unwrap_or(0.0)
}

/// Temp files that are safe to delete: raw run logs over 30 days old, old
/// update installers, downloaded app source, and review packs over 30 days.
/// Reports, plans, results and projects are never touched.
pub fn stale(root: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    for e in fs::read_dir(root.join("runs")).into_iter().flatten().flatten() {
        for name in ["stream.jsonl", "stderr.log"] {
            let p = e.path().join(name);
            if p.is_file() && age_days(&p) > 30.0 {
                out.push(p);
            }
        }
    }
    let tools = crate::paths::tools_dir();
    for e in fs::read_dir(tools.join("updates")).into_iter().flatten().flatten() {
        if age_days(&e.path()) > 2.0 {
            out.push(e.path());
        }
    }
    let cur = crate::update::release_tag().unwrap_or("").to_string();
    for e in fs::read_dir(tools.join("app-source")).into_iter().flatten().flatten() {
        if e.file_name().to_string_lossy() != cur {
            out.push(e.path());
        }
    }
    for e in fs::read_dir(root.join("reviews")).into_iter().flatten().flatten() {
        let p = e.path().join("pack.md");
        if p.is_file() && age_days(&p) > 30.0 {
            out.push(p);
        }
    }
    out
}

pub fn clean(root: &Path) -> (usize, u64) {
    let (mut n, mut bytes) = (0, 0);
    for p in stale(root) {
        let sz = dir_size(&p);
        let ok = if p.is_dir() { fs::remove_dir_all(&p).is_ok() } else { fs::remove_file(&p).is_ok() };
        if ok {
            n += 1;
            bytes += sz;
        }
    }
    (n, bytes)
}

fn report_path(root: &Path) -> PathBuf {
    root.join("tuneup").join("latest.json")
}

pub fn latest(root: &Path) -> Value {
    fs::read_to_string(report_path(root)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(Value::Null)
}

/// Run the whole tune-up: scan every project, clean temp files, save a report.
pub fn run(root: &Path) -> Result<Value, String> {
    let mut projects = BTreeMap::new();
    let mut total = 0;
    for e in fs::read_dir(root.join("projects")).into_iter().flatten().flatten() {
        let p = e.path();
        if !p.join("default.project.json").is_file() {
            continue;
        }
        let (mut findings, code) = scan_code(&p);
        let (mapf, map) = scan_map(root, &p);
        findings.extend(mapf);
        let rank = |v: &Value| match v["severity"].as_str() { Some("blocker") => 0, Some("major") => 1, _ => 2 };
        findings.sort_by_key(rank);
        total += findings.len();
        projects.insert(e.file_name().to_string_lossy().to_string(), json!({ "findings": findings, "code": code, "map": map }));
    }
    let (cleaned, bytes) = clean(root);
    let rep = json!({ "at": crate::now_rfc3339(), "projects": projects, "issues": total, "cleaned_files": cleaned, "cleaned_mb": (bytes as f64 / 1_048_576.0 * 10.0).round() / 10.0 });
    fs::create_dir_all(root.join("tuneup")).map_err(|e| e.to_string())?;
    fs::write(report_path(root), serde_json::to_string_pretty(&rep).unwrap()).map_err(|e| e.to_string())?;
    Ok(rep)
}

/// Run only when the last tune-up is older than `days` (0 turns it off).
pub fn run_if_due(root: &Path, days: u32) -> Option<Value> {
    if days == 0 {
        return None;
    }
    let p = report_path(root);
    if p.is_file() && age_days(&p) < days as f64 {
        return None;
    }
    run(root).ok()
}

/// Apply the safe fixes to one project: task.* replacements and formatting.
/// Files are backed up first and restored if the build gets worse.
pub fn fix(root: &Path, project: &str) -> Result<String, String> {
    if !crate::paths::valid_id(project) {
        return Err("bad project name".into());
    }
    let proj = root.join("projects").join(project);
    let rel = format!("projects/{project}");
    let passed = |r: &Path| crate::gates::build_gate(r, &rel).iter().filter(|c| c.passed).count();
    let before = passed(root);
    let mut files = vec![];
    luau_files(&proj.join("src"), &mut files);
    let mut backup = vec![];
    let mut changed = 0;
    for p in &files {
        let Ok(t) = fs::read_to_string(p) else { continue };
        let new = safe_fix(&t);
        backup.push((p.clone(), t.clone()));
        if new != t {
            fs::write(p, new).map_err(|e| e.to_string())?;
            changed += 1;
        }
    }
    let _ = crate::paths::output_timeout(crate::paths::command(crate::gates::tool("stylua")).arg(proj.join("src")).current_dir(&proj), 120);
    if passed(root) < before {
        for (p, t) in backup {
            let _ = fs::write(p, t);
        }
        return Err("The fixes made the build checks worse, so every file was put back as it was.".into());
    }
    Ok(format!("Updated {changed} file(s) to task.wait, task.spawn and task.delay, and formatted the code. Build checks still pass as before."))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_patterns_and_fixes_safely() {
        let d = std::env::temp_dir().join(format!("tune-t-{}", std::process::id()));
        fs::create_dir_all(d.join("src/client")).unwrap();
        fs::write(d.join("src/client/a.luau"), "wait(1)\ntask.wait(1)\nx:wait()\n-- wait(2)\nwhile wait() do end\nRunService.Heartbeat:Connect(function()\n  local p = workspace:FindFirstChild(\"A\")\nend)\nlocal i = Instance.new(\"Part\", workspace)\n").unwrap();
        fs::write(d.join("src/client/b.luau"), "").unwrap();
        let (fs_, sum) = scan_code(&d);
        let whats: Vec<&str> = fs_.iter().map(|v| v["what"].as_str().unwrap()).collect();
        assert_eq!(whats.iter().filter(|w| w.starts_with("wait() is deprecated")).count(), 2, "{whats:?}");
        assert!(whats.contains(&"busy loop that runs every frame forever"));
        assert!(whats.contains(&"searching or creating instances every frame"));
        assert!(whats.contains(&"Instance.new with a parent argument is slow"));
        assert!(whats.contains(&"empty file"));
        assert_eq!(sum["files"], 2);
        assert_eq!(safe_fix("wait(1)\ntask.wait(1)\nx:wait() -- wait(3)\n a = spawn(f)\n"), "task.wait(1)\ntask.wait(1)\nx:wait() -- wait(3)\n a = task.spawn(f)\n");
        let _ = fs::remove_dir_all(&d);
    }
}

#[cfg(test)]
mod safe_fix_tests {
    #[test]
    fn leaves_own_functions_and_strings_alone() {
        let t = "local function delay(x)\nprint(\"wait(1)\")\nwait(2)\n";
        assert_eq!(super::safe_fix(t), "local function delay(x)\nprint(\"wait(1)\")\ntask.wait(2)\n");
    }
}
