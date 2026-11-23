//! Watch a video the way people do: hear it (captions as a timestamped
//! transcript) and see it (frames spread across the video). Based on the
//! method of claude-watch / claude-video (MIT, Bradley Bonanno): captions
//! first, then frames, then Claude reads both.
//!
//! Free tools only, no keys: yt-dlp (downloads captions and a small copy of
//! the video) and ffmpeg (takes the frames). BrewixGames downloads both into
//! its tools folder on Windows; elsewhere it uses the ones on PATH.
//! Everything lands in runs/videos/<id>/ with a watch.md agents read first.

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub fn dir(root: &Path) -> PathBuf {
    root.join("runs").join("videos")
}

fn tool_dir() -> PathBuf {
    crate::paths::tools_dir().join("video")
}

fn exe(name: &str) -> String {
    if cfg!(windows) { format!("{name}.exe") } else { name.to_string() }
}

/// yt-dlp: ours first, then one on PATH.
pub fn ytdlp() -> Option<PathBuf> {
    let ours = tool_dir().join(exe("yt-dlp"));
    if ours.is_file() {
        return Some(ours);
    }
    on_path("yt-dlp")
}

pub fn ffmpeg() -> Option<PathBuf> {
    let ours = tool_dir().join(exe("ffmpeg"));
    if ours.is_file() {
        return Some(ours);
    }
    on_path("ffmpeg")
}

fn on_path(name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|p| std::env::split_paths(&p).map(|d| d.join(exe(name))).find(|f| f.is_file()))
}

/// Download yt-dlp and ffmpeg if they are missing (Windows: into the tools
/// folder; other systems: say how to install them).
pub fn ensure_tools(on_line: &mut dyn FnMut(&str)) -> Result<(), String> {
    if ytdlp().is_some() && ffmpeg().is_some() {
        return Ok(());
    }
    if !cfg!(windows) {
        return Err("Video watching needs yt-dlp and ffmpeg. On a Mac: brew install yt-dlp ffmpeg".into());
    }
    let d = tool_dir();
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    if ytdlp().is_none() {
        on_line("Getting yt-dlp (free video downloader, about 18 MB)");
        crate::apps::download("https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe", &d.join("yt-dlp.exe"), on_line)?;
    }
    if ffmpeg().is_none() {
        on_line("Getting ffmpeg (free video tool, about 90 MB)");
        let zip = d.join("ffmpeg.zip");
        crate::apps::download("https://github.com/yt-dlp/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip", &zip, on_line)?;
        let tmp = d.join("ffmpeg-unpack");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
        let ok = crate::paths::command("tar").arg("-xf").arg(&zip).arg("-C").arg(&tmp).status().map(|s| s.success()).unwrap_or(false);
        let found = find_file(&tmp, "ffmpeg.exe");
        match (ok, found) {
            (true, Some(f)) => {
                fs::copy(&f, d.join("ffmpeg.exe")).map_err(|e| e.to_string())?;
            }
            _ => return Err("could not unpack ffmpeg".into()),
        }
        let _ = fs::remove_dir_all(&tmp);
        let _ = fs::remove_file(&zip);
    }
    Ok(())
}

fn find_file(dir: &Path, name: &str) -> Option<PathBuf> {
    for e in fs::read_dir(dir).ok()?.flatten() {
        let p = e.path();
        if p.is_dir() {
            if let Some(f) = find_file(&p, name) {
                return Some(f);
            }
        } else if p.file_name().map(|n| n == name).unwrap_or(false) {
            return Some(p);
        }
    }
    None
}

