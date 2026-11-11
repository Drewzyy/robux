//! The studio's own checks, built into the app so nothing needs Python on
//! the Mac. Ported 1:1 from scripts/gates/check_stage.py, build_gate.py and
//! vault/.claude/hooks/log-load.py (those scripts stay as the reference and
//! are tested against these functions).
//!
//! Agents call these through the app binary, put on their PATH as `brewix`:
//!   brewix gate build projects/<game>
//!   brewix gate stage <agent> <output-file> [--project projects/<game>]
//!   brewix hook-log            (PostToolUse hook; reads the event on stdin)

use crate::queue::GateCheck;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const OFFLINE_STD: &str = include_str!("../../../scripts/gates/roblox-offline.yml");

fn c(name: &str, ok: bool, detail: &str) -> GateCheck {
    GateCheck { name: name.into(), passed: ok, detail: detail.into() }
}

fn re(p: &str) -> Regex {
    Regex::new(p).expect("valid regex")
}

fn has(text: &str, pattern: &str) -> bool {
    re(&format!("(?im){pattern}")).is_match(text)
}

/// Cells of one column in the first markdown table whose header names it.
fn table_col(text: &str, col: &str) -> Vec<String> {
    let rows: Vec<Vec<String>> = text
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with('|'))
        .map(|l| l.trim_matches('|').split('|').map(|c| c.trim().to_string()).collect())
        .collect();
    let col = col.to_lowercase();
    for (i, r) in rows.iter().enumerate() {
        if let Some(k) = r.iter().position(|c| c.to_lowercase().contains(&col)) {
            return rows[i + 1..]
                .iter()
                .take_while(|x| x.len() > k)
                .filter(|x| !x[0].starts_with("--") && !x[0].starts_with(":-"))
                .map(|x| x[k].clone())
                .collect();
        }
    }
    vec![]
}

fn heading(text: &str, words: &str) -> bool {
    has(text, &format!(r"^#{{1,4}}\s*(\d+[.)]\s*)?.*({words})"))
}

/// Where a tool lives: the app's bundled tools first, then PATH, then
/// ~/.cargo/bin (developer machines).
pub fn tool(name: &str) -> PathBuf {
    let mut dirs: Vec<PathBuf> = vec![];
    if let Ok(d) = std::env::var("BREWIX_TOOLS_DIR") {
        dirs.push(PathBuf::from(d));
    }
    if let Ok(exe) = std::env::current_exe() {
        let exe = exe.canonicalize().unwrap_or(exe);
        if let Some(macos) = exe.parent() {
            dirs.push(macos.join("../Resources/tools"));
            dirs.push(macos.join("tools"));
        }
    }
    if let Ok(path) = std::env::var("PATH") {
        dirs.extend(std::env::split_paths(&path));
    }
    if let Some(home) = crate::paths::home() {
        dirs.push(home.join(".cargo/bin"));
    }
    dirs.into_iter()
        .flat_map(|d| [d.join(name), d.join(format!("{name}.exe"))])
        .find(|p| p.is_file())
        .unwrap_or_else(|| PathBuf::from(name))
}

fn run(name: &str, args: &[&str], cwd: &Path) -> (Option<i32>, String) {
    match Command::new(tool(name)).args(args).current_dir(cwd).stdin(Stdio::null()).output() {
        Ok(o) => (o.status.code(), format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr))),
        Err(_) => (None, format!("NOT RUN: {name} not installed")),
    }
}

fn tail(s: &str, n: usize) -> String {
    let s = s.trim();
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= n {
        s.to_string()
    } else {
        format!("...{}", chars[chars.len() - n..].iter().collect::<String>())
    }
}

fn tempdir(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("brewix-{tag}-{}-{}", std::process::id(), chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)));
    let _ = fs::create_dir_all(&d);
    d
}

