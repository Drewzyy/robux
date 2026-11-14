//! The vault as a graph, for agents: one short line per note and its links,
//! so an agent can find the 1 to 3 notes it needs without reading prose.
//! `brewix map [section]` lists notes; `brewix node <note>` shows one note's
//! summary, first Do this steps, and its links in and out.
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub id: String,
    pub desc: String,
    pub dothis: Vec<String>,
    pub out: Vec<String>,
}

fn walk(d: &Path, base: &Path, out: &mut Vec<(String, String)>) {
    let Ok(rd) = fs::read_dir(d) else { return };
    let mut es: Vec<_> = rd.flatten().collect();
    es.sort_by_key(|e| e.path());
    for e in es {
        let p = e.path();
        let n = e.file_name().to_string_lossy().to_string();
        if n.starts_with('.') {
            continue;
        }
        if p.is_dir() {
            walk(&p, base, out);
        } else if let Some(stem) = n.strip_suffix(".md") {
            let rel = p.parent().and_then(|x| x.strip_prefix(base).ok()).map(|x| x.to_string_lossy().replace('\\', "/")).unwrap_or_default();
            let id = if rel.is_empty() { stem.to_string() } else { format!("{rel}/{stem}") };
            if let Ok(t) = fs::read_to_string(&p) {
                out.push((id, t));
            }
        }
    }
}

fn parse(id: &str, t: &str) -> Node {
    let mut n = Node { id: id.into(), ..Default::default() };
    let mut body = t;
    if let Some(rest) = t.strip_prefix("---") {
        if let Some(end) = rest.find("\n---") {
            for l in rest[..end].lines() {
                if let Some(d) = l.strip_prefix("description:") {
                    n.desc = d.trim().trim_matches('"').to_string();
                }
            }
            body = &rest[end + 4..];
        }
    }
    let mut in_do = false;
    for l in body.lines() {
        if l.starts_with("## ") {
            in_do = l.trim().to_ascii_lowercase().starts_with("## do this");
            continue;
        }
        if in_do && n.dothis.len() < 5 {
            if let Some(s) = l.trim().strip_prefix("- ").or_else(|| l.trim().strip_prefix("1. ")) {
                n.dothis.push(s.chars().take(220).collect());
            }
        }
    }
    let mut rest = body;
    while let Some(i) = rest.find("[[") {
        rest = &rest[i + 2..];
        let Some(j) = rest.find("]]") else { break };
        let target = rest[..j].split(['|', '#']).next().unwrap_or("").trim().trim_end_matches(".md").to_string();
        if !target.is_empty() && target != id && !n.out.contains(&target) {
            n.out.push(target);
        }
        rest = &rest[j + 2..];
    }
    n
}

pub fn build(root: &Path) -> BTreeMap<String, Node> {
    let base = root.join("vault");
    let mut files = vec![];
    walk(&base, &base, &mut files);
    files.iter().map(|(id, t)| (id.clone(), parse(id, t))).collect()
}

fn allowed(id: &str, sections: &[String]) -> bool {
    sections.is_empty() || sections.iter().any(|s| id.starts_with(&format!("{s}/")))
}

/// One line per note: `id | summary | -> links`. Index notes are skipped.
pub fn map(g: &BTreeMap<String, Node>, section: Option<&str>, sections: &[String]) -> String {
    let mut s = String::new();
    for n in g.values() {
        if n.id.ends_with("/index") || !allowed(&n.id, sections) || section.map(|x| !n.id.starts_with(&format!("{}/", x.trim_end_matches('/')))).unwrap_or(false) {
            continue;
        }
        let links: Vec<&str> = n.out.iter().filter(|o| !o.ends_with("/index")).map(|o| o.as_str()).take(6).collect();
        s.push_str(&format!("{} | {}{}\n", n.id, n.desc, if links.is_empty() { String::new() } else { format!(" | -> {}", links.join(", ")) }));
    }
    s
}

/// Resolve a note by full id or by its file name alone.
pub fn find<'a>(g: &'a BTreeMap<String, Node>, q: &str) -> Option<&'a Node> {
    let q = q.trim().trim_end_matches(".md");
    g.get(q).or_else(|| g.values().find(|n| n.id.rsplit('/').next() == Some(q)))
}

pub fn node(g: &BTreeMap<String, Node>, n: &Node) -> String {
    let incoming: Vec<&str> = g.values().filter(|m| !m.id.ends_with("/index") && m.out.iter().any(|o| o == &n.id || n.id.ends_with(&format!("/{o}")))).map(|m| m.id.as_str()).collect();
    let mut s = format!("{}\n  {}\n", n.id, n.desc);
    for d in &n.dothis {
        s.push_str(&format!("  do: {d}\n"));
    }
    if !n.out.is_empty() {
        s.push_str(&format!("  links to: {}\n", n.out.join(", ")));
    }
    if !incoming.is_empty() {
        s.push_str(&format!("  linked from: {}\n", incoming.join(", ")));
    }
    s.push_str(&format!("  full note: {}.md (Read it only to follow the steps in detail)\n", n.id));
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_desc_dothis_and_links() {
        let n = parse("a/x", "---\ndescription: Short.\n---\n# X\n## Do this\n- step one [[b/y]]\n- two\n## More\n- not this [[c/z|Z]] [[b/y]]\n");
        assert_eq!(n.desc, "Short.");
        assert_eq!(n.dothis, vec!["step one [[b/y]]", "two"]);
        assert_eq!(n.out, vec!["b/y", "c/z"]);
        let mut g = BTreeMap::new();
        g.insert("a/x".to_string(), n);
        g.insert("b/y".to_string(), parse("b/y", "---\ndescription: Why.\n---\n"));
        assert!(map(&g, Some("a"), &[]).starts_with("a/x | Short. | -> b/y, c/z"));
        assert!(node(&g, find(&g, "y").unwrap()).contains("linked from: a/x"));
    }
}