/// Links in a message that look like videos.
pub fn links_in(text: &str) -> Vec<String> {
    let re = regex::Regex::new(r#"https?://[^\s<>"')]+"#).unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().trim_end_matches(['.', ',', ';']).to_string())
        .filter(|u| {
            let l = u.to_lowercase();
            ["youtube.com/watch", "youtu.be/", "youtube.com/shorts/", "tiktok.com/", "vimeo.com/", "x.com/", "twitter.com/", "instagram.com/reel", "twitch.tv/", "streamable.com/"].iter().any(|k| l.contains(k))
        })
        .collect()
}

/// A timestamped transcript from a WebVTT caption file: one line per cue,
/// with the rolling repeats of auto-captions removed.
pub fn transcript_from_vtt(vtt: &str) -> String {
    let tag = regex::Regex::new(r"<[^>]+>").unwrap();
    let mut out: Vec<(String, String)> = vec![];
    let mut time = String::new();
    let mut last_line = String::new();
    for l in vtt.lines() {
        let l = l.trim();
        if l.contains("-->") {
            let start = l.split('.').next().unwrap_or("");
            // 00:01:05 -> 01:05 (hours only when the video is that long)
            time = start.strip_prefix("00:").unwrap_or(start).to_string();
            continue;
        }
        if l.is_empty() || l.starts_with("WEBVTT") || l.starts_with("Kind:") || l.starts_with("Language:") || l.starts_with("NOTE") {
            continue;
        }
        let text = tag.replace_all(l, "").trim().to_string();
        if text.is_empty() || text == last_line || out.last().map(|(_, t)| t.ends_with(&text)).unwrap_or(false) {
            continue;
        }
        last_line = text.clone();
        match out.last_mut() {
            Some((t, s)) if *t == time => {
                s.push(' ');
                s.push_str(&text);
            }
            _ => out.push((time.clone(), text)),
        }
    }
    out.into_iter().map(|(t, s)| format!("[{t}] {s}")).collect::<Vec<_>>().join("\n")
}

/// How many frames to take, by length (the claude-video budget).
pub fn frame_budget(seconds: f64) -> usize {
    match seconds {
        s if s <= 30.0 => 30,
        s if s <= 60.0 => 40,
        s if s <= 180.0 => 60,
        s if s <= 600.0 => 80,
        _ => 100,
    }
}

/// Watch one video into runs/videos/<id>/: watch.md (title, link,
/// transcript, frame list) and frames/*.jpg. Frames are skipped (with a
/// note) when the video itself cannot be downloaded; the transcript alone
/// is still useful.
pub fn watch(root: &Path, url: &str, on_line: &mut dyn FnMut(&str)) -> Result<Value, String> {
    if !url.starts_with("http") {
        let local = Path::new(url);
        if local.is_file() {
            return watch_file(root, local, on_line);
        }
        return Err("not a link or a video file".into());
    }
    ensure_tools(on_line)?;
    let yt = ytdlp().ok_or("yt-dlp is missing")?;
    on_line("Reading the video's details");
    let info = crate::paths::output_timeout(crate::paths::command(&yt).args(["--socket-timeout", "20", "--no-playlist", "--skip-download", "--dump-json", url]), 120).map_err(|e| format!("yt-dlp: {e}"))?;
    let meta: Value = serde_json::from_slice(&info.stdout).map_err(|_| format!("Could not open that video: {}", String::from_utf8_lossy(&info.stderr).lines().last().unwrap_or("")))?;
    let id: String = meta["id"].as_str().unwrap_or("video").chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').take(40).collect();
    let d = dir(root).join(&id);
    let frames_dir = d.join("frames");
    fs::create_dir_all(&frames_dir).map_err(|e| e.to_string())?;
    let title = meta["title"].as_str().unwrap_or("").to_string();
    let duration = meta["duration"].as_f64().unwrap_or(0.0);
    // Hear it: captions (made by the creator or automatic).
    on_line("Getting the captions");
    let _ = crate::paths::output_timeout(crate::paths::command(&yt)
        .args(["--socket-timeout", "20", "--no-playlist", "--skip-download", "--write-subs", "--write-auto-subs", "--sub-langs", "en.*,en", "--sub-format", "vtt", "-o"])
        .arg(d.join("captions").display().to_string())
        .arg(url), 180);
    let vtt = fs::read_dir(&d).into_iter().flatten().flatten().map(|e| e.path()).filter(|p| p.extension().map(|x| x == "vtt").unwrap_or(false)).min_by_key(|p| p.to_string_lossy().contains("orig") as u8);
    let transcript = vtt.and_then(|p| fs::read_to_string(p).ok()).map(|t| transcript_from_vtt(&t)).unwrap_or_default();
    // See it: a small copy of the video, frames spread across it.
    let mut frames: Vec<(String, String)> = vec![];
    let mut note = String::new();
    on_line("Taking frames");
    let video = d.join("video.mp4");
    let got = crate::paths::output_timeout(crate::paths::command(&yt)
        .args(["--socket-timeout", "20", "--no-playlist", "-f", "bv*[height<=480][ext=mp4]/b[height<=480]/worst", "-o"])
        .arg(video.display().to_string())
        .arg(url), 600)
        .map(|o| o.status.success())
        .unwrap_or(false);
    match (got && video.is_file(), ffmpeg()) {
        (true, Some(ff)) if duration > 0.0 => {
            frames = take_frames(&ff, &video, &frames_dir, duration);
            let _ = fs::remove_file(&video);
        }
        _ => note = "The video itself could not be downloaded here, so this is the transcript only.".into(),
    }
    let channel = meta["channel"].as_str().or(meta["uploader"].as_str()).unwrap_or("");
    let md = format!(
        "# {title}\n\nLink: {url}\nChannel: {channel}\nLength: {}:{:02}\nViews: {}\n\nHow to watch it: read the transcript below, then open the frames with Read (each is a moment of the video, in order). {note}\n\n## Frames\n{}\n\n## Transcript\n{}\n",
        (duration as u64) / 60,
        (duration as u64) % 60,
        meta["view_count"].as_u64().map(|v| v.to_string()).unwrap_or_default(),
        if frames.is_empty() { "(none)".to_string() } else { frames.iter().map(|(t, f)| format!("- {t} {f}")).collect::<Vec<_>>().join("\n") },
        if transcript.is_empty() { "(no captions on this video)".to_string() } else { transcript.clone() }
    );
    fs::write(d.join("watch.md"), md).map_err(|e| e.to_string())?;
    Ok(json!({ "id": id, "title": title, "channel": channel, "dir": format!("runs/videos/{id}"), "watch": format!("runs/videos/{id}/watch.md"), "frames": frames.len(), "transcript_words": transcript.split_whitespace().count(), "note": note }))
}

/// Frames from a video file into frames/, spread by the length budget.
fn take_frames(ff: &Path, video: &Path, frames_dir: &Path, duration: f64) -> Vec<(String, String)> {
    let n = frame_budget(duration);
    let fps = (n as f64 / duration.max(1.0)).min(2.0);
    let _ = crate::paths::output_timeout(crate::paths::command(ff)
        .args(["-loglevel", "error", "-i"])
        .arg(video)
        .args(["-vf", &format!("fps={fps:.4},scale=768:-2"), "-q:v", "5"])
        .arg(frames_dir.join("%03d.jpg")), 300);
    let mut files: Vec<PathBuf> = fs::read_dir(frames_dir).into_iter().flatten().flatten().map(|e| e.path()).collect();
    files.sort();
    files.iter().enumerate().map(|(i, f)| {
        let t = ((i as f64) + 0.5) / fps;
        (format!("{:02}:{:02}", (t as u64) / 60, (t as u64) % 60), format!("frames/{}", f.file_name().unwrap().to_string_lossy()))
    }).collect()
}

/// A video on this computer (a screen recording, a clip): frames only.
fn watch_file(root: &Path, file: &Path, on_line: &mut dyn FnMut(&str)) -> Result<Value, String> {
    ensure_tools(on_line)?;
    let ff = ffmpeg().ok_or("ffmpeg is missing")?;
    let probe = crate::paths::output_timeout(crate::paths::command(&ff).arg("-i").arg(file), 60).map_err(|e| e.to_string())?;
    let err = String::from_utf8_lossy(&probe.stderr);
    let duration = regex::Regex::new(r"Duration: (\d+):(\d+):(\d+(?:\.\d+)?)").unwrap().captures(&err)
        .map(|c| c[1].parse::<f64>().unwrap_or(0.0) * 3600.0 + c[2].parse::<f64>().unwrap_or(0.0) * 60.0 + c[3].parse::<f64>().unwrap_or(0.0))
        .ok_or("that file is not a video ffmpeg can read")?;
    let stem: String = file.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default().chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').take(40).collect();
    let id = format!("file-{stem}");
    let d = dir(root).join(&id);
    let frames_dir = d.join("frames");
    let _ = fs::remove_dir_all(&frames_dir);
    fs::create_dir_all(&frames_dir).map_err(|e| e.to_string())?;
    on_line("Taking frames");
    let frames = take_frames(&ff, file, &frames_dir, duration);
    let md = format!("# {}\n\nFile: {}\nLength: {}:{:02}\n\nHow to watch it: open the frames with Read; each is a moment of the video, in order. (A file has no captions; listen for nothing.)\n\n## Frames\n{}\n", stem, file.display(), (duration as u64) / 60, (duration as u64) % 60, frames.iter().map(|(t, f)| format!("- {t} {f}")).collect::<Vec<_>>().join("\n"));
    fs::write(d.join("watch.md"), md).map_err(|e| e.to_string())?;
    Ok(json!({ "id": id, "title": stem, "channel": "", "dir": format!("runs/videos/{id}"), "watch": format!("runs/videos/{id}/watch.md"), "frames": frames.len(), "transcript_words": 0, "note": "" }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_auto_captions_without_the_rolling_repeats() {
        let vtt = "WEBVTT\nKind: captions\nLanguage: en\n\n00:00:01.000 --> 00:00:03.000\nso first open the\n\n00:00:03.000 --> 00:00:05.000\nso first open the\nterrain <c>editor</c>\n\n00:00:05.000 --> 00:00:07.000\nterrain editor\nand click generate\n";
        let t = transcript_from_vtt(vtt);
        assert_eq!(t, "[00:01] so first open the\n[00:03] terrain editor\n[00:05] and click generate");
    }
    #[test]
    fn finds_video_links_and_budgets_frames() {
        let l = links_in("look at https://youtu.be/abc123 and https://example.com/x and https://www.youtube.com/watch?v=Q9, ok");
        assert_eq!(l, vec!["https://youtu.be/abc123", "https://www.youtube.com/watch?v=Q9"]);
        assert_eq!(frame_budget(20.0), 30);
        assert_eq!(frame_budget(900.0), 100);
    }

    #[test]
    fn takes_frames_from_a_video_file() {
        let Some(ff) = ffmpeg() else { return };
        let d = std::env::temp_dir().join(format!("vid-t-{}", std::process::id()));
        fs::create_dir_all(&d).unwrap();
        let clip = d.join("clip.mp4");
        let ok = crate::paths::command(&ff).args(["-loglevel", "error", "-f", "lavfi", "-i", "testsrc=duration=20:size=320x240:rate=10", "-y"]).arg(&clip).status().map(|s| s.success()).unwrap_or(false);
        assert!(ok);
        let v = watch(&d, clip.to_str().unwrap(), &mut |_| {}).unwrap();
        assert_eq!(v["frames"], 30, "20 s video gets the 30-frame budget");
        let md = fs::read_to_string(d.join(v["watch"].as_str().unwrap())).unwrap();
        assert!(md.contains("- 00:00 frames/001.jpg") && md.contains("frames/030.jpg"), "{md}");
        let _ = fs::remove_dir_all(&d);
    }
}