/// Rojo build, Selene, StyLua, Lune tests. A missing tool is a failed check.
pub fn build_gate(root: &Path, project: &str) -> Vec<GateCheck> {
    let proj = if Path::new(project).is_absolute() { PathBuf::from(project) } else { root.join(project) };
    let pj = proj.join("default.project.json");
    if !pj.is_file() {
        return vec![c("rojo project exists", false, &format!("no default.project.json in {project}"))];
    }
    let mut checks = vec![];
    let td = tempdir("rojo");
    let out_file = td.join("build.rbxl");
    let (code, out) = run("rojo", &["build", &pj.to_string_lossy(), "-o", &out_file.to_string_lossy()], &proj);
    checks.push(c("rojo build", code == Some(0), &if code == Some(0) { "built place file".into() } else { tail(&out, 600) }));
    let _ = fs::remove_dir_all(&td);

    let src = proj.join("src");
    let td = tempdir("selene");
    let cfg = fs::read_to_string(proj.join("selene.toml")).unwrap_or_else(|_| "std = \"roblox\"\n".into());
    let _ = fs::write(td.join("selene.toml"), &cfg);
    let (mut code, mut out) = run("selene", &["--no-summary", &src.to_string_lossy()], &td);
    let mut note = "";
    if code.is_some() && out.contains("Could not collect standard library") {
        let _ = fs::write(td.join("roblox-offline.yml"), OFFLINE_STD);
        let rest: Vec<&str> = cfg.lines().filter(|l| !l.trim().starts_with("std")).collect();
        let _ = fs::write(td.join("selene.toml"), format!("std = \"roblox-offline\"\n{}\n", rest.join("\n")));
        (code, out) = run("selene", &["--no-summary", &src.to_string_lossy()], &td);
        note = " (offline std: Roblox globals only, API members not checked)";
    }
    checks.push(c("selene lint", code == Some(0), &format!("{}{note}", if code == Some(0) { "clean".into() } else { tail(&out, 600) })));
    let _ = fs::remove_dir_all(&td);

    let (code, out) = run("stylua", &["--check", &src.to_string_lossy()], &proj);
    checks.push(c("stylua --check", code == Some(0), &if code == Some(0) { "formatted".into() } else { tail(&out, 600) }));

    if !proj.join("tests/run.luau").is_file() {
        checks.push(c("lune unit tests", false, "no tests/run.luau in the project"));
    } else {
        let (code, out) = run("lune", &["run", "tests/run.luau"], &proj);
        let t = tail(&out, 400);
        checks.push(c("lune unit tests", code == Some(0), if t.is_empty() { "passed" } else { &t }));
    }
    checks
}

fn common(text: &str) -> Vec<GateCheck> {
    let lower = text.to_lowercase();
    let filler: Vec<&str> = ["lorem ipsum", "as an ai", "in today's fast-paced", "game-changer", "unleash", "delve into"]
        .into_iter()
        .filter(|p| lower.contains(p))
        .collect();
    vec![
        c("has a Loaded section listing vault notes read", has(text, r"^#{1,4}\s*(\d+[.)]\s*)?Loaded|^\*\*Loaded|^Loaded:"), ""),
        c("no filler phrases", filler.is_empty(), &if filler.is_empty() { "none found".into() } else { filler.join(", ") }),
    ]
}

fn project_from(text: &str) -> Option<String> {
    re(r"(?m)^\s*Project:\s*`?(projects/[A-Za-z0-9_-]+|none)`?").captures(text).map(|m| m[1].to_string())
}

fn svg_words(svg: &str) -> usize {
    let tags = re(r"<[^>]+>");
    re(r"(?s)<text\b[^>]*>(.*?)</text>")
        .captures_iter(svg)
        .map(|m| tags.replace_all(&m[1], " ").split_whitespace().count())
        .sum()
}

/// Text lines that run past the 1920x1080 frame. Width is estimated from
/// font size and character count (heavy display fonts run about 0.58 em
/// per character), so it is a rule of thumb, not a renderer.
fn svg_text_overflow(svg: &str) -> Vec<String> {
    let tags = re(r"<[^>]+>");
    let attr = |tag: &str, k: &str| -> Option<String> {
        re(&format!(r#"\b{k}\s*=\s*"([^"]*)""#)).captures(tag).map(|m| m[1].to_string())
    };
    let mut bad = vec![];
    for m in re(r"(?s)<text\b([^>]*)>(.*?)</text>").captures_iter(svg) {
        let tag = &m[1];
        let text = tags.replace_all(&m[2], "").trim().to_string();
        if text.is_empty() {
            continue;
        }
        let num = |k: &str, d: f64| attr(tag, k).and_then(|v| v.trim_end_matches("px").parse::<f64>().ok()).unwrap_or(d);
        let (x, y, size) = (num("x", 0.0), num("y", 0.0), num("font-size", 16.0));
        let w = text.chars().count() as f64 * size * 0.58;
        let (left, right) = match attr(tag, "text-anchor").as_deref() {
            Some("middle") => (x - w / 2.0, x + w / 2.0),
            Some("end") => (x - w, x),
            _ => (x, x + w),
        };
        if left < 0.0 || right > 1920.0 || y - size * 0.8 < 0.0 || y > 1080.0 {
            bad.push(text);
        }
    }
    bad
}

fn svg_colors(svg: &str) -> usize {
    let mut cols = std::collections::BTreeSet::new();
    for m in re(r#"(?:fill|stroke|stop-color)\s*[:=]\s*"?(#[0-9a-fA-F]{3,6})"#).captures_iter(svg) {
        let mut v = m[1].to_lowercase();
        if v.len() == 4 {
            v = format!("#{}", v[1..].chars().flat_map(|ch| [ch, ch]).collect::<String>());
        }
        if v.len() != 7 {
            continue;
        }
        let ch = |i: usize| u8::from_str_radix(&v[i..i + 2], 16).unwrap_or(0) as i32;
        let (r, g, b) = (ch(1), ch(3), ch(5));
        if r.max(g).max(b) - r.min(g).min(b) < 24 {
            continue; // black, white and greys are not palette colours
        }
        cols.insert(v);
    }
    cols.len()
}

fn splash_checks(text: &str, out_path: &Path) -> Vec<GateCheck> {
    let tdir = out_path.parent().unwrap_or(Path::new(".")).join("thumbnails");
    let mut thumbs: Vec<PathBuf> = fs::read_dir(&tdir)
        .map(|rd| rd.flatten().map(|e| e.path()).filter(|p| {
            let n = p.file_name().unwrap_or_default().to_string_lossy();
            n.starts_with('t') && n.ends_with(".svg")
        }).collect())
        .unwrap_or_default();
    thumbs.sort();
    let mut r = vec![c("10 to 20 thumbnail comps", (10..=20).contains(&thumbs.len()), &format!("{} found in thumbnails/", thumbs.len()))];
    let (mut bad_size, mut bad_words, mut bad_cols, mut real) = (vec![], vec![], vec![], 0usize);
    let href = re(r#"href="([^"]+\.(?:png|jpe?g))""#);
    for t in &thumbs {
        let svg = fs::read_to_string(t).unwrap_or_default();
        let name = t.file_name().unwrap_or_default().to_string_lossy().to_string();
        if !svg.contains(r#"width="1920""#) || !svg.contains(r#"height="1080""#) {
            bad_size.push(name.clone());
        }
        let w = svg_words(&svg);
        if w > 3 {
            bad_words.push(format!("{name} ({w} words)"));
        }
        let k = svg_colors(&svg);
        if k > 3 {
            bad_cols.push(format!("{name} ({k} colours)"));
        }
        if href.captures_iter(&svg).any(|m| {
            let h = &m[1];
            !h.starts_with("http") && !h.starts_with("data:") && t.parent().unwrap().join(h).is_file()
        }) {
            real += 1;
        }
    }
    let any = !thumbs.is_empty();
    let join = |v: &Vec<String>, ok: &str| if v.is_empty() { ok.to_string() } else { v.join(", ") };
    r.push(c("thumbnails are 1920x1080", any && bad_size.is_empty(), &join(&bad_size, "all")));
    r.push(c("0 to 3 words of text each", any && bad_words.is_empty(), &join(&bad_words, "all within limit")));
    r.push(c("max 3 saturated colours each", any && bad_cols.is_empty(), &join(&bad_cols, "all within limit")));
    let isvg = fs::read_to_string(tdir.join("icon.svg")).unwrap_or_default();
    r.push(c("512x512 icon", isvg.contains(r#"width="512""#) && isvg.contains(r#"height="512""#), if isvg.is_empty() { "missing" } else { "thumbnails/icon.svg" }));
    r.push(c("uses the game's own visuals (real captures embedded)", any && real == thumbs.len(), &format!("{real}/{} embed a capture that exists", thumbs.len())));
    r.push(c("ranking names a launch pick", heading(text, "rank") && has(text, "launch"), ""));
    r
}

fn sounds_checks(root: &Path, text: &str, project: Option<&str>) -> Vec<GateCheck> {
    let proj = project.map(|s| s.to_string()).or_else(|| re(r"projects/[A-Za-z0-9_-]+").find(text).map(|m| m.as_str().to_string()));
    let Some(proj) = proj else { return vec![c("sounds.json found", false, "no project named in the output")] };
    let f = root.join(&proj).join("sounds.json");
    let Ok(t) = fs::read_to_string(&f) else { return vec![c("sounds.json found", false, &format!("{proj}/sounds.json missing"))] };
    let data: serde_json::Value = match serde_json::from_str(&t) {
        Ok(v) => v,
        Err(e) => return vec![c("sounds.json parses", false, &e.to_string().chars().take(120).collect::<String>())],
    };
    let Some(sounds) = data.get("sounds").and_then(|s| s.as_array()) else {
        return vec![c("sounds.json parses with a sounds list", false, "0 sounds")];
    };
    let mut out = vec![c("sounds.json parses with a sounds list", !sounds.is_empty(), &format!("{} sounds", sounds.len()))];
    let s = |v: &serde_json::Value, k: &str| v.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let id_ok = re(r"^rbxassetid://\d+$");
    let ids: Vec<String> = sounds.iter().map(|v| s(v, "id")).collect();
    let bad_shape: Vec<String> = sounds.iter().filter(|v| s(v, "id").is_empty() || s(v, "event").is_empty() || s(v, "prompt").is_empty()).map(|v| s(v, "id")).collect();
    let bad_id: Vec<String> = sounds
        .iter()
        .filter(|v| v.get("asset_id").map(|a| !a.is_null() && !id_ok.is_match(a.as_str().unwrap_or(""))).unwrap_or(false))
        .map(|v| s(v, "id"))
        .collect();
    let mut dup: Vec<String> = ids.iter().filter(|i| ids.iter().filter(|j| j == i).count() > 1).cloned().collect();
    dup.sort();
    dup.dedup();
    let join = |v: &Vec<String>, ok: &str| if v.is_empty() { ok.to_string() } else { v.join(", ") };
    out.push(c("every sound has id, event, prompt", bad_shape.is_empty(), &join(&bad_shape, "all")));
    out.push(c("asset ids are null or rbxassetid://<number>", bad_id.is_empty(), &join(&bad_id, "all valid")));
    out.push(c("sound ids are unique", dup.is_empty(), &join(&dup, "unique")));
    out.push(c("first 60 seconds has sounds requested", sounds.iter().any(|v| v.get("first_60_seconds").and_then(|b| b.as_bool()).unwrap_or(false)), ""));
    out
}

/// Studio-side checks on one agent's output file.
pub fn check_stage(root: &Path, agent: &str, out_path: &Path, project: Option<&str>) -> Vec<GateCheck> {
    let text = fs::read_to_string(out_path).unwrap_or_default();
    if text.is_empty() {
        return vec![c("output readable", false, &format!("{} missing", out_path.file_name().unwrap_or_default().to_string_lossy()))];
    }
    let mut r = common(&text);
    match agent {
        "pulse" => {
            r.push(c("names its data sources", heading(&text, "data sources|sources"), ""));
            r.push(c("ranked pitches present", heading(&text, "rank"), ""));
            r.push(c("numbers are sourced or marked UNVERIFIED", has(&text, r"https?://") || text.contains("UNVERIFIED"), ""));
        }
        "lead" => {
            let rows = text.lines().filter(|l| l.trim().starts_with('|')).count();
            r.push(c("plan table with stages", rows >= 4, &format!("{rows} table rows")));
            r.push(c("has a Blocked section", heading(&text, "blocked"), ""));
            r.push(c("ends in the review queue", has(&text, "review queue"), ""));
        }
        "architect" => {
            r.push(c("core loop section", heading(&text, "core loop"), ""));
            r.push(c("first 60 seconds section", heading(&text, "first 60"), ""));
            let windows = [r"0\s*[-–]\s*5", r"5\s*[-–]\s*10", r"10\s*[-–]\s*30", r"30\s*[-–]\s*(45|60)"].iter().all(|w| has(&text, w));
            r.push(c("first 60 seconds covers 0-5, 5-10, 10-30, 30-60", windows, ""));
            r.push(c("says what we will not do", heading(&text, "not do|will not|won't"), ""));
        }
        "builder" => {
            let declared = project.map(|s| s.to_string()).or_else(|| project_from(&text));
            r.push(c("declares its project folder", declared.is_some(), declared.as_deref().unwrap_or("no 'Project:' line")));
            match declared.as_deref() {
                Some("none") => r.push(c("produced a buildable project", false, "Project: none (snippet only)")),
                Some(d) => {
                    let pdir = root.join(d);
                    r.push(c("project folder exists", pdir.is_dir(), d));
                    if pdir.is_dir() {
                        let n = count_luau(&pdir.join("src"));
                        r.push(c("project has Luau source", n > 0, &format!("{n} .luau files under src/")));
                        r.extend(build_gate(root, d));
                    }
                }
                None => {}
            }
        }
        "playtest" => {
            r.push(c("states how it tested", heading(&text, "how tested|method"), ""));
            r.push(c("has a verdict", heading(&text, "verdict"), ""));
            r.push(c("problems have severities", has(&text, r"\b(blocker|major|minor)\b") || has(&text, "no issue found"), ""));
            let denies = has(&text, "nothing was played live|not played live|no live play|code walk only|studio mcp was not used");
            let claims = has(&text, r"\bplayed (it )?live\b|\blive in studio\b|\bvia (the )?studio mcp\b") && !denies;
            r.push(c("does not claim live play it could not do", !claims, if claims { "claims live play in Studio" } else { "headless or code walk (says so)" }));
        }
        "merchant" => {
            r.push(c("catalog present", heading(&text, "catalog"), ""));
            r.push(c("do-not-sell check present", heading(&text, "do-not-sell|do not sell"), ""));
        }
        "librarian" => r.extend(sounds_checks(root, &text, project)),
        "frame" => r.push(c("mobile checklist results present", has(&text, "mobile|phone"), "")),
        "splash" => r.extend(splash_checks(&text, out_path)),
        "liveops" => r.push(c("numbers come with dates", has(&text, r"\d{4}-\d{2}-\d{2}") || text.to_uppercase().contains("BLOCKED"), "")),
        "producer" => r.push(c("one named reason", heading(&text, "reason"), "")),
        "simulator" => r.extend(simulation_checks(out_path)),
        "modeler" => {
            r.push(c("models table with file, triangles, license", has(&text, r"^\|.*file.*\|.*triangles.*\|.*license.*\|"), ""));
            let path_re = re(r"projects/[A-Za-z0-9_-]+/assets/models/[A-Za-z0-9_.-]+\.(?:fbx|obj|py)");
            let files: Vec<String> = table_col(&text, "file").iter().filter_map(|cell| path_re.find(cell).map(|m| m.as_str().to_string())).collect();
            let rows = table_col(&text, "file").len();
            let missing: Vec<&String> = files.iter().filter(|f| !root.join(f).is_file()).collect();
            let ok = rows > 0 && files.len() == rows && missing.is_empty();
            r.push(c("every model file in the table exists", ok, &if rows == 0 { "no Models table".to_string() } else if files.len() < rows { format!("{} of {} rows name no file path under projects/<game>/assets/models/", rows - files.len(), rows) } else if missing.is_empty() { format!("{} file(s)", files.len()) } else { format!("missing: {}", missing.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")) }));
            let lic = table_col(&text, "license");
            let bad: Vec<&String> = lic.iter().filter(|l| !has(l, r"\bcc0\b|made (in blender )?for this game") || has(l, r"sketchfab|poly ?pizza|cc[- ]by|paid|purchased")).collect();
            r.push(c("licenses are CC0 or made for this game", !lic.is_empty() && bad.is_empty(), &bad.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("; ")));
            r.push(c("import steps for Andrew", heading(&text, "import"), ""));
        }
        "motion" => {
            r.push(c("action, trigger, effect table", has(&text, r"^\|.*action.*\|.*trigger.*\|"), ""));
            r.push(c("effects are cleaned up", has(&text, r"debris|:destroy|destroy\("), ""));
        }
        "crowd" => {
            r.push(c("server authority section", heading(&text, "server authority"), ""));
            r.push(c("works with one player", has(&text, r"one player|solo|single player|1 player"), ""));
        }
        "tuner" => {
            r.push(c("budget section", heading(&text, "budget"), ""));
            r.push(c("findings cite file:line or none found", has(&text, r"\.luau:\d+") || has(&text, "no issue found"), ""));
            r.push(c("says measured or estimated", has(&text, r"measured|estimated"), ""));
        }
        "director" => {
            r.push(c("identity section", heading(&text, "identity"), ""));
            r.push(c("cut list section", heading(&text, "cut"), ""));
            r.push(c("verdict section", heading(&text, "verdict"), ""));
        }
        "release" => {
            r.push(c("checklist section", heading(&text, "checklist"), ""));
            r.push(c("blockers section", heading(&text, "blocker"), ""));
            r.push(c("says nothing was published", has(&text, r"nothing (was|has been) published|not published|andrew publishes"), ""));
            r.push(c("no paid promotion", !has(&text, r"\b(paid ads?|sponsor(ed)? ads?|ad credits?|buy ads?)\b") || has(&text, r"no (paid )?ads"), ""));
        }
        _ => {}
    }
    r
}

/// The simulator's structured result must be complete and honest about
/// what it is (an estimate from reading the game, not real players).
fn simulation_checks(out_path: &Path) -> Vec<GateCheck> {
    let f = out_path.parent().unwrap_or(Path::new(".")).join("simulation.json");
    let Ok(t) = fs::read_to_string(&f) else { return vec![c("simulation.json written", false, "missing")] };
    let v: serde_json::Value = match serde_json::from_str(&t) {
        Ok(v) => v,
        Err(e) => return vec![c("simulation.json parses", false, &e.to_string().chars().take(120).collect::<String>())],
    };
    let personas = v.get("personas").and_then(|p| p.as_array()).cloned().unwrap_or_default();
    let s = |x: &serde_json::Value, k: &str| x.get(k).and_then(|y| y.as_str()).unwrap_or("").to_string();
    let n = |x: &serde_json::Value, k: &str| x.get(k).and_then(|y| y.as_f64());
    let mut bands: Vec<String> = personas.iter().map(|p| s(p, "age_band")).filter(|b| !b.is_empty()).collect();
    bands.sort();
    bands.dedup();
    let complete = personas.iter().all(|p| {
        n(p, "session_minutes").map(|m| (0.0..=180.0).contains(&m)).unwrap_or(false)
            && n(p, "fun").map(|m| (1.0..=10.0).contains(&m)).unwrap_or(false)
            && !s(p, "quit_reason").is_empty()
            && !s(p, "evidence").is_empty()
            && ["yes", "maybe", "no"].contains(&s(p, "would_return").as_str())
    });
    let score = v.pointer("/rating/score").and_then(|x| x.as_f64());
    let imps = v.get("improvements").and_then(|p| p.as_array()).cloned().unwrap_or_default();
    let imps_ok = imps.len() >= 3 && imps.iter().all(|i| !s(i, "title").is_empty() && !s(i, "how").is_empty() && !s(i, "why").is_empty());
    let method = s(&v, "method").to_lowercase();
    vec![
        c("simulation.json parses", true, &format!("{} personas", personas.len())),
        c("at least 5 personas across 4+ age bands", personas.len() >= 5 && bands.len() >= 4, &bands.join(", ")),
        c("every persona has minutes, fun, quit reason, evidence, return answer", !personas.is_empty() && complete, ""),
        c("rating between 0 and 100", score.map(|x| (0.0..=100.0).contains(&x)).unwrap_or(false), &score.map(|x| x.to_string()).unwrap_or_default()),
        c("3 or more improvements with why and how", imps_ok, &format!("{} improvements", imps.len())),
        c("says it is an estimate, not real player data", method.contains("not real") || method.contains("estimate"), ""),
    ]
}

/// Score one thumbnail comp against the parts of the rule sheet a program
/// can check. This is a rule score, not a prediction of clicks.
pub fn thumb_score(path: &Path) -> serde_json::Value {
    let svg = fs::read_to_string(path).unwrap_or_default();
    let href = re(r#"href="([^"]+\.(?:png|jpe?g))""#);
    let words = svg_words(&svg);
    let cols = svg_colors(&svg);
    let capture = href.captures_iter(&svg).any(|m| {
        let h = &m[1];
        !h.starts_with("http") && !h.starts_with("data:") && path.parent().map(|d| d.join(h).is_file()).unwrap_or(false)
    });
    let outlined = words == 0 || svg.contains("stroke=");
    let checks = vec![
        ("1920x1080", svg.contains(r#"width="1920""#) && svg.contains(r#"height="1080""#)),
        ("0 to 3 words", words <= 3),
        ("3 colours or fewer", (1..=3).contains(&cols)),
        ("text has a heavy outline", outlined),
        ("text fits inside the frame", svg_text_overflow(&svg).is_empty()),
        ("uses a real game capture", capture),
    ];
    let pass = checks.iter().filter(|(_, ok)| *ok).count();
    let grade = match pass {
        6 => "A+",
        5 => "A",
        4 => "B",
        3 => "C",
        _ => "D",
    };
    serde_json::json!({
        "score": pass, "of": checks.len(), "grade": grade, "words": words, "colours": cols, "overflow": svg_text_overflow(&svg),
        "checks": checks.iter().map(|(n, ok)| serde_json::json!({"name": n, "passed": ok})).collect::<Vec<_>>()
    })
}

fn count_luau(dir: &Path) -> usize {
    let mut n = 0;
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                n += count_luau(&p);
            } else if p.extension().and_then(|x| x.to_str()) == Some("luau") {
                n += 1;
            }
        }
    }
    n
}

/// PostToolUse hook: log every file or search an agent touches.
pub fn hook_log(stdin: &str) {
    let Ok(data) = serde_json::from_str::<serde_json::Value>(stdin) else { return };
    let tool = data.get("tool_name").and_then(|t| t.as_str()).unwrap_or("").to_string();
    let tin = data.get("tool_input").cloned().unwrap_or(serde_json::Value::Null);
    let g = |k: &str| tin.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let target = if tool == "Glob" || tool == "Grep" {
        let p = g("path");
        if p.is_empty() { "(vault root)".to_string() } else { p }
    } else {
        [g("file_path"), g("skill"), g("path")].into_iter().find(|s| !s.is_empty()).unwrap_or_default()
    };
    let vault = std::env::var("CLAUDE_PROJECT_DIR").ok().or_else(|| data.get("cwd").and_then(|c| c.as_str()).map(|s| s.to_string())).unwrap_or_default();
    let prefix = format!("{}/", vault.trim_end_matches('/'));
    let rel = if !vault.is_empty() && target.starts_with(&prefix) { target[prefix.len()..].to_string() } else { target };
    let record = serde_json::json!({
        "ts": chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "run_id": std::env::var("ROBUX_RUN_ID").ok(),
        "agent": std::env::var("ROBUX_AGENT").ok().or_else(|| data.get("agent_type").and_then(|a| a.as_str()).map(|s| s.to_string())),
        "subagent": data.get("agent_type"),
        "tool": tool,
        "target": rel,
        "input": if tool == "Grep" || tool == "Glob" { tin } else { serde_json::Value::Null },
    });
    let out_dir = std::env::var("ROBUX_RUN_DIR").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(if vault.is_empty() { "." } else { &vault }).join(".claude/logs"));
    let _ = fs::create_dir_all(&out_dir);
    use std::io::Write;
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(out_dir.join("loads.jsonl")) {
        let _ = writeln!(f, "{record}");
    }
}

/// Command-line entry for `brewix gate ...` and `brewix hook-log`. Returns
/// Some(exit code) when argv was one of these, None otherwise.
pub fn cli(args: &[String]) -> Option<i32> {
    let a: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match a.as_slice() {
        ["hook-log"] => {
            let mut s = String::new();
            let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut s);
            hook_log(&s);
            Some(0)
        }
        ["studio-guard"] => {
            let mut s = String::new();
            let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut s);
            Some(crate::studio_mcp::guard_hook(&s))
        }
        ["mock-studio-mcp"] => Some(crate::studio_mcp::mock_server()),
        ["blender-guard"] => {
            let mut s = String::new();
            let _ = std::io::Read::read_to_string(&mut std::io::stdin(), &mut s);
            Some(crate::blender_mcp::guard_hook(&s))
        }
        ["mock-blender-mcp"] => Some(crate::blender_mcp::mock_server()),
        ["gate", ..] if crate::paths::studio_root().is_err() => {
            eprintln!("error: no studio folder found (open BrewixGames once to set it up)");
            Some(1)
        }
        ["search", rest @ ..] if !rest.is_empty() => {
            let Ok(root) = crate::paths::studio_root() else {
                eprintln!("error: no studio folder found (open BrewixGames once to set it up)");
                return Some(1);
            };
            let mut words = vec![];
            let (mut agent, mut limit) = (None, 5usize);
            let mut i = 0;
            while i < rest.len() {
                match rest[i] {
                    "--agent" => { agent = rest.get(i + 1).copied(); i += 1; }
                    "--limit" => { limit = rest.get(i + 1).and_then(|x| x.parse().ok()).unwrap_or(5).min(20); i += 1; }
                    w => words.push(w),
                }
                i += 1;
            }
            let sections: Vec<String> = agent
                .and_then(|a| {
                    let t = fs::read_to_string(root.join("studio/agents.json")).ok()?;
                    let v: serde_json::Value = serde_json::from_str(&t).ok()?;
                    let secs = v.get(a)?.get("allowed_sections")?.as_array()?.iter().filter_map(|x| x.as_str().map(String::from)).collect::<Vec<_>>();
                    (!secs.is_empty()).then_some(secs)
                })
                .unwrap_or_default();
            match crate::search::search(&root, &words.join(" "), &sections, limit) {
                Ok(h) if h.is_empty() => println!("No vault notes match. Try fewer or different words."),
                Ok(h) => {
                    for x in h {
                        println!("{}  ({})\n  {}\n", x.path, x.title, x.snippet);
                    }
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    return Some(1);
                }
            }
            Some(0)
        }
        ["gate", "build", project] => {
            let root = crate::paths::studio_root().ok()?;
            println!("{}", serde_json::to_string_pretty(&build_gate(&root, project)).unwrap());
            Some(0)
        }
        ["gate", "stage", agent, out, rest @ ..] => {
            let root = crate::paths::studio_root().ok()?;
            let project = rest.iter().position(|x| *x == "--project").and_then(|i| rest.get(i + 1)).copied();
            println!("{}", serde_json::to_string(&check_stage(&root, agent, Path::new(out), project)).unwrap());
            Some(0)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflowing_text_is_caught() {
        let off = r#"<svg width="1920" height="1080"><text x="1400" y="260" text-anchor="middle" font-size="210">9 MINUTES</text></svg>"#;
        let fits = r#"<svg width="1920" height="1080"><text x="1430" y="960" text-anchor="middle" font-size="190">FUEL IT</text></svg>"#;
        assert_eq!(svg_text_overflow(off), vec!["9 MINUTES".to_string()]);
        assert!(svg_text_overflow(fits).is_empty());
    }
}
