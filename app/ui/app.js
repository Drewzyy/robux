// BrewixGames UI. Plain JS, no build step. Every rule that matters
// (approval, caps, budget, gates) is enforced in Rust; this file shows
// state and forwards clicks.

const T = window.__TAURI__;
const invoke = (cmd, args) => T.core.invoke(cmd, args);
const $ = (id) => document.getElementById(id);

// ---------- helpers ----------
function el(tag, attrs = {}, ...kids) {
  const n = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs || {})) {
    if (v === null || v === undefined || v === false) continue;
    if (k === "class") n.className = v;
    else if (k === "html") n.innerHTML = v; // only our own static SVG strings
    else if (k.startsWith("on")) n.addEventListener(k.slice(2), v);
    else n.setAttribute(k, v === true ? "" : v);
  }
  for (const k of kids.flat(Infinity)) {
    if (k === null || k === undefined || k === false) continue;
    n.append(typeof k === "string" || typeof k === "number" ? document.createTextNode(String(k)) : k);
  }
  return n;
}
const usd = (x, d = 2) => (x === null || x === undefined ? "–" : "$" + Number(x).toFixed(d));
function ago(iso) {
  if (!iso) return "";
  const s = (Date.now() - Date.parse(iso)) / 1000;
  if (s < 60) return "just now";
  if (s < 3600) return Math.floor(s / 60) + " min ago";
  if (s < 86400) return Math.floor(s / 3600) + " h ago";
  return Math.floor(s / 86400) + " d ago";
}
const cap = (s) => (s ? s[0].toUpperCase() + s.slice(1) : s);

const ICONS = {
  tray: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M4 13l2.5-7h11L20 13"/><path d="M4 13v5a1 1 0 001 1h14a1 1 0 001-1v-5h-5l-1.5 2.5h-3L9 13H4z"/></svg>',
  people: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"><circle cx="9" cy="8" r="3.2"/><path d="M3.5 19c.8-3.2 3-5 5.5-5s4.7 1.8 5.5 5"/><circle cx="17" cy="9" r="2.4"/><path d="M15.5 14.2c2.3.2 4 1.8 4.7 4.8"/></svg>',
  book: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"><path d="M4 5.5C6.5 4.5 9.5 4.5 12 6c2.5-1.5 5.5-1.5 8-.5V19c-2.5-1-5.5-1-8 .5-2.5-1.5-5.5-1.5-8-.5V5.5z"/><path d="M12 6v13.5"/></svg>',
  list: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round"><path d="M9 6h11M9 12h11M9 18h11"/><circle cx="4.5" cy="6" r="1"/><circle cx="4.5" cy="12" r="1"/><circle cx="4.5" cy="18" r="1"/></svg>',
  gear: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7"><circle cx="12" cy="12" r="3"/><path d="M12 2.8v2.4M12 18.8v2.4M21.2 12h-2.4M5.2 12H2.8M18.5 5.5l-1.7 1.7M7.2 16.8l-1.7 1.7M18.5 18.5l-1.7-1.7M7.2 7.2L5.5 5.5"/></svg>',
  chat: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linejoin="round"><path d="M4 6.5A2.5 2.5 0 016.5 4h11A2.5 2.5 0 0120 6.5v7a2.5 2.5 0 01-2.5 2.5H11l-4.5 3.5V16A2.5 2.5 0 014 13.5v-7z"/></svg>',
  pulse: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.7" stroke-linecap="round" stroke-linejoin="round"><path d="M3 12h4l2.5-6 4 12 2.5-6H21"/></svg>',
  chev: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M9 6l6 6-6 6"/></svg>',
  cube: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round"><path d="M12 3l8 4.5v9L12 21l-8-4.5v-9L12 3z"/><path d="M4 7.5l8 4.5 8-4.5M12 12v9"/></svg>',
  doc: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round"><path d="M6 3h8l4 4v14H6V3z"/><path d="M14 3v4h4M9 12h6M9 16h6"/></svg>',
  spark: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><path d="M12 3v4M12 17v4M3 12h4M17 12h4M6 6l2.5 2.5M15.5 15.5L18 18M6 18l2.5-2.5M15.5 8.5L18 6"/></svg>',
  image: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linejoin="round"><rect x="3.5" y="5" width="17" height="14" rx="2"/><circle cx="9" cy="10" r="1.6"/><path d="M4 17l5-4 4 3 3-2 4 3"/></svg>',
  check: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round"><path d="M5 12.5l4.5 4.5L19 7.5"/></svg>',
};

const KIND_ICON = { build: "cube", design: "doc", pitch: "spark", thumbnail: "image", "playtest-report": "check", asset: "cube" };

let toastTimer;
function toast(msg, err = false) {
  const t = $("toast");
  t.textContent = msg;
  t.className = "toast" + (err ? " err" : "");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => t.classList.add("hidden"), 4200);
}
function sheet(title, ...content) {
  $("sheet-title").textContent = title;
  $("sheet-content").replaceChildren(...content.flat().filter(Boolean));
  $("sheet").classList.remove("hidden");
}
function closeSheet() {
  $("sheet").classList.add("hidden");
}
$("sheet-close").addEventListener("click", closeSheet);
$("sheet").addEventListener("click", (e) => {
  if (e.target === $("sheet")) closeSheet();
});
function segmented(options, value, onChange) {
  return el("div", { class: "segmented", role: "tablist" }, options.map(([v, label]) =>
    el("button", { class: v === value ? "on" : "", "data-seg": v, onclick: () => onChange(v) }, label)
  ));
}
function sw(checked, onChange, id) {
  const input = el("input", { type: "checkbox", id });
  input.checked = !!checked;
  input.addEventListener("change", () => onChange(input.checked));
  return el("label", { class: "switch" }, input, el("span"));
}
function page(...kids) {
  return el("div", { class: "page" }, ...kids);
}
function head(title, subtitle, right, clamp = false) {
  return el("div", { class: "page-head" }, el("div", {}, el("h1", { class: clamp ? "clamp" : null, title: clamp ? title : null }, title), subtitle ? el("div", { class: "subtitle" }, subtitle) : null), right || null);
}
// Each navigation gets a number. A page that finishes loading after you have
// moved on is dropped instead of drawing over the page you are on.
let navSeq = 0, navTimer = null, freshNav = true;
function showIf(nav, node) { if (nav === navSeq) show(node); }
function show(node) {
  clearTimeout(navTimer);
  const cur = (location.hash.replace(/^#/, "") || "home").split("/")[0];
  const g = GROUPS[GROUP_OF[cur === "run" ? "runs" : cur]];
  if (g && node.classList && node.classList.contains("page")) {
    node.prepend(el("div", { class: "tabs-top" }, g.map(([r, label]) => el("a", { href: `#${r}`, class: r === cur || (cur === "run" && r === "runs") ? "active" : null }, label))));
  }
  $("view").classList.remove("loading");
  $("view").replaceChildren(node);
  if (freshNav) { $("view").scrollTop = 0; freshNav = false; }
}

// ---------- global state ----------
const state = {
  spend: null,
  claude: null,
  live: null, // { kind, id, estimate, cap }
  logs: {},
  queueFilter: "pending",
  runsTab: "history",
  composer: { mode: "agent", agent: null, pipeline: null, project: "", prompt: "" },
  prepared: null,
  vaultSection: null,
};

// ---------- sidebar status: running cost always visible ----------
// Pages wait for status only the first time; after that the 5 s timer keeps it fresh.
const freshStatus = () => (state.spend ? void refreshStatus() : refreshStatus());
async function refreshStatus() {
  try {
    state.spend = await invoke("spend");
  } catch {
    state.spend = null;
  }
  const sp = state.spend;
  if (sp) {
    $("spend-line").textContent = `Plan usage this month ${usd(sp.spent_usd)}`;
    $("running-line").textContent = state.live
      ? `Running · est. ${usd(state.live.estimate, 4)} (cap ${usd(state.live.cap)})`
      : sp.running.length
      ? `Running · est. ${usd(sp.running_expected_usd, 4)}`
      : "No run in progress";
  }
  try {
    const items = await invoke("queue_list");
    const n = items.filter((i) => i.status === "pending").length;
    $("nav-count").textContent = n ? String(n) : "";
  } catch {}
}
async function refreshClaude() {
  try {
    state.claude = await invoke("claude_status");
  } catch (e) {
    state.claude = { installed: false, logged_in: false, message: String(e) };
  }
  const c = state.claude;
  $("claude-dot").className = "dot " + (c.logged_in && c.uses_subscription ? "ok" : "bad");
  $("claude-line").textContent = !c.installed ? "Claude Code not found" : c.logged_in ? (c.uses_subscription ? "Claude connected" : "Claude: not a subscription login") : "Claude not signed in";
}
// Connection lights: green ready, red needs attention, grey off.
async function refreshLights() {
  const box = $("lights");
  if (!box) return;
  let ls;
  try { ls = await invoke("connections"); } catch { return; }
  box.replaceChildren(...ls.map((l) => el("button", { class: "light " + l.state, title: `${l.name}: ${l.detail}`, onclick: () => { if (l.state === "red") toast(`${l.name}: ${l.detail}`, true); location.hash = "#settings"; } }, el("span", { class: "ld" }), l.name)));
}
refreshLights();
setInterval(refreshLights, 30000);
T.event.listen("apps-opened", (ev) => { toast(`Opened ${ev.payload.apps.join(" and ")} for the agents.`); setTimeout(refreshLights, 8000); });
setInterval(() => { if (state.live || (state.spend && state.spend.running.length) || Date.now() - (state.lastStatus || 0) > 20000) { state.lastStatus = Date.now(); refreshStatus(); } }, 5000);

function runningSubtitle() {
  const sp = state.spend;
  const run = state.live ? state.live.id : sp && sp.running.length ? sp.running[0] : null;
  return el("span", {},
    "BrewixGames · ",
    run ? el("span", {}, "Current run ", el("b", {}, run), ` · est. ${usd(state.live ? state.live.estimate : sp.running_expected_usd, 4)}`) : "No run in progress",
    sp ? ` · ${usd(sp.spent_usd)} plan usage this month` : ""
  );
}

// ---------- drag and drop: images dropped onto a zone ----------
// Dropping anywhere else does nothing (instead of opening the file).
window.addEventListener("dragover", (e) => e.preventDefault());
window.addEventListener("drop", (e) => e.preventDefault());
function dropZone(node, onFiles, accept = /^image\//) {
  let depth = 0;
  node.addEventListener("dragenter", (e) => { e.preventDefault(); depth += 1; node.classList.add("drop-on"); });
  node.addEventListener("dragleave", () => { depth = Math.max(0, depth - 1); if (!depth) node.classList.remove("drop-on"); });
  node.addEventListener("dragover", (e) => { e.preventDefault(); e.stopPropagation(); });
  node.addEventListener("drop", (e) => {
    e.preventDefault(); e.stopPropagation(); depth = 0; node.classList.remove("drop-on");
    const files = [...(e.dataTransfer ? e.dataTransfer.files : [])].filter((f) => accept.test(f.type));
    if (files.length) onFiles(files); else toast("Drop an image (PNG, JPG or WebP).", true);
  });
  return node;
}

// ---------- command palette (Ctrl+K or Cmd+K): jump anywhere ----------
const PALETTE = [["Dashboard", "home"], ["Game Ideas", "ideas"], ["Game Brief", "brief"], ["Game Plan", "plans"], ["References", "references"], ["Inspiration (game pictures)", "inspo"], ["Change Requests", "changes"], ["Autopilot (supervise)", "autopilot"], ["Build (runs)", "runs"], ["Team", "agents"], ["Ask Claude", "chat"], ["Player Test", "simulator"], ["Knowledge (vault)", "vault"], ["Game Score", "score"], ["Second Opinion", "review"], ["Tune-Up", "tuneup"], ["Review Queue", "queue"], ["Thumbnails", "thumbnails"], ["Money", "money"], ["Settings", "settings"], ["Find My Files", "files"]];
function openPalette() {
  if ($("palette")) return;
  const input = el("input", { id: "palette-input", placeholder: "Go to...", autocomplete: "off" });
  const list = el("div", { class: "palette-list" });
  const close = () => wrap.remove();
  let sel = 0, shown = PALETTE;
  const draw = () => {
    const q = input.value.trim().toLowerCase();
    shown = PALETTE.filter(([n]) => n.toLowerCase().includes(q));
    sel = Math.min(sel, Math.max(0, shown.length - 1));
    list.replaceChildren(...shown.map(([n, r], i) => el("div", { class: "palette-item" + (i === sel ? " sel" : ""), onclick: () => { close(); location.hash = `#${r}`; } }, n)));
  };
  input.addEventListener("input", () => { sel = 0; draw(); });
  input.addEventListener("keydown", (e) => {
    if (e.key === "Escape") close();
    else if (e.key === "ArrowDown") { sel = Math.min(sel + 1, shown.length - 1); draw(); e.preventDefault(); }
    else if (e.key === "ArrowUp") { sel = Math.max(sel - 1, 0); draw(); e.preventDefault(); }
    else if (e.key === "Enter" && shown[sel]) { close(); location.hash = `#${shown[sel][1]}`; }
  });
  const wrap = el("div", { id: "palette", class: "palette-wrap", onclick: (e) => { if (e.target === wrap) close(); } }, el("div", { class: "palette" }, input, list));
  document.body.append(wrap);
  draw();
  input.focus();
}
$("search-hint").addEventListener("click", openPalette);
window.addEventListener("keydown", (e) => { if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") { e.preventDefault(); openPalette(); } });

// ---------- router ----------
// Six places in the sidebar; grouped pages get tabs along the top.
const GROUPS = {
  home: [["home", "Dashboard"], ["money", "Money"]],
  make: [["autopilot", "Autopilot"], ["brief", "Game Brief"], ["plans", "Game Plan"], ["references", "Graphics"], ["inspo", "Inspiration"], ["runs", "Build"], ["changes", "Changes"], ["agents", "Team"], ["simulator", "Player Test"], ["vault", "Knowledge"]],
  check: [["score", "Game Score"], ["review", "Second Opinion"], ["tuneup", "Tune-Up"], ["queue", "Review Queue"]],
};
const GROUP_OF = {};
for (const [g, pages] of Object.entries(GROUPS)) for (const [r] of pages) GROUP_OF[r] = g;
const routes = { files: viewFiles, score: viewScore, brief: viewBrief, tuneup: viewTuneup, ideas: viewHome, review: viewReview, plans: viewPlans, money: viewMoney, references: viewReferences, inspo: viewInspo, changes: viewChanges, autopilot: viewAutopilot, home: viewDashboard, chat: viewChat, simulator: viewSimulator, thumbnails: viewThumbnails, queue: viewQueue, agents: viewAgents, vault: viewVault, runs: viewRuns, run: viewRunDetail, settings: viewSettings };
function route() {
  const h = location.hash.replace(/^#/, "") || "home";
  const [name, ...rest] = h.split("/");
  const fn = routes[name] || viewDashboard;
  const navName = GROUP_OF[name === "run" ? "runs" : name] || (routes[name] ? name : "home");
  document.querySelectorAll("#nav a").forEach((a) => a.classList.toggle("active", a.dataset.route === navName));
  closeSheet();
  navSeq += 1;
  freshNav = true;
  const mine = navSeq;
  clearTimeout(navTimer);
  navTimer = setTimeout(() => { const v = $("view"); if (v) v.classList.add("loading"); }, 250);
  Promise.resolve(fn(decodeURIComponent(rest.join("/")))).catch((e) => showIf(mine, page(head("Something went wrong"), el("div", { class: "card" }, String(e)))));
}
window.addEventListener("hashchange", route);

// ---------- the game you are working on ----------
// One picker at the top of the sidebar. Every tab follows it, and it is
// remembered after a restart.
try { state.game = localStorage.getItem("game") || ""; } catch { state.game = ""; }
state.projects = [];
async function gameList() {
  const [ap, projects, briefs, plans] = await Promise.all([invoke("autopilot_list").catch(() => []), invoke("projects_list").catch(() => []), invoke("brief_list").catch(() => []), invoke("plans_list").catch(() => [])]);
  state.projects = projects;
  const m = new Map();
  ap.forEach((a) => m.set(a.slug, a.title));
  briefs.forEach((b) => { if (!m.has(b.slug)) m.set(b.slug, b.title); });
  plans.forEach((p) => { if (p.slug && !m.has(p.slug)) m.set(p.slug, p.title); });
  projects.forEach((p) => { const g = p.replace(/^projects\//, ""); if (!m.has(g)) m.set(g, g); });
  return [...m].map(([slug, title]) => ({ slug, title }));
}
// Point every page's notion of "the game" at the picked one.
function syncGame() {
  const pr = `projects/${state.game}`;
  state.project = state.projects.includes(pr) ? pr : "";
  state.chatGame = state.game;
}
function setGame(slug, redraw = true) {
  if (!slug || slug === state.game) return;
  state.game = slug;
  try { localStorage.setItem("game", slug); } catch {}
  syncGame();
  drawGamePick();
  if (redraw) route();
}
async function drawGamePick() {
  const sel = $("game-pick");
  if (!sel) return;
  const games = await gameList();
  if (!games.some((g) => g.slug === state.game)) { state.game = (games[0] && games[0].slug) || ""; try { localStorage.setItem("game", state.game); } catch {} }
  syncGame();
  sel.replaceChildren(
    ...(games.length ? games.map((g) => el("option", { value: g.slug, selected: g.slug === state.game }, g.title)) : [el("option", { value: "" }, "No game yet")]),
    el("option", { value: "__new" }, "+ New game"));
  sel.onchange = () => { if (sel.value === "__new") { sel.value = state.game; location.hash = "#autopilot/new"; } else setGame(sel.value); };
}
drawGamePick();

// =====================================================================
// 0. Home: game ideas from the top 100 Roblox games and current trends
// =====================================================================
const TREND_SCAN = "Trend scan: study the top 100 Roblox experiences right now and what is trending in pop culture in the last 30 days, then pitch 5 original game ideas. Write pitch.md and pitches.json.";
async function viewHome() {
  const nav = navSeq;
  await freshStatus();
  const p = await invoke("pitches_latest").catch(() => null);
  const box = el("div", { id: "pitch-cost" });
  const go = el("button", { class: "btn primary", id: "pitch-new", onclick: () => { costCard(box, "pulse", TREND_SCAN, "Pitch Ideas"); setTimeout(() => box.scrollIntoView({ behavior: "smooth" }), 50); } }, "Pitch New Ideas");
  const risk = { low: "pill white", medium: "pill red-soft", high: "pill red" };
  const design = (x) => {
    state.composer = { ...state.composer, mode: "agent", agent: "planner", prompt: `Plan this pitch from Pulse (run ${p.run_id}): ${x.title}. ${x.one_liner} Core verb: ${x.core_verb}. Twist: ${x.twist}. Trend: ${x.trend}. Audience: ${x.audience}. First 60 seconds: ${x.first_60_seconds}. Keep it original: ${x.originality}` };
    state.runsTab = "new";
    location.hash = "#runs";
  };
  const field = (k, v) => v ? el("div", { class: "pfield" }, el("div", { class: "k" }, k), el("div", {}, Array.isArray(v) ? v.join(", ") : String(v))) : null;
  const body = !p || !Array.isArray(p.pitches) ? [
    el("div", { class: "card empty", style: "padding:56px 24px" }, el("div", { class: "ic-big", html: ICONS.spark }),
      el("div", { class: "empty-title" }, "No ideas yet"),
      el("div", {}, "Pulse studies the 100 most popular Roblox games and what is trending in pop culture right now, then pitches 5 original ideas with sources."),
      el("div", { style: "margin-top:16px" }, go)),
  ] : [
    p.data_status !== "live" ? el("div", { class: "card warn-card" }, el("strong", {}, p.data_status === "partial" ? "Partly live data. " : "No live chart data in this scan. "),
      `Pulse saw ${p.top_games_studied || 0} of the top 100 games listed with numbers, so treat these ideas as a starting point. Pitch again to refresh.`) : null,
    p.trends && p.trends.length ? el("div", { class: "trend-row" }, el("span", { class: "k" }, "Trending now"), p.trends.map((t) => el("span", { class: "pill", title: t.why_it_matters || "" }, t.name))) : null,
    el("div", { class: "pitch-grid" }, p.pitches.map((x) => el("div", { class: "card pitch" },
      el("div", { class: "pitch-top" }, el("span", { class: "rank" }, `#${x.rank}`), el("h2", { style: "margin:0;flex:1" }, x.title), x.score ? el("span", { class: "score" }, `${x.score}/25`) : null),
      el("p", { class: "one-liner" }, x.one_liner),
      el("div", { class: "inline", style: "flex-wrap:wrap;gap:6px;margin-bottom:10px" },
        x.trend ? el("span", { class: "pill" }, x.trend) : null, x.audience ? el("span", { class: "pill" }, x.audience) : null,
        el("span", { class: risk[x.policy_risk] || "pill" }, `Policy risk: ${x.policy_risk}`)),
      field("Core verb", x.core_verb), field("Twist", x.twist), field("Why now", x.why_now), field("First 60 seconds", x.first_60_seconds),
      field("Like", x.comps), field("Money", x.monetization), field("Stays original", x.originality),
      el("div", { class: "inline", style: "margin-top:12px" }, el("button", { class: "btn primary make-this", onclick: () => startAutopilot(x.title, `${x.title}. ${x.one_liner} Core verb: ${x.core_verb}. Twist: ${x.twist}. Audience: ${x.audience}. First 60 seconds: ${x.first_60_seconds}. Keep it original: ${x.originality}`) }, "Make This Game"), el("button", { class: "btn secondary design-this", onclick: () => design(x) }, "Plan Only"))))),
    p.sources && p.sources.length ? el("details", { class: "sources" }, el("summary", {}, `Sources (${p.sources.length}) · studied ${p.top_games_studied || "?"} top games`),
      el("ul", {}, p.sources.map((s) => el("li", {}, `${s.name} · ${s.date || ""} · ${s.url || ""}`)))) : null,
  ];
  showIf(nav, page(head("Game Ideas", el("span", {}, p && p.at ? `From the top Roblox games and current trends · ${String(p.at).slice(0, 10)} · ` : "", runningSubtitle()), p ? go : null), ideaDump(), ...body, box));
}

// =====================================================================
// Dashboard: one look at everything, each widget opens its page
// =====================================================================
async function viewDashboard() {
  const inf = await invoke("studio_info").catch(() => ({}));
  if (!inf.root) { location.hash = "#files"; return; }
  const nav = navSeq;
  await freshStatus();
  const [p, money, plans, rev, queue, games] = await Promise.all([
    invoke("pitches_latest").catch(() => null), invoke("money_summary").catch(() => null), invoke("plans_list").catch(() => []),
    invoke("reviews_list").catch(() => null), invoke("queue_list").catch(() => []), invoke("autopilot_list").catch(() => []),
  ]);
  // Autopilot games: a finished milestone waiting for Andrew comes first.
  const toReview = games.map((g) => [g, g.milestones.find((m) => m.status === "review")]).find(([, m]) => m);
  const making = games.filter((g) => g.status !== "done");
  const nowOf = (g) => (g.milestones.find((m) => m.status === "working" || m.status === "revising") || g.milestones.find((m) => m.status === "review") || {}).title || (g.status === "done" ? "Done" : "Starting");
  const heatSlot = el("div", { id: "heat-slot" });
  invoke("usage_heatmap").then((u) => { if (u) heatSlot.replaceChildren(heatmapCard(u)); }).catch(() => {});
  invoke("scores_list").then((l) => { const n = $("score-widget"); if (!n) return; const s = l.map((g) => g.saved).filter(Boolean); n.textContent = s.length ? s.map((x) => `${x.project}: ${x.score}`).join(" · ") : "Not scored yet"; }).catch(() => {});
  invoke("tuneup_get", { force: false }).then((t) => { const n = $("tune-widget"); if (n && t) n.textContent = t.issues ? `${t.issues} to look at` : "All clean"; }).catch(() => {});
  const w = (title, href, ...body) => el("a", { class: "card widget", href }, el("div", { class: "k" }, title), ...body);
  const plan = plans[0];
  const last = rev && rev.reviews.find((r) => r.report);
  const verdict = last ? ((last.report.match(/NOT SOUND|SOUND WITH FIXES|SOUND/) || ["see report"])[0]) : null;
  const waiting = queue.filter((i) => i.status === "pending").length;
  const sp = state.spend;
  // One clear next step, picked from where the game is right now.
  const left = plan ? plan.total - plan.done : 0;
  const next = toReview ? [`Review ${toReview[0].title}: ${toReview[1].title}`, "The agents finished this milestone. Approve it or send notes; they keep working meanwhile.", "Open Autopilot", `#autopilot/${toReview[0].slug}`]
    : making.length ? [`${making[0].title} is being made`, `${nowOf(making[0])} · ${({ running: "agents working", paused: "paused by you", budget: "stopped at its budget" })[making[0].status] || making[0].status}. Watch and chime in any time.`, "Watch It", `#autopilot/${making[0].slug}`]
    : waiting ? ["Review what the team made", `${waiting} item${waiting === 1 ? "" : "s"} waiting for your yes or no`, "Open Review Queue", "#queue"]
    : !(p && p.pitches) ? ["Get game ideas", "Pulse studies the top Roblox games and trends, then pitches 5 ideas", "Go to Game Ideas", "#ideas"]
    : !plan ? ["Pick an idea and plan it", "Planner mocks up the whole game before anything is built", "Choose an Idea", "#ideas"]
    : left ? [`Finish checking ${plan.title}`, `${left} of ${plan.total} plan items left to check`, "Open Game Plan", `#plans/${plan.run_id}`]
    : !last ? ["Get a second opinion", "Have another AI check the code before you publish", "Open Double-Check", "#review"]
    : [`Build ${plan.title}`, "Every plan item is checked", "Open Game Plan", `#plans/${plan.run_id}`];
  const hero = el("div", { class: "card hero" }, el("div", {}, el("div", { class: "k" }, "Next step"), el("div", { class: "hero-title" }, next[0]), el("div", { class: "note" }, next[1])),
    el("a", { class: "btn primary cta", id: "next-step", href: next[3] }, next[2], el("span", { class: "cta-arrow" }, "→")));
  const earned = money && money.total_robux > 0;
  showIf(nav, page(head("Dashboard", runningSubtitle()),
    hero,
    heatSlot,
    money ? (earned ? moneyStrip(money) : el("a", { class: "fold-line", id: "money-strip", href: "#money" }, "Made this month: $0.00 · add earnings in Money")) : null,
    el("div", { class: "widget-grid" },
      w("Game ideas", "#ideas", p && p.pitches ? el("ol", { class: "widget-list" }, p.pitches.slice(0, 3).map((x) => el("li", {}, x.title))) : el("div", { class: "v small" }, "No ideas yet"), el("div", { class: "note" }, p && p.pitches ? "Open to plan one" : "Open to pitch ideas")),
      w("Game plan", plan ? `#plans/${plan.run_id}` : "#plans", el("div", { class: "v small" }, plan ? plan.title : "No plan yet"),
        plan ? el("div", { class: "plan-bar", style: "margin:8px 0" }, el("div", { class: "plan-fill", style: `width:${plan.total ? Math.round((100 * plan.done) / plan.total) : 0}%` })) : null,
        el("div", { class: "note" }, plan ? `${plan.done} of ${plan.total} checked` : "Plan an idea first")),
      w("Autopilot", making.length ? `#autopilot/${making[0].slug}` : "#autopilot", making.length
        ? el("ul", { class: "widget-list", id: "ap-widget" }, making.slice(0, 3).map((g) => el("li", {}, `${g.title}: ${nowOf(g)}${g.milestones.some((m) => m.status === "review") ? " · needs your review" : ""}`)))
        : el("div", { class: "v small" }, sp && sp.running.length ? `${sp.running.length} run going` : "No game being made"), el("div", { class: "note" }, making.length ? "Open to watch the agents" : "Press Make This Game on an idea")),
      w("Waiting for you", "#queue", el("div", { class: "v" }, String(waiting)), el("div", { class: "note" }, waiting ? "Items to approve or send back" : "Nothing to review")),
      w("Game score", "#score", el("div", { class: "v small", id: "score-widget" }, "..."), el("div", { class: "note" }, "75 is the standard to publish")),
      w("Double-check", "#review", el("div", { class: "v small" }, verdict || "Not checked yet"), el("div", { class: "note" }, last ? last.title : "Have another AI check the code")),
      w("Tune-up", "#tuneup", el("div", { class: "v small", id: "tune-widget" }, "Checking..."), el("div", { class: "note" }, "Free bug, lag and map check")))));
}

// =====================================================================
// Game Brief: your own answers and images for every part of an idea
// =====================================================================
async function viewBrief(arg) {
  const nav = navSeq;
  const briefs = await invoke("brief_list");
  // A remembered idea that was deleted since falls back to the newest one.
  const known = (x) => x && briefs.some((b) => b.slug === x);
  const slug = known(arg) ? arg : known(state.game) ? state.game : known(state.briefSlug) ? state.briefSlug : (briefs[0] && briefs[0].slug);
  if (slug && slug !== state.game) setGame(slug, false);
  const name = el("input", { id: "brief-name", placeholder: "Name your game idea" });
  const start = el("button", { class: "btn primary", id: "brief-new", onclick: async () => { try { state.briefSlug = await invoke("brief_create", { title: name.value }); setGame(state.briefSlug, false); viewBrief(state.briefSlug); } catch (e) { toast(String(e), true); } } }, "Start a Brief");
  const newRow = el("div", { class: "inline" }, name, start);
  if (!slug) {
    showIf(nav, page(head("Game Brief", "Write down everything you want in the game, with images, before the team plans it"),
      el("div", { class: "card" }, el("p", { class: "note", style: "margin-top:0" }, "A checklist that covers every part of a game: the loop, graphics, animation, sound, systems. Fill in what you know and add reference images. The more you give, the closer the plan."), newRow)));
    return;
  }
  state.briefSlug = slug;
  const b = await invoke("brief_get", { slug });
  const fill = el("div", { class: "plan-fill", style: `width:${Math.round((100 * b.done) / b.total)}%` });
  const count = el("span", {}, `${b.done} of ${b.total} covered`);
  const bump = async () => { const n = await invoke("brief_get", { slug }); fill.style.width = `${Math.round((100 * n.done) / n.total)}%`; count.textContent = `${n.done} of ${n.total} covered`; return n; };
  const readFile = (f) => new Promise((res, rej) => { const r = new FileReader(); r.onload = () => res(String(r.result).split(",")[1]); r.onerror = rej; r.readAsDataURL(f); });
  const item = (it) => {
    const tick = el("span", { class: "brief-tick" + (it.text.trim() || it.images.length ? " on" : "") }, "✓");
    const ta = el("textarea", { class: "brief-text", rows: 2, placeholder: it.hint }, it.text);
    ta.addEventListener("change", async () => { try { await (state.briefSave = invoke("brief_set", { slug, id: it.id, text: ta.value })); const n = await bump(); const me = n.items.find((x) => x.id === it.id); tick.classList.toggle("on", !!(me.text.trim() || me.images.length)); } catch (e) { toast(String(e), true); } });
    const thumbs = el("div", { class: "brief-thumbs" });
    const addThumb = (file) => { const img = el("img", { class: "brief-thumb", alt: file }); invoke("refs_image", { project: slug, file }).then((src) => (img.src = src)).catch(() => img.remove()); thumbs.append(img); };
    it.images.forEach(addThumb);
    const upload = async (f) => { try { await invoke("brief_add_image", { slug, id: it.id, name: f.name || "pasted.png", data: await readFile(f) }); const n = await bump(); const me = n.items.find((x) => x.id === it.id); thumbs.replaceChildren(); me.images.forEach(addThumb); tick.classList.add("on"); } catch (e) { toast(String(e), true); } };
    const pick = el("input", { type: "file", accept: "image/png,image/jpeg,image/webp", multiple: true, class: "hidden" });
    pick.addEventListener("change", () => [...pick.files].forEach(upload));
    ta.addEventListener("paste", (e) => { const f = [...(e.clipboardData || {}).items || []].find((x) => x.type.startsWith("image/")); if (f) { e.preventDefault(); upload(f.getAsFile()); } });
    return dropZone(el("div", { class: "plan-item brief-item", "data-id": it.id }, el("div", { class: "inline", style: "gap:10px" }, tick, el("strong", {}, it.title), el("button", { class: "link", style: "margin-left:auto", onclick: () => pick.click() }, "Add image"), pick), ta, thumbs, el("div", { class: "drop-hint" }, "Drop screenshots here")), (fs) => fs.forEach(upload));
  };
  const groups = [...new Set(b.items.map((i) => i.group))];
  const plan = el("button", { class: "btn secondary", id: "brief-plan", onclick: async () => {
    // Save the box still being typed in, then wait for saves, then read.
    if (document.activeElement && document.activeElement.blur) document.activeElement.blur();
    await new Promise((r) => setTimeout(r, 0));
    try { await state.briefSave; } catch {}
    try { const prompt = await invoke("brief_prompt", { slug }); state.composer = { ...state.composer, mode: "agent", agent: "planner", prompt }; state.runsTab = "new"; location.hash = "#runs"; } catch (e) { toast(String(e), true); } } }, "Plan Only");
  const make = el("button", { class: "btn primary", id: "brief-make", onclick: async () => {
    if (document.activeElement && document.activeElement.blur) document.activeElement.blur();
    await new Promise((r) => setTimeout(r, 0));
    try { await state.briefSave; } catch {}
    try { const idea = await invoke("brief_prompt", { slug }); startAutopilot(b.title, idea, slug); } catch (e) { toast(String(e), true); } } }, "Make This Game");
  const pickBrief = briefs.length > 1 ? el("select", { onchange: (e) => viewBrief(e.target.value) }, briefs.map((x) => el("option", { value: x.slug, selected: x.slug === slug }, `${x.title} · ${x.done}/${x.total}`))) : null;
  showIf(nav, page(head(b.title, count, pickBrief),
    el("div", { class: "card" }, el("div", { class: "plan-bar" }, fill),
      el("p", { class: "note" }, "Answer what you can, in your own words. Drag screenshots onto any item, paste them, or use Add image (they are saved to the vault for the whole team). Answers save when you click out of a box. Empty items are fine: Planner decides those and asks you to confirm."),
      el("div", { class: "inline", style: "justify-content:space-between;flex-wrap:wrap;gap:10px" }, el("div", { class: "inline" }, make, plan), el("details", {}, el("summary", { class: "link" }, "Start another idea"), el("div", { style: "margin-top:8px" }, newRow)))),
    ideaDump({ slug, dump: b.dump || "" }),
    ...groups.map((g) => el("div", { class: "card plan-sec" }, el("h2", {}, g), b.items.filter((i) => i.group === g).map(item)))));
}

// =====================================================================
// Game Score: one number out of 100; under the standard, keep working
// =====================================================================
function scoreCard(v) {
  const ring = el("div", { class: "score-ring " + (v.ready ? "ok" : "low") }, el("div", { class: "score-num" }, String(v.score)), el("div", { class: "note" }, `Grade ${v.grade}`));
  const keep = el("button", { class: "btn primary", onclick: () => {
    state.composer = { ...state.composer, mode: "pipeline", pipeline: "improve-build", project: `projects/${v.project}`, prompt: `Raise this game's score from ${v.score} to at least ${v.standard}. Do these, most valuable first, without breaking what works:\n` + v.todo.slice(0, 12).map((t) => `- ${t}`).join("\n") };
    state.runsTab = "new"; location.hash = "#runs"; } }, "Keep Working on It");
  return el("div", { class: "card" },
    el("div", { class: "inline", style: "gap:22px;align-items:center" }, ring,
      el("div", { style: "flex:1" }, el("h2", { style: "margin:0" }, v.project),
        el("div", { class: "note" }, v.ready ? `Meets the standard (${v.standard}). Keep polishing, or move on to publishing.` : `Below the standard of ${v.standard}. ${v.todo.length} thing(s) to do before it is ready.`),
        el("div", { class: "inline", style: "margin-top:10px" }, !v.ready ? keep : null, el("button", { class: "btn secondary", onclick: () => viewScore(v.project) }, "Score Again")))),
    el("div", { class: "score-parts" }, v.parts.map((p) => el("div", { class: "score-part" },
      el("div", { class: "inline", style: "justify-content:space-between" }, el("span", {}, p.name), el("span", { class: "note" }, `${p.got} / ${p.max}`)),
      el("div", { class: "plan-bar" }, el("div", { class: "plan-fill", style: `width:${Math.round((100 * p.got) / p.max)}%;background:${p.got / p.max >= 0.75 ? "#30D158" : p.got / p.max >= 0.5 ? "#FF9F0A" : "#FF453A"}` })),
      p.missing.length ? el("ul", { class: "score-todo" }, p.missing.map((m) => el("li", {}, m))) : null))));
}
async function viewScore(project) {
  const nav = navSeq;
  const list = await invoke("scores_list");
  if (!list.length) { showIf(nav, page(head("Game Score"), el("div", { class: "card" }, "No game projects yet. Build one first."))); return; }
  const target = project || null;
  const cards = await Promise.all(list.map(async (g) => (target === g.project || !g.saved) ? await invoke("score_run", { project: g.project }).catch((e) => ({ error: String(e), project: g.project })) : g.saved));
  showIf(nav, page(head("Game Score", "How ready each game is, out of 100. 75 is the standard to publish."),
    el("div", { class: "card note" }, "Scored from what the studio can check itself: does it build, is the code clean, graphics and feel (lighting, materials, motion, particles, sound, UI), what keeps players playing (saves, daily streaks, playtime gifts, goals, leaderboards, friends, favorite reminder, badges), fair monetization, plus the latest Player Test and Second Opinion. Free, no AI."),
    ...cards.map((v) => v.error ? el("div", { class: "card" }, `${v.project}: ${v.error}`) : scoreCard(v))));
}

// =====================================================================
// Find My Files: studios, vaults and apps anywhere on this computer
// =====================================================================
async function viewFiles() {
  const nav = navSeq;
  showIf(nav, page(head("Find My Files", "Searching your drives..."), el("div", { class: "card" }, "Looking in your home folder, Documents, Desktop, Downloads, OneDrive and every drive. This can take a minute.")));
  const [found, info, st] = await Promise.all([invoke("locate_scan"), invoke("studio_info"), invoke("settings_get")]);
  const label = { studio: "Studio folder (vault, games, agents)", vault: "Vault on its own", blender: "Blender", obsidian: "Obsidian", claude: "Claude Code copy" };
  const act = async (fn, args) => { try { toast(await invoke(fn, args)); viewFiles(); } catch (e) { toast(String(e), true); } };
  const moveHere = async () => { const dest = await invoke("pick_folder"); if (!dest) return; try { toast(await invoke("studio_move", { dest: dest + (dest.endsWith("BrewixGames") ? "" : (dest.includes("\\\\") ? "\\\\" : "/") + "BrewixGames") })); viewFiles(); } catch (e) { toast(String(e), true); } };
  const row = (f) => el("div", { class: "row" },
    el("div", { class: "row-main", style: "flex:1;min-width:0" }, el("div", { class: "row-title" }, label[f.kind] || f.kind, f.current ? el("span", { class: "pill white", style: "margin-left:8px" }, "In use") : null),
      el("div", { class: "note", style: "word-break:break-all" }, f.path),
      el("div", { class: "note" }, [f.notes ? `${f.notes} notes` : null, f.projects ? `${f.projects} game(s)` : null, f.modified ? `changed ${f.modified}` : null].filter(Boolean).join(" · "))),
    el("div", { class: "inline" },
      f.kind === "vault" ? el("button", { class: "btn secondary", onclick: () => act("locate_merge_vault", { path: f.path }) }, "Copy Into My Vault")
        : f.current ? (f.kind === "studio" ? el("button", { class: "btn secondary", onclick: moveHere }, "Move to Another Folder") : null)
        : el("button", { class: "btn primary", onclick: () => act("locate_use", { kind: f.kind, path: f.path }) }, "Use This One")));
  showIf(nav, page(head("Find My Files", `${found.length} found`, el("button", { class: "btn secondary", onclick: viewFiles }, "Search Again")),
    el("div", { class: "card" },
      el("div", { class: "row-title" }, "In use now"),
      el("div", { class: "note", style: "word-break:break-all" }, info.root ? `Studio: ${info.root}` : `No studio folder: ${info.root_error || ""}`),
      el("div", { class: "note", style: "word-break:break-all" }, `Downloads folder: ${st.tools_dir || "(default, inside the studio)"} · Settings file: ${info.config_file}`),
      el("p", { class: "note" }, "Use This One points the app at a folder where it is. Move to Another Folder copies your whole studio (vault, games, runs) to a folder you pick, such as one on your D drive, and switches to it; the old folder is kept until you delete it. A vault found on its own can be copied into yours: notes you already have are never overwritten.")),
    found.length ? el("div", { class: "list" }, found.map(row)) : el("div", { class: "card" }, "Nothing found. If your files are somewhere unusual, set the studio folder in Settings.")));
}

// Word-dump an idea, then let the model sort it into the checklist and fill
// every empty item with a default (marked "Default:").
function ideaDump({ slug = null, dump = "" } = {}) {
  const name = slug ? null : el("input", { id: "idea-name", placeholder: "Name it (you can change it later)", style: "margin-bottom:8px;width:100%" });
  const box = el("textarea", { id: "idea-dump", rows: slug ? 4 : 7, style: "width:100%", placeholder: "Word-vomit the whole idea here: the vibe, what you do, what you collect, what it looks like, games it is like, anything. Messy is fine." }, dump);
  const status = el("span", { class: "note", id: "idea-status" }, "");
  const go = el("button", { class: "btn primary", id: "idea-fill", onclick: async () => {
    if (!box.value.trim() && !confirm("Nothing written yet. Fill every item with a default version?")) return;
    go.disabled = true; status.textContent = "Sorting your idea into the checklist... (about 10-30 seconds)";
    try {
      const s2 = slug || await invoke("brief_create", { title: ((name && name.value) || box.value.split(/[.\n]/)[0] || "New idea").slice(0, 40) });
      const n = await invoke("brief_autofill", { slug: s2, dump: box.value });
      toast(`Filled ${n} item(s). Your own answers were kept.`);
      state.briefSlug = s2;
      if (location.hash === `#brief/${s2}`) viewBrief(s2); else location.hash = `#brief/${s2}`;
    } catch (e) { go.disabled = false; status.textContent = ""; toast(String(e), true); }
  } }, slug ? "Fill Empty Items" : "Fill the Checklist");
  return el("div", { class: "card", id: "idea-card" },
    el("h2", { style: "margin:0 0 6px" }, slug ? "Your idea, in your words" : "Your own idea"),
    el("p", { class: "note", style: "margin-top:0" }, "Write it however it comes out. Fill the Checklist sorts it into the 19-item game checklist and writes a sensible default (marked Default:) for anything you left out. Your own answers are never overwritten. Uses a little Claude usage (the smallest model), or your free local model if that is on."),
    name, box, el("div", { class: "inline", style: "margin-top:10px;gap:12px" }, go, status));
}

// Claude Code usage heatmap (idea from seunggabi/ai-heatmap, MIT): one square
// per day for the last year, darker to brighter red as more tokens are used.
const HEAT = ["#232326", "#4A1714", "#8A231A", "#C92D20", "#FF453A"];
const bigNum = (n) => n >= 1e9 ? `${(n / 1e9).toFixed(2)}B` : n >= 1e6 ? `${(n / 1e6).toFixed(1)}M` : n >= 1e3 ? `${(n / 1e3).toFixed(1)}K` : String(n);
function heatmapCard(u) {
  const byDate = Object.fromEntries(u.days.map((d) => [d.date, d]));
  const vals = u.days.map((d) => d.tokens).filter((t) => t > 0).sort((a, b) => a - b);
  const q = (f) => vals.length ? vals[Math.min(vals.length - 1, Math.floor(f * vals.length))] : 0;
  const cuts = [q(0.25), q(0.5), q(0.75)];
  const level = (t) => (!t ? 0 : t <= cuts[0] ? 1 : t <= cuts[1] ? 2 : t <= cuts[2] ? 3 : 4);
  const iso = (d) => `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
  const end = new Date(); const start = new Date(end); start.setDate(start.getDate() - 364 - start.getDay());
  const NS = "http://www.w3.org/2000/svg", B = 11, G = 3, L = 28, T = 16;
  const svg = document.createElementNS(NS, "svg");
  const weeks = Math.ceil((end - start) / 864e5 / 7) + 1;
  svg.setAttribute("viewBox", `0 0 ${L + weeks * (B + G)} ${T + 7 * (B + G)}`); svg.setAttribute("class", "heat");
  const add = (tag, a, text) => { const n = document.createElementNS(NS, tag); for (const [k, v] of Object.entries(a)) n.setAttribute(k, v); if (text) n.textContent = text; svg.append(n); return n; };
  ["Mon", "Wed", "Fri"].forEach((d, i) => add("text", { x: 0, y: T + (1 + i * 2) * (B + G) + 9, class: "heat-l" }, d));
  let lastMonth = -1;
  for (let d = new Date(start), i = 0; d <= end; d.setDate(d.getDate() + 1), i++) {
    const w = Math.floor(i / 7), dow = d.getDay(), day = byDate[iso(d)];
    if (d.getDate() <= 7 && d.getMonth() !== lastMonth && dow === 0) { lastMonth = d.getMonth(); add("text", { x: L + w * (B + G), y: 10, class: "heat-l" }, d.toLocaleString(undefined, { month: "short" })); }
    const r = add("rect", { x: L + w * (B + G), y: T + dow * (B + G), width: B, height: B, rx: 2, fill: HEAT[level(day ? day.tokens : 0)] });
    const tip = document.createElementNS(NS, "title");
    tip.textContent = day ? `${iso(d)}\n${day.tokens.toLocaleString()} tokens\nIn ${day.input.toLocaleString()} · Out ${day.output.toLocaleString()}\nCache read ${day.cache_read.toLocaleString()} · write ${day.cache_write.toLocaleString()}\n${day.messages} messages · ${usd(day.cost)} API-equivalent` : `${iso(d)}\nNo usage`;
    r.append(tip);
  }
  const L2 = u.lifetime;
  const stat = (k, v, n) => el("div", {}, el("div", { class: "k" }, k), el("div", { class: "v small" }, v), n ? el("div", { class: "note" }, n) : null);
  return el("div", { class: "card", id: "usage-heatmap" },
    el("div", { class: "inline", style: "justify-content:space-between;margin-bottom:10px" }, el("div", { class: "k" }, "Claude usage"),
      el("div", { class: "inline heat-key" }, el("span", { class: "note" }, "Less"), HEAT.map((c) => el("span", { class: "heat-sw", style: `background:${c}` })), el("span", { class: "note" }, "More"))),
    svg,
    el("div", { class: "heat-stats" },
      stat("Today", `${bigNum(u.today.tokens)} tokens`, usd(u.today.cost) + " API-equivalent"),
      stat("Lifetime", `${bigNum(L2.tokens)} tokens`, `In ${bigNum(L2.input)} · Out ${bigNum(L2.output)} · Cache ${bigNum(L2.cache_read + L2.cache_write)}`),
      stat("Days active", String(L2.days_active), `${u.streak}-day streak`),
      stat("Busiest day", u.busiest ? bigNum(u.busiest.tokens) : "-", u.busiest ? u.busiest.date : ""),
      stat("Messages", L2.messages.toLocaleString(), `${usd(L2.cost)} API-equivalent; your plan is not billed this`)));
}

// A Claude model picker: cheapest first, with a plain hint of size.
const MODEL_NAMES = { "claude-haiku-5-5": "Haiku 5.5 · fastest, lightest", "claude-sonnet-5-5": "Sonnet 5.5 · balanced", "claude-opus-5-5": "Opus 5.5 · strongest, uses more", "claude-fable-5-1": "Fable 5.1 · most capable, uses the most" };
function modelSelect(id, models, current, defLabel, onchange) {
  const sel = el("select", { id, class: "model-select", title: "Which Claude model to use" }, el("option", { value: "" }, defLabel), models.map((m) => el("option", { value: m.id, selected: m.id === current }, MODEL_NAMES[m.id] || m.id)));
  sel.addEventListener("change", () => onchange(sel.value));
  return sel;
}

// =====================================================================
// Tune-Up: free automatic bug, lag and map checks, plus temp-file cleanup
// =====================================================================
async function viewTuneup(arg) {
  const nav = navSeq;
  showIf(nav, page(head("Tune-Up", "Checking your games..."), el("div", { class: "card" }, "Scanning code and maps. This is free and uses no AI.")));
  let rep;
  try { rep = await invoke("tuneup_get", { force: arg === "now" }); } catch (e) { showIf(nav, page(head("Tune-Up"), el("div", { class: "card" }, String(e)))); return; }
  if (arg === "now") { history.replaceState(null, "", "#tuneup"); }
  const s = await invoke("settings_get");
  const every = el("select", { id: "tuneup-days", onchange: async () => { await invoke("settings_patch", { patch: { tuneup_days: parseInt(every.value, 10) } }).catch((e) => toast(String(e), true)); toast("Saved."); } },
    [[0, "Never"], [1, "Every day"], [3, "Every 3 days"], [7, "Every week"], [30, "Every month"]].map(([v, l]) => el("option", { value: v, selected: v === s.tuneup_days }, l)));
  const runNow = el("button", { class: "btn primary", id: "tuneup-run", onclick: () => viewTuneup("now") }, "Run Tune-Up Now");
  const sev = { blocker: "pill red", major: "pill red-soft", minor: "pill" };
  const proj = ([name, p]) => {
    const m = p.map || {};
    const fixable = p.findings.some((x) => /Fix Safe Items/.test(x.fix));
    return el("div", { class: "card" }, el("div", { class: "inline", style: "justify-content:space-between" }, el("h2", { style: "margin:0" }, name),
        el("div", { class: "inline" },
          fixable ? el("button", { class: "btn secondary", onclick: async (e) => { e.target.disabled = true; try { toast(await invoke("tuneup_fix", { project: name })); } catch (err) { toast(String(err), true); } viewTuneup("now"); } }, "Fix Safe Items") : null,
          p.findings.length ? el("button", { class: "btn secondary", onclick: () => { state.composer = { ...state.composer, mode: "pipeline", pipeline: "improve-build", project: `projects/${name}`, prompt: "Fix these tune-up findings without changing how the game plays:\n" + p.findings.slice(0, 25).map((x) => `- [${x.severity}] ${x.file}:${x.line} ${x.what}. ${x.fix}`).join("\n") }; state.runsTab = "new"; location.hash = "#runs"; } }, "Have the Team Fix These") : null)),
      el("div", { class: "note" }, `${p.code.files} code files, ${p.code.lines} lines` + (m.parts !== undefined ? ` · map: ${m.instances} objects, ${m.parts} parts, ${m.lights} lights, ${m.emitters} particle emitters, ${m.scripts} scripts` : "")),
      p.findings.length ? el("div", { class: "tune-list" }, p.findings.map((x) => el("div", { class: "tune-row" }, el("span", { class: sev[x.severity] || "pill" }, x.severity),
        el("div", {}, el("div", {}, x.what, el("span", { class: "note" }, x.line ? `  ${x.file}:${x.line}` : `  ${x.file}`)), el("div", { class: "note" }, x.fix)))))
        : el("p", { style: "margin:10px 0 0" }, "Nothing found. Code and map look clean."));
  };
  const entries = rep && rep.projects ? Object.entries(rep.projects) : [];
  showIf(nav, page(head("Tune-Up", rep ? `Last run ${String(rep.at).slice(0, 16).replace("T", " ")} · ${rep.issues} thing${rep.issues === 1 ? "" : "s"} to look at` : "Not run yet", runNow),
    el("div", { class: "card" }, el("p", { class: "note", style: "margin-top:0" }, "Free and automatic, no AI: looks for code that causes bugs or lag, reviews each map for heavy parts, lights and effects, and clears old temp files (raw run logs over 30 days, old update installers). Your games, plans and reports are never deleted."),
      el("div", { class: "inline" }, el("label", { for: "tuneup-days" }, "Run automatically"), every),
      rep && rep.cleaned_files ? el("div", { class: "note", style: "margin-top:8px" }, `Cleared ${rep.cleaned_files} old temp file(s), ${rep.cleaned_mb} MB.`) : null),
    ...entries.map(proj),
    rep && !entries.length ? el("div", { class: "card" }, "No game projects yet.") : null));
}

// =====================================================================
// Second Opinion: a different AI (not Claude) reviews a game or this app
// =====================================================================
async function viewReview() {
  const nav = navSeq;
  const d = await invoke("reviews_list", { engines: true });
  const target = el("select", { id: "review-target" }, d.projects.map((p) => el("option", { value: p }, `Game: ${p}`)), el("option", { value: "app" }, "The BrewixGames app itself"));
  const status = el("div", { class: "hint", id: "review-status" }, "");
  const start = (engine) => async () => {
    if (engine !== "local" && !confirm("This sends the code to an outside AI company. Game code is fine to share; never include passwords or keys. Continue?")) return;
    if (state.reviewing) { toast("A review is already running."); return; }
    status.textContent = "Starting...";
    state.reviewing = true;
    try { await invoke("review_start", { target: target.value, engine }); } catch (e) { state.reviewing = false; status.textContent = ""; toast(String(e), true); }
  };
  const engines = el("div", { class: "inline", style: "flex-wrap:wrap;gap:8px;margin-top:12px" },
    el("button", { class: "btn primary", id: "review-local", onclick: start("local") }, d.local_model ? `Review on This Computer (${d.local_model})` : "Review on This Computer"),
    el("button", { class: "btn secondary", id: "review-chatgpt", onclick: start("chatgpt") }, "Review in ChatGPT"),
    el("button", { class: "btn secondary", id: "review-codex", disabled: !d.codex, title: d.codex ? "" : "Install OpenAI's Codex CLI and sign in once to use this" }, d.codex ? "Review with Codex" : "Codex not installed"));
  if (d.codex) engines.lastChild.addEventListener("click", start("codex"));
  const card = (r) => {
    const body = r.report ? el("pre", { class: "review-report" }, r.report) : null;
    const paste = !r.report && r.engine === "chatgpt" ? (() => {
      const ta = el("textarea", { rows: 6, placeholder: "Paste ChatGPT's answer here", class: "review-paste" });
      return el("div", {}, el("p", { class: "note" }, "ChatGPT opened in your browser. Drag pack.md from the review folder into the chat, send it, then paste the answer here."),
        el("div", { class: "inline" }, el("button", { class: "btn secondary", onclick: () => invoke("review_reveal", { id: r.id }) }, "Show pack.md"),
          el("button", { class: "btn primary", onclick: async () => { await invoke("review_answer", { id: r.id, text: ta.value }).catch((e) => toast(String(e), true)); viewReview(); } }, "Save Answer")), ta);
    })() : null;
    return el("details", { class: "card", open: r === d.reviews[0] }, el("summary", { class: "row-title" }, `${r.title} · ${r.engine} · ${r.files} files · ${String(r.at || "").slice(0, 16).replace("T", " ")}`), body, paste);
  };
  showIf(nav, page(head("Second Opinion", "A different AI checks the code, so Claude is not grading its own work"),
    el("div", { class: "card" }, el("div", { class: "field" }, el("label", { for: "review-target" }, "What to check"), target),
      el("p", { class: "note" }, "On This Computer is free and private (a local model in Ollama). ChatGPT uses your free ChatGPT account in the browser. Codex uses OpenAI's free Codex CLI if you installed it and signed in. No API keys."),
      engines, status),
    ...d.reviews.map(card)));
}
T.event.listen("review-line", (ev) => { const n = $("review-status"); if (n) n.textContent = ev.payload.line; });
T.event.listen("review-done", (ev) => {
  state.reviewing = false;
  if (ev.payload.error) toast(ev.payload.error, true); else toast(ev.payload.engine === "chatgpt" ? "pack.md is ready. Drag it into ChatGPT." : "Review finished.");
  if ((location.hash || "") === "#review") viewReview();
});

// =====================================================================
// Game Plan: Planner's checklist with mockups. Build only at 100%.
// =====================================================================
async function viewPlans(id) {
  const nav = navSeq;
  const [plans, chg] = await Promise.all([invoke("plans_list"), invoke("changes_list").catch(() => ({ changes: [] }))]);
  if (!plans.length) {
    showIf(nav, page(head("Game Plan", "Every part of the game, mocked up, for you to check off before anything is built"),
      el("div", { class: "card empty", style: "padding:56px 24px" }, el("div", { class: "empty-title" }, "No plans yet"),
        el("div", {}, "On Home, press Plan This Game on an idea. Planner writes the full plan with mockups of the map, HUD, shop and character effects. Nothing is built until you check every item."))));
    return;
  }
  const p = plans.find((x) => x.run_id === id) || plans.find((x) => x.slug === state.game) || plans[0];
  const ap = p.approvals || {};
  const checked = (ap.checked || {}), notes = (ap.notes || {});
  const bar = el("div", { class: "plan-bar" }, el("div", { class: "plan-fill", style: `width:${p.total ? Math.round((100 * p.done) / p.total) : 0}%` }));
  const box = el("div", {});
  const all = p.total > 0 && p.done === p.total;
  const build = el("button", { class: "btn primary", id: "plan-build", disabled: !all, onclick: () => { state.composer = { ...state.composer, mode: "plan", planRun: p.run_id }; estimate(box); } }, all ? "Build This Game" : `Build This Game · ${p.total - p.done} left to check`);
  const mockImg = (path) => {
    const img = el("img", { class: "plan-mock", alt: path });
    invoke("plan_mock", { runId: p.run_id, path }).then((svg) => { img.src = "data:image/svg+xml;charset=utf-8," + encodeURIComponent(svg); }).catch(() => img.replaceWith(el("div", { class: "note" }, `Mockup missing: ${path}`)));
    return img;
  };
  const target = "plan:" + p.run_id;
  const itemChanges = (iid) => chg.changes.filter((c) => c.target === target && c.item === iid);
  const changedBy = ap.changed || {};
  const pending = () => p.sections.flatMap((s) => s.items).filter((it) => (notes[it.id] || "").trim() && !itemChanges(it.id).some((c) => ["queued", "working", "paused"].includes(c.status)));
  const item = (it) => {
    const cb = el("input", { type: "checkbox", class: "plan-check", checked: !!checked[it.id], onchange: async () => { await invoke("plan_set", { runId: p.run_id, item: it.id, checked: cb.checked, note: null }).catch((e) => toast(String(e), true)); viewPlans(p.run_id); } });
    const note = el("textarea", { class: "plan-note", rows: 1, placeholder: "Change request (optional)" }, notes[it.id] || "");
    const saveNote = () => { notes[it.id] = note.value; return invoke("plan_set", { runId: p.run_id, item: it.id, checked: null, note: note.value }).catch((e) => toast(String(e), true)); };
    note.addEventListener("change", saveNote);
    const sendOne = el("button", { class: "btn secondary plan-send", disabled: !note.value.trim(), onclick: async () => { await saveNote(); sendChanges(target, [{ item: it.id, text: note.value }], () => viewPlans(p.run_id)); } }, "Send");
    note.addEventListener("input", () => { sendOne.disabled = !note.value.trim(); });
    note.addEventListener("keydown", (e) => { if (e.key === "Enter" && !e.shiftKey && note.value.trim()) { e.preventDefault(); sendOne.click(); } });
    const last = itemChanges(it.id).slice(-1)[0];
    const status = last && ["queued", "working", "paused"].includes(last.status) ? el("div", { class: "plan-chg" }, changePill(last), el("span", { class: "note" }, last.text))
      : changedBy[it.id] && last && last.status === "done" ? el("div", { class: "plan-chg done" }, el("span", { class: "pill white" }, `Changed by ${cap(last.agent)}`), el("span", { class: "note" }, `${last.summary || ""} Check it again.`))
      : last && last.status === "failed" ? el("div", { class: "plan-chg" }, changePill(last), el("button", { class: "link", onclick: () => (location.hash = "#run/" + last.run_id) }, "Why"))
      : null;
    return el("div", { class: "plan-item", "data-item": it.id }, el("label", { class: "inline", style: "align-items:flex-start;gap:10px" }, cb, el("span", {}, it.text)), status, it.mock ? mockImg(it.mock) : null, el("div", { class: "plan-note-row" }, note, sendOne));
  };
  const others = plans.length > 1 ? el("select", { onchange: (e) => { location.hash = `#plans/${e.target.value}`; } }, plans.map((x) => el("option", { value: x.run_id, selected: x.run_id === p.run_id }, `${x.title} · ${String(x.at || "").slice(0, 10)}`))) : null;
  showIf(nav, page(head(p.title, `${p.one_liner} · ${p.done} of ${p.total} checked`, others),
    el("div", { class: "card" }, bar, el("p", { class: "note" }, "Check each item you are happy with. Write a change request on anything you are not and press Send (or Send All): the right agent for each item makes the change, up to 3 at once, and the item comes back for you to check again. Building only unlocks at 100%."),
      el("div", { class: "inline" }, build, (() => { const n = pending().length; return el("button", { class: "btn secondary", id: "plan-send-all", disabled: !n, onclick: () => sendChanges(target, pending().map((it) => ({ item: it.id, text: notes[it.id] })), () => viewPlans(p.run_id)) }, n ? `Send All Change Requests (${n})` : "Send All Change Requests"); })(), el("button", { class: "btn secondary", id: "plan-figma", onclick: async () => { try { await invoke("plan_to_figma", { runId: p.run_id }); toast("Drag the SVG files from the folder into your Figma file. Each screen arrives with named layers."); } catch (e) { toast(String(e), true); } } }, "Edit in Figma")), box),
    el("div", { class: "chips", style: "margin:0 0 14px" }, p.sections.map((s) => el("a", { class: "chip", onclick: () => { const n = document.getElementById(`sec-${s.id}`); if (n) n.scrollIntoView({ behavior: "smooth" }); } }, `${s.title} ${s.items.filter((i) => checked[i.id]).length}/${s.items.length}`))),
    ...p.sections.map((s) => el("div", { class: "card plan-sec", id: `sec-${s.id}` }, el("h2", {}, s.title, el("span", { class: "note" }, `  ${s.items.filter((i) => checked[i.id]).length}/${s.items.length}`)), s.items.map(item)))));
}

// =====================================================================
// Money: Robux earned, converted to dollars at the DevEx rate
// =====================================================================
const robux = (n) => `${Math.round(n).toLocaleString()} R$`;
const dollars = (n) => `$${Number(n).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
function moneyStrip(m) {
  return el("div", { class: "money-strip", id: "money-strip", onclick: () => (location.hash = "#money") },
    el("div", {}, el("div", { class: "k" }, "Made this month"), el("div", { class: "v" }, dollars(m.this_month_usd)), el("div", { class: "note" }, robux(m.this_month_robux))),
    el("div", {}, el("div", { class: "k" }, "All time"), el("div", { class: "v" }, dollars(m.total_usd)), el("div", { class: "note" }, robux(m.total_robux))),
    el("div", {}, el("div", { class: "k" }, "Last 30 days"), el("div", { class: "v" }, dollars(m.last_30_robux * m.rate)), el("div", { class: "note" }, robux(m.last_30_robux))),
    el("div", {}, el("div", { class: "k" }, "Best game"), el("div", { class: "v small" }, m.per_game[0] ? m.per_game[0][0] : "None yet"), el("div", { class: "note" }, m.per_game[0] ? dollars(m.per_game[0][2]) : "")));
}
async function viewMoney() {
  const nav = navSeq;
  await freshStatus();
  const m = await invoke("money_summary");
  const today = new Date().toISOString().slice(0, 10);
  const date = el("input", { type: "date", id: "money-date", value: today });
  const game = el("input", { type: "text", id: "money-game", placeholder: "Game name" });
  const amt = el("input", { type: "number", id: "money-robux", placeholder: "Robux earned", min: "0" });
  const csv = el("input", { type: "file", accept: ".csv,text/csv", style: "display:none", onchange: async () => {
    const f = csv.files[0]; if (!f) return;
    try { const n = await invoke("money_import", { text: await f.text(), game: game.value.trim() || "My game" }); toast(`Imported ${n} new rows.`); viewMoney(); } catch (e) { toast(String(e), true); }
  } });
  const max = Math.max(1, ...m.per_month.map((x) => x[1]));
  const pct = Math.min(100, (m.total_robux / m.devex_minimum) * 100);
  const delEntry = async (i) => { await invoke("money_remove", { index: i }); viewMoney(); };
  showIf(nav, page(head("Money", el("span", {}, `Robux converted at $${m.rate} per Robux (Roblox's standard DevEx rate) · `, runningSubtitle())),
    moneyStrip(m),
    el("div", { class: "card" }, el("div", { class: "row-title" }, "Cash-out progress"),
      el("div", { class: "hint" }, `Roblox pays out from ${m.devex_minimum.toLocaleString()} earned Robux (${dollars(m.devex_minimum * m.rate)}). You have ${robux(m.total_robux)} recorded.`),
      el("div", { class: "bar", style: "margin-top:10px" }, el("div", { style: `width:${pct}%` }))),
    m.per_month.length ? el("div", { class: "card" }, el("div", { class: "row-title" }, "By month"),
      el("div", { class: "month-bars" }, m.per_month.map(([mo, r, u]) => el("div", { class: "mbar", title: `${robux(r)} · ${dollars(u)}` }, el("div", { class: "fill", style: `height:${(r / max) * 100}%` }), el("div", { class: "note" }, mo.slice(2)))))) : null,
    m.per_game.length ? el("div", { class: "list" }, m.per_game.map(([g, r, u]) => el("div", { class: "srow" }, el("div", { class: "grow label" }, g), el("span", { class: "note" }, robux(r)), el("span", { class: "v small", style: "margin-left:16px" }, dollars(u))))) : null,
    el("div", { class: "card" }, el("div", { class: "row-title" }, "Add earnings"),
      el("div", { class: "hint" }, "Type what Roblox's Creator Dashboard shows, or import its CSV export. BrewixGames never signs in to Roblox."),
      el("div", { class: "inline", style: "gap:8px;margin-top:10px;flex-wrap:wrap" }, date, game, amt,
        el("button", { class: "btn primary", id: "money-add", onclick: async (ev) => {
          ev.target.disabled = true;
          try { await invoke("money_add", { date: date.value, game: game.value, robux: parseInt(amt.value || "0", 10), note: "" }); toast("Added."); viewMoney(); } catch (e) { toast(String(e), true); }
        } }, "Add"),
        el("button", { class: "btn secondary", onclick: () => csv.click() }, "Import CSV"), csv)),
    m.entries.length ? el("details", { class: "sources" }, el("summary", {}, `All entries (${m.entries.length})`),
      el("div", { class: "list" }, m.entries.map((e, i) => el("div", { class: "srow" }, el("div", { class: "grow" }, `${e.date} · ${e.game}`), el("span", { class: "note" }, robux(e.robux)),
        el("button", { class: "link", style: "margin-left:12px", onclick: () => delEntry(i) }, "Delete"))).reverse())) : null));
}

// =====================================================================
// References: images and video frames for a game, filed into the vault
// =====================================================================
const REF_CATS = ["Character", "Model", "Environment", "UI", "Animation", "Effects", "Sound and music", "Other"];
state.refCat = "Character";
async function viewReferences() {
  const nav = navSeq;
  await freshStatus();
  const pick = await projectPicker(viewReferences);
  const project = state.project;
  const refs = project ? await invoke("refs_list", { project }).catch(() => []) : [];
  const cat = el("select", { id: "ref-cat" }, REF_CATS.map((c) => el("option", { value: c }, c)));
  cat.value = state.refCat; cat.addEventListener("change", () => (state.refCat = cat.value));
  const note = el("textarea", { id: "ref-note", placeholder: "What should the team take from this? (for example: the round friendly eyes, the glowing trim, this camera angle)", style: "min-height:56px" });
  const add = async (name, data, source) => {
    if (!project) { toast("Choose a game first.", true); return; }
    try { await invoke("refs_add", { project, name, data, category: cat.value, note: note.value, source }); toast("Added to the vault and the game folder."); }
    catch (e) { toast(String(e), true); }
  };
  const readFile = (f) => new Promise((ok) => { const r = new FileReader(); r.onload = () => ok(r.result); r.readAsDataURL(f); });
  const files = el("input", { type: "file", id: "ref-files", accept: "image/png,image/jpeg,image/webp", multiple: true, style: "display:none", onchange: async () => {
    for (const f of files.files) await add(f.name, await readFile(f), `added from ${f.name}`);
    viewReferences();
  } });
  // Video: play it here and grab frames; frames are what agents can look at.
  const video = el("video", { id: "ref-video", controls: true, style: "width:100%;max-height:300px;display:none;border-radius:10px;background:#000" });
  let videoName = "";
  const vfile = el("input", { type: "file", id: "ref-video-file", accept: "video/*", style: "display:none", onchange: () => {
    const f = vfile.files[0]; if (!f) return;
    videoName = f.name; video.src = URL.createObjectURL(f); video.style.display = ""; grabRow.style.display = "";
  } });
  const grab = async () => {
    const c = document.createElement("canvas"); c.width = video.videoWidth; c.height = video.videoHeight;
    c.getContext("2d").drawImage(video, 0, 0);
    const t = video.currentTime, mm = String(Math.floor(t / 60)).padStart(2, "0"), ss = String(Math.floor(t % 60)).padStart(2, "0");
    await add(`${videoName}-${mm}${ss}.png`, c.toDataURL("image/png"), `frame ${mm}:${ss} of ${videoName}`);
  };
  const seek = (t) => new Promise((ok) => { video.onseeked = () => ok(); video.currentTime = t; });
  const grabRow = el("div", { class: "inline", style: "display:none;margin-top:10px" },
    el("button", { class: "btn secondary", id: "ref-grab", onclick: async () => { await grab(); viewReferences(); } }, "Grab This Frame"),
    el("button", { class: "btn secondary", id: "ref-grab8", onclick: async (ev) => {
      ev.target.disabled = true; video.pause();
      for (let i = 1; i <= 8; i++) { await seek((video.duration * i) / 9); await grab(); }
      viewReferences();
    } }, "Grab 8 Frames Across the Video"));
  const grid = el("div", { class: "ref-grid" }, refs.slice().reverse().map((r) => {
    const img = el("img", { alt: r.note || r.file });
    invoke("refs_image", { project, file: r.file }).then((d) => (img.src = d)).catch(() => {});
    return el("div", { class: "card ref" }, img,
      el("div", { class: "inline", style: "justify-content:space-between;margin-top:8px" }, el("span", { class: "pill" }, r.category),
        el("button", { class: "link", onclick: async () => { await invoke("refs_remove", { project, file: r.file }); viewReferences(); } }, "Remove")),
      r.note ? el("div", { class: "hint", style: "margin-top:6px" }, r.note) : null,
      r.source ? el("div", { class: "note", style: "margin-top:4px" }, r.source) : null);
  }));
  showIf(nav, page(head("References", el("span", {}, `Images and video frames the team uses for look and ideas · ${refs.length} for this game · `, runningSubtitle()), pick),
    el("div", { class: "card", id: "ref-drop" },
      el("div", { class: "inline", style: "gap:12px;align-items:flex-start;flex-wrap:wrap" },
        el("div", { class: "field", style: "width:200px" }, el("label", { for: "ref-cat" }, "What it is for"), cat),
        el("div", { class: "field grow" }, el("label", { for: "ref-note" }, "What to take from it"), note)),
      el("div", { class: "inline", style: "gap:10px;flex-wrap:wrap" },
        el("button", { class: "btn primary", onclick: () => files.click() }, "Add Images"),
        el("button", { class: "btn secondary", onclick: () => vfile.click() }, "Add a Video"),
        el("span", { class: "note" }, "Or paste an image anywhere on this page (Ctrl+V). Everything goes into the vault (19-Project-Memory) and the game's references folder.")),
      files, vfile, el("div", { style: "margin-top:12px" }, video, grabRow)),
    refs.length ? grid : el("div", { class: "empty", style: "padding:40px 20px" }, el("div", { class: "empty-title" }, "No references yet"), el("div", {}, "Paste a screenshot, add images, or grab frames from a video. Modeler, Architect, Frame, Scenic, Motion and Director look at them before they work."))));
  const pg = $("view").firstChild;
  if (nav === navSeq && pg) dropZone(pg, async (fs) => { for (const f of fs) await add(f.name, await readFile(f), `dropped ${f.name}`); viewReferences(); });
}
document.addEventListener("paste", async (e) => {
  if ((location.hash || "") !== "#references") return;
  const items = [...(e.clipboardData || {}).items || []].filter((i) => i.type.startsWith("image/"));
  if (!items.length) return;
  e.preventDefault();
  for (const it of items) {
    const f = it.getAsFile();
    const data = await new Promise((ok) => { const r = new FileReader(); r.onload = () => ok(r.result); r.readAsDataURL(f); });
    try { await invoke("refs_add", { project: state.project, name: "pasted.png", data, category: ($("ref-cat") || {}).value || "Other", note: ($("ref-note") || {}).value || "", source: "pasted" }); toast("Pasted image added to the vault."); }
    catch (err) { toast(String(err), true); }
  }
  viewReferences();
});

// =====================================================================
// 1. Review queue
// =====================================================================
async function viewQueue() {
  const nav = navSeq;
  await freshStatus();
  let items = [];
  let err = null;
  try {
    items = await invoke("queue_list");
  } catch (e) {
    err = String(e);
  }
  const f = state.queueFilter;
  const shown = items.filter((i) => (f === "pending" ? i.status === "pending" : f === "decided" ? i.status !== "pending" : true));
  const right = el("div", { class: "inline" },
    segmented([["pending", "Pending"], ["decided", "Decided"], ["all", "All"]], f, (v) => { state.queueFilter = v; viewQueue(); }),
    el("button", { class: "btn primary", id: "new-run", onclick: () => { state.runsTab = "new"; location.hash = "#runs"; } }, "New Run")
  );
  let body;
  if (err) body = el("div", { class: "card" }, err);
  else if (!shown.length)
    body = el("div", { class: "empty", id: "queue-empty" },
      el("div", { class: "ic-big", html: ICONS.tray }),
      el("div", { class: "empty-title" }, f === "pending" ? "Nothing to review" : "Nothing here yet"),
      el("div", {}, f === "pending" ? "When an agent finishes something, it waits here for your yes or no." : "Decisions you make will show up here.")
    );
  else body = el("div", { class: "qcards" }, await Promise.all(shown.map(queueCard)));
  showIf(nav, page(head("Review Queue", runningSubtitle(), right), body));
}

// An image that shows at once as a placeholder and fades in when loaded,
// so a page never waits for its pictures.
function lazyImg(path, attrs = {}) {
  const img = el("img", { alt: attrs.alt || "", class: "lazy" + (attrs.class ? " " + attrs.class : "") });
  invoke("artifact_image", { path }).then((src) => { img.src = src; img.onload = () => img.classList.add("in"); }).catch(() => img.classList.add("in"));
  return img;
}
function thumbFor(item) {
  const img = (item.artifacts || []).find((a) => /\.(svg|png|jpe?g)$/.test(a));
  if (img) return el("div", { class: "thumb" }, lazyImg(img));
  return el("div", { class: "thumb", html: ICONS[KIND_ICON[item.kind] || "doc"] });
}

function statusPill(item) {
  if (item.status === "approved") return el("span", { class: "pill white" }, "Approved");
  if (item.status === "rejected") return el("span", { class: "pill" }, "Rejected");
  if (item.status === "changes_requested") return el("span", { class: "pill" }, "Changes requested");
  return item.gate.passed ? el("span", { class: "pill red-soft" }, "Pending") : el("span", { class: "pill red" }, "Gate failed");
}

async function queueCard(item) {
  const by = item.created_by.startsWith("pipeline:") ? `Pipeline ${item.created_by.slice(9)}` : cap(item.created_by);
  const sentences = (item.summary || "").split(/(?<=\.)\s/);
  const firstLine = sentences.find((x) => x.startsWith("Stages:")) || sentences[0];
  const actions = item.status === "pending"
    ? el("div", { class: "qactions" },
        el("button", { class: "btn primary", disabled: !item.gate.passed, title: item.gate.passed ? "Record your approval. Nothing is published." : "A failed gate cannot be approved. Reject it and rerun.", onclick: () => decide(item, "approved") }, "Approve"),
        el("button", { class: "btn secondary", onclick: () => itemSheet(item, true) }, "Request Changes"),
        el("button", { class: "btn secondary", onclick: () => decide(item, "rejected") }, "Reject"))
    : null;
  return el("div", { class: "card qcard", "data-id": item.id },
    await thumbFor(item),
    el("div", { class: "qmain", onclick: () => itemSheet(item) },
      el("div", { class: "qtitle" }, item.title),
      el("div", { class: "qsum" }, firstLine || "No summary."),
      el("div", { class: "meta" }, statusPill(item), el("span", {}, by), el("span", {}, "·"), el("span", {}, ago(item.created_at)),
        el("span", {}, "·"), el("span", {}, `${item.gate.checks.filter((c) => c.passed).length}/${item.gate.checks.length} checks`))
    ),
    actions
  );
}

async function decide(item, verdict, note = "") {
  if (state.deciding) return; // a second click while the first is saving
  state.deciding = true;
  try {
    await invoke("queue_decide", { id: item.id, verdict, note });
    toast(verdict === "approved" ? "Approved. Nothing was published." : "Rejected.");
    closeSheet();
    viewQueue();
  } catch (e) {
    toast(String(e), true);
  } finally {
    state.deciding = false;
  }
}

function checksList(checks) {
  return el("ul", { class: "checks" }, checks.map((c) =>
    el("li", { class: c.passed ? "" : "fail" }, el("span", { class: "mk" }, c.passed ? "✓" : "✕"), el("span", {}, c.name, c.detail ? el("span", { class: "det" }, " · " + c.detail.slice(0, 220)) : ""))
  ));
}

async function exportPng(path, dataUrl) {
  const img = new Image();
  img.src = dataUrl;
  await img.decode();
  const c = document.createElement("canvas");
  c.width = img.naturalWidth || 1920;
  c.height = img.naturalHeight || 1080;
  c.getContext("2d").drawImage(img, 0, 0, c.width, c.height);
  const out = await invoke("export_png", { svgPath: path, pngBase64: c.toDataURL("image/png") });
  toast("Saved " + out);
}

async function artifactsBlock(artifacts) {
  const images = artifacts.filter((a) => /\.(svg|png|jpe?g)$/.test(a));
  const files = artifacts.filter((a) => !images.includes(a));
  const grid = images.length
    ? el("div", { class: "tgrid" }, images.map((p) => {
        const im = lazyImg(p, { alt: p });
        return el("div", { class: "tcell" }, im,
          el("div", { class: "tfoot" }, el("span", {}, p.split("/").pop()), p.endsWith(".svg") ? el("button", { class: "link", onclick: () => exportPng(p, im.src).catch((e) => toast(String(e), true)) }, "Export PNG") : null));
      }))
    : null;
  return el("div", {},
    el("div", { class: "files" }, files.map((p) => el("button", { class: "link", onclick: () => openText(p) }, p.split("/").slice(-2).join("/")))),
    grid);
}

// ForgeGUI sound sheet: copy each prompt, paste back the Roblox asset ID.
async function soundsBlock(artifacts) {
  const sheetPath = artifacts.find((a) => a.endsWith("/sounds.json"));
  if (!sheetPath) return null;
  const project = sheetPath.replace(/\/sounds\.json$/, "");
  let data;
  try {
    data = await invoke("sounds_get", { project });
  } catch (e) {
    return el("p", { class: "note" }, "Could not read the sound sheet: " + e);
  }
  const sounds = data.sounds || [];
  const done = () => sounds.filter((x) => x.asset_id).length;
  const count = el("span", { class: "note", id: "sounds-count" }, `${done()} of ${sounds.length} have a Roblox asset ID`);
  const rows = sounds.map((x) => {
    const asset = el("input", { type: "text", placeholder: "rbxassetid://… or the number", value: x.asset_id || "" });
    const lic = el("input", { type: "text", placeholder: "License (who made it, terms)", value: x.license || "" });
    const copy = async () => {
      try {
        await navigator.clipboard.writeText(x.prompt);
        toast("Prompt copied. Paste it into ForgeGUI.");
      } catch {
        asset.value = asset.value; // keep focus behaviour simple
        sheet("Prompt for " + x.id, el("pre", { class: "log" }, x.prompt));
      }
    };
    const save = async () => {
      try {
        const next = await invoke("sounds_set_asset", { project, id: x.id, assetId: asset.value, license: lic.value });
        const updated = next.sounds.find((y) => y.id === x.id);
        x.asset_id = updated.asset_id;
        x.license = updated.license;
        asset.value = x.asset_id || "";
        count.textContent = `${done()} of ${sounds.length} have a Roblox asset ID`;
        toast(x.asset_id ? `Saved ${x.id}.` : `Cleared ${x.id}.`);
      } catch (e) {
        toast(String(e), true);
      }
    };
    return el("div", { class: "srow", style: "align-items:flex-start", "data-sound": x.id },
      el("div", { class: "grow" },
        el("div", { class: "inline", style: "gap:8px" }, el("span", { class: "label" }, x.id), x.first_60_seconds ? el("span", { class: "pill red-soft" }, "First 60s") : null, x.loop ? el("span", { class: "pill" }, "Loop") : null),
        el("div", { class: "hint" }, x.event),
        el("div", { class: "note", style: "margin-top:4px;color:#D1D1D6" }, x.prompt),
        el("div", { class: "inline", style: "margin-top:8px" }, asset, lic, el("button", { class: "btn secondary small", onclick: save }, "Save"))),
      el("button", { class: "btn secondary small", onclick: copy }, "Copy Prompt"));
  });
  return el("div", { class: "section", style: "margin-top:18px" },
    el("div", { class: "inline", style: "justify-content:space-between" }, el("h2", { style: "margin:0" }, "Sounds for ForgeGUI"), count),
    el("p", { class: "note" }, "Copy a prompt, make the sound in ForgeGUI, upload it to Roblox, then paste its asset ID here. Builder only wires sounds that have an ID."),
    el("div", { class: "list", id: "sounds-list" }, rows));
}

async function openText(path) {
  try {
    const t = await invoke("artifact_text", { path });
    sheet(path, path.endsWith(".md") ? markdown(t) : el("pre", { class: "log" }, t));
  } catch (e) {
    toast(String(e), true);
  }
}

async function requestChanges(item, note) {
  if (!note.trim()) { toast("Write what to change in the note first.", true); return; }
  try {
    const spec = await invoke("queue_request_changes", { id: item.id, note });
    closeSheet();
    state.composer = { ...state.composer, ...spec };
    state.runsTab = "new";
    location.hash = "#runs";
    toast("Changes recorded. Check the cost, then approve the redo.");
    setTimeout(() => { const b = $("estimate-btn"); if (b) b.click(); }, 400);
  } catch (e) { toast(String(e), true); }
}

async function itemSheet(item, focusNote = false) {
  const note = el("input", { type: "text", id: "decision-note", placeholder: focusNote ? "What should change? Be specific." : "Note: why yes or no, or what to change" });
  if (focusNote) setTimeout(() => note.focus(), 50);
  sheet(item.title,
    el("div", { class: "meta" }, statusPill(item), el("span", {}, item.created_by), el("span", {}, "·"), el("span", {}, item.created_at)),
    markdown(item.summary || ""),
    el("h2", { style: "margin-top:18px" }, "What was produced"),
    await artifactsBlock(item.artifacts || []),
    await soundsBlock(item.artifacts || []),
    el("h2", { style: "margin-top:18px" }, "How it was verified"),
    checksList(item.gate.checks),
    item.run_id ? el("p", {}, el("button", { class: "link", onclick: () => { location.hash = "#run/" + item.run_id; } }, "Open the run")) : null,
    item.status === "pending"
      ? el("div", { class: "inline", style: "margin-top:16px" }, note,
          el("button", { class: "btn primary", disabled: !item.gate.passed, onclick: () => decide(item, "approved", note.value) }, "Approve"),
          el("button", { class: "btn secondary", id: "request-changes", disabled: !item.run_id, onclick: () => requestChanges(item, note.value) }, "Request Changes"),
          el("button", { class: "btn secondary", onclick: () => decide(item, "rejected", note.value) }, "Reject"))
      : item.decision ? el("p", { class: "note" }, `${cap(item.decision.verdict)} by ${item.decision.by} at ${item.decision.at}${item.decision.note ? ` · "${item.decision.note}"` : ""}`) : null
  );
}

// =====================================================================
// 2. Agents
// =====================================================================
const AGENT_STATUS = {
  idle: () => el("span", { class: "pill" }, "Idle"),
  working: () => el("span", { class: "pill red" }, "Working"),
  // Finished work waiting in the Review Queue reads differently from a run
  // that needs your yes before it starts.
  waiting: (a) => el("span", { class: "pill red-soft", title: a && a.detail || "" }, a && /review queue/.test(a.detail || "") ? "Ready for review" : "Needs your approval"),
  blocked: () => el("span", { class: "pill white" }, "Blocked"),
};
async function viewAgents() {
  const nav = navSeq;
  await freshStatus();
  const agents = await invoke("agents_status");
  const working = agents.filter((a) => a.status === "working").length;
  showIf(nav, page(
    head("Agents", el("span", {}, `${agents.length} agents · ${working ? working + " working" : "none working"} · `, runningSubtitle())),
    el("div", { class: "list", id: "agent-list" }, agents.map((a) =>
      el("div", { class: "row tap" + (a.status === "working" ? " active-agent" : ""), "data-agent": a.name, onclick: () => agentSheet(a) },
        el("div", { class: "grow" }, el("div", { class: "row-title" }, cap(a.name), a.role ? el("span", { class: "role" }, a.role) : null), el("div", { class: "row-sub" }, a.description)),
        AGENT_STATUS[a.status](a),
        el("span", { class: "chev", html: ICONS.chev })
      )
    ))
  ));
}
function agentSheet(a) {
  sheet(cap(a.name),
    el("p", { class: "prose" }, a.description),
    el("div", { class: "list" },
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Status"), AGENT_STATUS[a.status](a)),
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Right now"), el("span", { class: "note" }, a.detail)),
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Model"), el("span", { class: "note" }, a.model)),
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Writes"), el("span", { class: "note" }, a.output || "–")),
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Vault sections it may read"), el("span", { class: "note" }, a.sections.join(", ")))
    ),
    el("div", { class: "inline", style: "margin-top:16px" },
      el("button", { class: "btn primary", onclick: () => { state.composer.mode = "agent"; state.composer.agent = a.name; state.runsTab = "new"; location.hash = "#runs"; } }, `Run ${cap(a.name)}`),
      a.last_run ? el("button", { class: "btn secondary", onclick: () => { location.hash = "#run/" + a.last_run; } }, "Last Run") : null)
  );
}

// =====================================================================
// 3. Vault
// =====================================================================
function inline(text) {
  const frag = document.createDocumentFragment();
  const re = /(`[^`]+`|\*\*[^*]+\*\*|\[\[[^\]]+\]\])/g;
  let last = 0;
  for (const m of text.matchAll(re)) {
    if (m.index > last) frag.append(text.slice(last, m.index));
    const t = m[0];
    if (t.startsWith("`")) frag.append(el("code", {}, t.slice(1, -1)));
    else if (t.startsWith("**")) frag.append(el("strong", {}, t.slice(2, -2)));
    else {
      const target = t.slice(2, -2).split("|")[0];
      frag.append(el("span", { class: "wl", onclick: () => openNote(target + ".md") }, target.split("/").pop()));
    }
    last = m.index + t.length;
  }
  frag.append(text.slice(last));
  return frag;
}
function markdown(src) {
  const out = el("div", { class: "prose" });
  let body = src;
  if (src.startsWith("---\n")) {
    const end = src.indexOf("\n---\n", 4);
    if (end > 0) {
      const fm = src.slice(4, end).split("\n").filter((l) => /^(status|last_checked|verify):/.test(l));
      out.append(el("div", { class: "fm" }, fm.map((l) => el("div", {}, l))));
      body = src.slice(end + 5);
    }
  }
  const lines = body.split("\n");
  let i = 0;
  while (i < lines.length) {
    const l = lines[i];
    if (l.startsWith("```")) {
      const code = [];
      i++;
      while (i < lines.length && !lines[i].startsWith("```")) code.push(lines[i++]);
      i++;
      out.append(el("pre", {}, el("code", {}, code.join("\n"))));
      continue;
    }
    if (/^#{1,3} /.test(l)) {
      const n = l.match(/^#+/)[0].length;
      out.append(el("h" + n, {}, inline(l.slice(n + 1))));
      i++;
      continue;
    }
    if (l.startsWith("|")) {
      const rows = [];
      while (i < lines.length && lines[i].startsWith("|")) rows.push(lines[i++]);
      const cells = (r) => r.trim().replace(/^\||\|$/g, "").split("|").map((c) => c.trim());
      const tbl = el("table", {});
      rows.forEach((r, k) => {
        if (/^\|[\s:|-]+\|$/.test(r.trim())) return;
        tbl.append(el("tr", {}, cells(r).map((c) => el(k === 0 ? "th" : "td", {}, inline(c)))));
      });
      out.append(tbl);
      continue;
    }
    if (/^\s*(- |\d+\. )/.test(l)) {
      const ordered = /^\s*\d+\. /.test(l);
      const list = el(ordered ? "ol" : "ul");
      while (i < lines.length && /^\s*(- |\d+\. )/.test(lines[i])) list.append(el("li", {}, inline(lines[i++].replace(/^\s*(- \[[ x]\] |- |\d+\. )/, ""))));
      out.append(list);
      continue;
    }
    if (l.trim()) out.append(el("p", {}, inline(l)));
    i++;
  }
  return out;
}
async function openNote(path) {
  try {
    sheet(path, markdown(await invoke("vault_note", { path })));
  } catch (e) {
    toast(String(e), true);
  }
}
// Free CC0 textures from Poly Haven into the vault.
function texturePanel(tex) {
  const boxes = tex.groups.map((g) => el("label", { class: "chip" }, el("input", { type: "checkbox", value: g, checked: true }), ` ${g}`));
  const per = el("select", { id: "tex-per" }, [3, 5, 8, 12].map((n) => el("option", { value: n }, `${n} each`)));
  per.value = "5";
  const status = el("div", { class: "hint", id: "tex-status" }, `${tex.count} textures in the vault (10-Art-Environment/texture-library). Free CC0 colour, normal, roughness and metalness maps.`);
  const go = el("button", { class: "btn primary", id: "tex-download", onclick: async () => {
    const groups = boxes.map((b) => b.querySelector("input")).filter((i) => i.checked).map((i) => i.value);
    if (!groups.length) { toast("Pick at least one kind.", true); return; }
    go.disabled = true; status.textContent = "Starting...";
    try { await invoke("textures_download", { groups, perGroup: parseInt(per.value, 10) }); } catch (e) { go.disabled = false; toast(String(e), true); }
  } }, "Download Textures");
  return el("details", { class: "card tex-panel" }, el("summary", { class: "row-title" }, `Texture library · ${tex.count} textures · Powered by Poly Haven`),
    status, el("div", { class: "chips" }, boxes), el("div", { class: "inline", style: "margin-top:10px" }, per, go));
}
T.event.listen("tex-line", (ev) => { const n = $("tex-status"); if (n) n.textContent = `Downloading ${ev.payload.line}`; });
T.event.listen("tex-done", (ev) => {
  if (ev.payload.error) toast(ev.payload.error, true); else toast(ev.payload.message);
  if ((location.hash || "") === "#vault") viewVault();
});

// Autopilot: Andrew approves an idea once (with a budget for the whole game);
// the agents make it milestone by milestone. He watches, chimes in, and only
// approves finished milestones.
async function startAutopilot(title, idea, slug) {
  let est;
  try { est = await invoke("autopilot_estimate", { idea }); } catch (e) { toast(String(e), true); return; }
  const name = el("input", { type: "text", id: "ap-title", value: title || "" });
  const go = el("button", { class: "btn primary", id: "ap-go" }, "Approve and Start");
  go.addEventListener("click", async () => {
    go.disabled = true;
    try {
      const r = await invoke("autopilot_start", { title: name.value.trim() || "My game", idea, budgetUsd: 0, slug: slug || null });
      closeSheet(); toast("Approved. The agents are starting on the plan."); setGame(r.slug, false); location.hash = "#autopilot/" + r.slug;
    } catch (e) { toast(String(e), true); go.disabled = false; }
  });
  sheet("Make this game",
    el("p", { class: "prose" }, "The agents make the whole game from here, milestone by milestone, and keep going on their own: failed checks are retried, and agents call in other agents when they need them. You only approve finished milestones, and you can send notes any time. Nothing is ever published."),
    el("div", { class: "field" }, el("label", { for: "ap-title" }, "Game name"), name),
    el("div", { class: "list" }, est.milestones.map((m) => el("div", { class: "srow" }, el("div", { class: "grow label" }, m.title), el("span", { class: "note" }, `about ${usd(m.cap_usd)}`)))),
    el("div", { class: "inline", style: "margin-top:12px" }, go, el("span", { class: "note" }, "Approving starts the agents.")));
}
const AP_STATUS = { running: "Running", paused: "Paused by you", budget: "Stopped at the budget", done: "Done", stopped: "Stopped" };
const MS_PILL = (st) => el("span", { class: st === "working" || st === "revising" ? "pill red" : st === "review" ? "pill white" : st === "approved" ? "pill white" : "pill" }, ({ waiting: "Waiting", working: "Agents working", revising: "Revising from your notes", review: "Ready for your review", approved: "Approved" })[st] || cap(st));
function jobRow(j) {
  return el("div", { class: "row ap-job", "data-run": j.run_id },
    el("div", { class: "grow" },
      el("div", { class: "row-title" }, `${cap(j.agent)}${j.source.startsWith("agent:") ? " (asked by " + cap(j.source.slice(6)) + ")" : j.source === "andrew" ? " (your note)" : ""}`),
      el("div", { class: "row-sub" }, j.status === "working" ? el("span", { class: "ap-live", id: "live-" + j.run_id }, state.apLive[j.run_id] || "Working…") : (j.summary || j.text)),
      j.attempt ? el("div", { class: "note" }, `Try ${j.attempt + 1} of 4${j.attempt >= 3 ? " (stronger model)" : ""}${j.rescuing ? "; Lead is working out a new approach" : ""}`) : null),
    el("span", { class: j.stuck ? "pill red-soft" : j.status === "working" ? "pill red" : j.status === "done" ? "pill white" : "pill" },
      j.rescuing ? "Lead is finding a fix" : j.launch_failed || (j.status === "queued" && j.retry_at) ? `Retrying ${j.retry_at ? "at " + new Date(j.retry_at).toLocaleTimeString([], { hour: "numeric", minute: "2-digit" }) : "soon"}` : ({ queued: "Waiting", working: "Working", paused: "Paused", done: "Done", failed: "Retrying" })[j.status] || cap(j.status)),
    ...agentControls(j.status, j.run_id),
    j.output ? el("button", { class: "link", onclick: () => openText(j.output) }, "Open") : null,
    el("button", { class: "link", onclick: () => (location.hash = "#run/" + j.run_id) }, "Details"));
}
// Pause / Stop for an agent that is working or waiting; Resume for a paused one.
function agentControls(status, runId) {
  const act = (label, fn) => el("button", { class: "link", onclick: async (e) => { e.stopPropagation(); e.currentTarget.disabled = true; try { await fn(); } catch (err) { toast(String(err), true); e.currentTarget.disabled = false; } } }, label);
  const stop = (pause) => async () => { await invoke("agent_stop", { id: runId, pause }); toast(pause ? "Pausing. Its files are kept." : "Stopped. Its files are kept."); };
  if (status === "working" || status === "queued" || status === "running" || status === "approved") return [act("Pause", stop(true)), act("Stop", stop(false))];
  if (status === "paused") return [act("Resume", async () => { await invoke("continue_now", { id: runId }); toast("Continuing where it left off."); })];
  return [];
}
async function viewAutopilot(slug) {
  const nav = navSeq;
  await freshStatus();
  state.apLive ||= {};
  const games = await invoke("autopilot_list");
  if (slug && slug !== "new" && slug !== state.game) setGame(slug, false);
  const cur = slug === "new" ? null : games.find((g) => g.slug === (slug || state.game));
  if (!cur) {
    // No autopilot for this game yet: one box for the idea, one button.
    const named = slug === "new" ? "" : ((await gameList()).find((g) => g.slug === state.game) || {}).title || "";
    const t = el("input", { type: "text", id: "ap-new-title", placeholder: "Game name", value: named });
    const idea = el("textarea", { id: "ap-new-idea", rows: 4, placeholder: "Your idea, in your own words. A sentence is enough; the agents work out the rest and you approve each finished part." });
    return showIf(nav, page(head("Make a game", "Say the idea once. The agents make it; you chime in whenever you like."),
      el("div", { class: "card" }, el("div", { class: "field" }, el("label", { for: "ap-new-title" }, "Game name"), t), idea,
        el("div", { class: "inline", style: "margin-top:10px" }, el("button", { class: "btn primary", id: "ap-new", onclick: () => startAutopilot(t.value.trim(), idea.value.trim(), named && slug !== "new" ? state.game : null) }, "Make This Game")))));
  }
  let v;
  try { v = await invoke("autopilot_view", { slug: cur.slug }); } catch (e) { return showIf(nav, page(head("Autopilot"), el("div", { class: "card" }, String(e)))); }
  const ctl = (action, label, extra, cls = "btn secondary") => el("button", { class: cls, id: "ap-" + action, onclick: async () => { try { await invoke("autopilot_control", { slug: v.slug, action, budgetUsd: extra ? extra() : null }); viewAutopilot(v.slug); } catch (e) { toast(String(e), true); } } }, label);
  // Chime in: the main thing on this page.
  const chime = el("textarea", { id: "ap-note", rows: 3, placeholder: "Chime in: anything you want changed, added or fixed. Enter sends. The team works out the rest." });
  chime.value = state.apNote || "";
  chime.addEventListener("input", () => { state.apNote = chime.value; });
  const sendNote = async () => { const t = chime.value.trim(); if (!t) return; try { await invoke("autopilot_note", { slug: v.slug, text: t, agent: null }); state.apNote = ""; chime.value = ""; toast("Sent to the team."); viewAutopilot(v.slug); } catch (e) { toast(String(e), true); } };
  chime.addEventListener("keydown", (e) => { if (e.key === "Enter" && !e.shiftKey) { e.preventDefault(); sendNote(); } });
  const reviews = v.milestones.filter((m) => m.status === "review").map((m) => {
    const notes = el("textarea", { class: "ap-review-notes", rows: 2, placeholder: "Anything to change? Leave empty to approve." });
    const drafts = m.jobs.filter((j) => j.status === "done");
    const qs = [...new Map(drafts.flatMap((j) => (j.questions || []).map((q) => [q.toLowerCase(), { q, who: j.agent }]))).values()].slice(0, 10);
    return el("div", { class: "card ap-review", "data-milestone": m.id },
      el("h2", { style: "margin:0 0 4px" }, `Ready for you: ${m.title}`),
      el("p", { class: "note", style: "margin:0 0 8px" }, m.review),
      el("div", { class: "inline", style: "flex-wrap:wrap" },
        m.id === "plan" && v.plan_run ? el("button", { class: "btn secondary", onclick: () => (location.hash = "#plans/" + v.plan_run) }, "Open the Plan") : null,
        m.id === "build" && v.project ? el("button", { class: "btn secondary", onclick: async () => { try { const pth = await invoke("project_open_in_studio", { project: v.project }); toast(`Opened in Studio: ${pth.split(/[\\/]/).pop()}. Nothing was published.`); } catch (e) { toast(String(e), true); } } }, "Play It in Roblox Studio") : null),
      el("div", { class: "ap-drafts" }, drafts.map((j) => el("div", { class: "ap-draft" },
        el("div", { class: "row-title" }, cap(j.agent)), el("div", { class: "note" }, j.summary || ""),
        j.mockups.length ? el("div", { class: "ap-mocks" }, j.mockups.slice(0, 6).map((mk) => el("button", { class: "ap-mock", onclick: async () => sheet(mk.split("/").pop(), lazyImg(mk, { class: "plan-mock" })) }, lazyImg(mk)))) : null,
        j.output ? el("button", { class: "link", onclick: () => openText(j.output) }, "Read it") : null))),
      qs.length ? el("div", { class: "ap-questions" }, el("div", { class: "row-title" }, "Questions for you"), el("div", { class: "note" }, "Each already has a safe default. Answer only the ones you care about."),
        el("ul", {}, qs.map((x) => el("li", {}, x.q, el("span", { class: "note" }, ` (${cap(x.who)})`))))) : null,
      notes,
      el("div", { class: "inline", style: "margin-top:8px" },
        el("button", { class: "btn primary ap-approve", onclick: async (e) => { e.currentTarget.disabled = true; try { await invoke("autopilot_review", { slug: v.slug, milestone: m.id, approve: !notes.value.trim(), notes: notes.value }); toast(notes.value.trim() ? "Sent. The agents are revising it." : `Approved: ${m.title}.`); } catch (er) { toast(String(er), true); } viewAutopilot(v.slug); } }, "Approve"),
        el("button", { class: "btn secondary ap-reject", onclick: async (e) => { e.currentTarget.disabled = true; try { await invoke("autopilot_review", { slug: v.slug, milestone: m.id, approve: false, notes: notes.value }); toast("Sent back. The team keeps improving it."); } catch (er) { toast(String(er), true); } viewAutopilot(v.slug); } }, "Not Good Enough"),
        el("span", { class: "note" }, "Not Good Enough sends it back to keep improving (your notes, if any, go with it).")));
  });
  // Progress: six steps in a line.
  const steps = el("div", { class: "ap-steps" }, v.milestones.map((m) => el("div", { class: "ap-step " + m.status, title: m.title }, el("span", { class: "dot" }), el("span", { class: "lbl" }, m.title))));
  const active = v.milestones.flatMap((m) => m.jobs).filter((j) => ["working", "queued", "paused"].includes(j.status) || j.rescuing);
  const nowM = v.milestones.find((m) => m.status === "working" || m.status === "revising");
  showIf(nav, page(
    head(v.title, el("span", {}, `${({ running: "Being made", paused: "Paused", budget: "Waiting for more budget", done: "Done" })[v.status] || v.status}${nowM ? " · " + nowM.title : ""}`)),
    v.status === "paused" ? el("div", { class: "card warn-card" }, el("div", { style: "white-space:pre-wrap" }, v.why || "Paused. Nothing new starts until you resume."), el("div", { style: "margin-top:8px" }, ctl("resume", "Resume", null, "btn primary"))) : null,
    v.status === "running" && active.some((j) => j.status === "paused") ? el("div", { class: "card paused-card" },
      el("div", { class: "row-title" }, "Claude's usage ran out"),
      el("p", { class: "note" }, "The team picks up by itself when the limit resets. Or keep going now for free on this computer: slower and weaker, and Claude takes over new steps again after the reset."),
      el("button", { class: "btn primary", onclick: (e) => continueLocal(null, e.currentTarget) }, "Continue on Local Model")) : null,
    el("div", { class: "card ap-chime" }, chime, el("div", { class: "inline", style: "margin-top:8px" }, el("button", { class: "btn primary", id: "ap-send", onclick: sendNote }, "Send"), el("span", { class: "note" }, "Problems are handled by the team: failed work is retried and fixed on its own."))),
    ...reviews,
    steps,
    el("div", { class: "section" }, el("div", { class: "section-label" }, active.length ? `Working now (${active.filter((j) => j.status === "working").length} agents)` : "Working now"),
      active.length ? el("div", { class: "list" }, active.map(jobRow)) : el("div", { class: "note" }, v.status === "done" ? "Every part is approved." : "Waiting for your review above, or starting the next part.")),
    el("details", { class: "ap-more" }, el("summary", { class: "link" }, "Everything the team did"),
      el("div", { class: "inline", style: "margin:10px 0" }, el("span", { class: "note" }, `Plan usage so far: ${usd(v.spent_usd, 2)}`), v.status === "running" ? ctl("pause", "Pause") : v.status === "paused" ? ctl("resume", "Resume") : null),
      ...v.milestones.map((m) => el("details", { class: "card ap-ms", "data-ms": m.id },
        el("summary", {}, el("span", { class: "row-title" }, m.title), MS_PILL(m.status)),
        m.jobs.length ? el("div", { class: "list" }, m.jobs.map(jobRow)) : el("div", { class: "note" }, "Not started yet."))),
      el("div", { class: "section-label", style: "margin-top:14px" }, "Team board"),
      el("div", { class: "list ap-board" }, v.board.map((b) => el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, `${cap(b.from)}${b.to ? " → " + cap(b.to) : ""}`), el("div", { class: "note" }, b.text)), el("span", { class: "note" }, ago(b.at))))))));
}
T.event.listen("run-line", (ev) => {
  const t = renderLine(ev.payload.line);
  if (!t) return;
  const last = t.split("\n").filter(Boolean).pop() || "";
  (state.apLive ||= {})[ev.payload.run_id] = last.slice(0, 160);
  const n = $("live-" + ev.payload.run_id);
  if (n) n.textContent = last.slice(0, 160);
});

// Change requests: write what to change, press Send, and each request goes
// to the agent that fits it. Up to three agents work at once.
const CHANGE_AGENTS = ["auto", "builder", "frame", "scenic", "motion", "modeler", "librarian", "merchant", "tuner", "crowd", "splash", "architect", "planner", "liveops"];
const changeLabel = (c) => ({ queued: `Waiting for ${cap(c.agent)}`, working: `${cap(c.agent)} is on it`, paused: "Paused", cancelled: "Stopped", done: "Done", failed: "Didn't finish" })[c.status] || cap(c.status);
const changePill = (c) => el("span", { class: c.status === "working" ? "pill red" : c.status === "failed" ? "pill red-soft" : c.status === "done" ? "pill white" : "pill" }, changeLabel(c));
// Confirm sheet: who does each request and what it may cost. Send approves.
async function sendChanges(target, reqs, after) {
  reqs = reqs.filter((r) => r.text && r.text.trim());
  if (!reqs.length) { toast("Write a change request first.", true); return; }
  const rows = reqs.map((r) => ({ ...r, agent: r.agent || "auto" }));
  const list = el("div", { class: "list" });
  const total = el("div", { class: "hint" });
  const go = el("button", { class: "btn primary", id: "changes-go" }, "Send");
  const draw = async () => {
    let pv;
    try { pv = await invoke("changes_preview", { target, requests: rows }); } catch (e) { total.textContent = String(e); go.disabled = true; return; }
    go.disabled = false;
    list.replaceChildren(...rows.map((r, i) => {
      const sel = el("select", { class: "chg-agent" }, CHANGE_AGENTS.map((a) => el("option", { value: a }, a === "auto" ? `Auto (${cap(pv.rows[i].agent)})` : cap(a))));
      sel.value = r.agent;
      sel.addEventListener("change", () => { r.agent = sel.value; draw(); });
      return el("div", { class: "srow" }, el("div", { class: "grow label" }, r.text), sel, el("span", { class: "note", style: "min-width:70px;text-align:right" }, `up to ${usd(pv.rows[i].cap_usd)}`));
    }));
    total.textContent = `${rows.length} request${rows.length > 1 ? "s" : ""}, up to ${usd(pv.total_cap_usd)} in all. Up to 3 agents work at once; each one reads its request, makes only that change, and reports back.`;
    go.textContent = `Send ${rows.length > 1 ? rows.length + " Requests" : "Request"}`;
  };
  go.addEventListener("click", async () => {
    go.disabled = true;
    try {
      const made = await invoke("changes_send", { target, requests: rows.map((r) => ({ item: r.item || null, text: r.text, agent: r.agent })) });
      closeSheet();
      toast(`Sent ${made.length} change request${made.length > 1 ? "s" : ""}. The agents are starting.`);
      if (after) after(made);
    } catch (e) { toast(String(e), true); go.disabled = false; }
  });
  sheet("Send change requests", total, list, el("div", { class: "inline", style: "margin-top:12px" }, go, el("span", { class: "note" }, "Pressing Send approves these costs.")));
  await draw();
}
function changeRow(c) {
  return el("div", { class: "row chg-row", "data-change": c.id },
    el("div", { class: "grow" },
      el("div", { class: "row-title" }, c.text),
      el("div", { class: "row-sub" }, `${cap(c.agent)}${c.source && c.source.startsWith("agent:") ? " (started by " + cap(c.source.slice(6)) + ")" : ""}${c.after ? " · after the one before" : ""} · ${c.target.replace(/^(project|game):/, "").replace(/^plan:.*/, "game plan")}${c.cost_usd != null ? " · " + usd(c.cost_usd, 2) : ""} · ${ago(c.created_at)}`),
      c.summary ? el("div", { class: "note", style: "margin-top:4px" }, c.summary) : null,
      c.status === "failed" && c.checks.length ? el("div", { class: "note" }, c.checks.filter((k) => !k.passed).map((k) => `${k.name}${k.detail ? ": " + k.detail.slice(0, 160) : ""}`).join(" · ")) : null),
    changePill(c),
    ...agentControls(c.status, c.run_id),
    el("button", { class: "link", onclick: () => (location.hash = "#run/" + c.run_id) }, "Details"));
}
async function viewChanges() {
  const nav = navSeq;
  await freshStatus();
  const pick = await projectPicker(viewChanges);
  const { changes } = await invoke("changes_list");
  const draft = (state.changeDraft ||= []);
  const input = el("textarea", { id: "change-input", rows: 2, placeholder: "What should change? One request per line, e.g. \"Make the shop button bigger\". Enter adds it, Shift+Enter for a new line." });
  const drafts = el("div", { class: "list", id: "change-drafts" });
  const send = el("button", { class: "btn primary", id: "change-send" });
  const drawDrafts = () => {
    drafts.replaceChildren(...draft.map((t, i) => el("div", { class: "srow" }, el("div", { class: "grow label" }, t), el("button", { class: "link", onclick: () => { draft.splice(i, 1); drawDrafts(); } }, "Remove"))));
    send.disabled = !state.project || !draft.length;
    send.textContent = draft.length ? `Send ${draft.length} Change Request${draft.length > 1 ? "s" : ""}` : "Send";
  };
  const add = () => { for (const l of input.value.split("\n").map((x) => x.trim()).filter(Boolean)) draft.push(l); input.value = ""; state.changeInput = ""; drawDrafts(); };
  input.value = state.changeInput || "";
  input.addEventListener("input", () => { state.changeInput = input.value; });
  input.addEventListener("keydown", (e) => { if (e.key === "Enter" && !e.shiftKey) { e.preventDefault(); add(); } });
  send.addEventListener("click", () => { add(); sendChanges("project:" + state.project.replace(/^projects\//, ""), draft.map((text) => ({ text })), () => { draft.length = 0; viewChanges(); }); });
  drawDrafts();
  const mine = changes.slice().reverse();
  const live = mine.filter((c) => ["queued", "working", "paused"].includes(c.status));
  const past = mine.filter((c) => !live.includes(c)).slice(0, 40);
  showIf(nav, page(
    head("Change Requests", el("span", {}, "Write what to change, press Send, and the right agents make the tweaks. Up to 3 work at once.")),
    el("div", { class: "card" }, el("div", { class: "inline", style: "margin-bottom:10px" }, el("label", { for: "project-pick" }, "Game"), pick),
      input, el("div", { class: "inline", style: "margin-top:8px" }, el("button", { class: "btn secondary", onclick: add }, "Add to List"), send), drafts),
    el("div", { class: "section" }, el("div", { class: "section-label" }, `Working now (${live.length})`),
      live.length ? el("div", { class: "list", id: "changes-live" }, live.map(changeRow)) : el("div", { class: "note" }, "Nothing in progress.")),
    past.length ? el("div", { class: "section" }, el("div", { class: "section-label" }, "Finished"), el("div", { class: "list", id: "changes-past" }, past.map(changeRow))) : null));
  input.focus();
}
// An agent started other agents (requests.json) when it finished.
T.event.listen("handoff", (ev) => {
  const d = ev.payload || {};
  if (d.started && d.started.length) toast(`Started ${d.started.map(cap).join(", ")} on their own. Watch them in Make the Game, Changes.`);
  else if (d.notes && d.notes.length) toast(d.notes[0], true);
  refreshStatus();
  if ((location.hash || "") === "#changes") viewChanges();
});
// Many updates arrive together when several agents work: redraw once.
let changeRedraw = null;
T.event.listen("change-update", (ev) => {
  const rid = ev.payload && ev.payload.run_id;
  if (rid && ev.payload.status && ev.payload.status !== "working") { delete state.logs[rid]; if (state.apLive) delete state.apLive[rid]; }
  clearTimeout(changeRedraw);
  changeRedraw = setTimeout(redrawAfterChange, 400);
});
function redrawAfterChange() {
  const h = location.hash || "";
  refreshStatus();
  // Never redraw under someone typing; the next visit shows the news.
  const a = document.activeElement;
  if (a && (a.tagName === "TEXTAREA" || a.tagName === "INPUT") && $("view").contains(a) && a.value) return;
  if (h.startsWith("#autopilot")) viewAutopilot(h.split("/")[1]);
  else if (h === "#changes") viewChanges();
  else if (h.startsWith("#plans")) viewPlans(h.split("/")[1]);
}

// Inspiration: pictures of popular games (UI, characters, worlds) with what
// to copy from each. Pictures download to this computer only.
function refPic(path, attrs = {}) {
  const img = el("img", { alt: attrs.alt || "", class: "lazy" + (attrs.class ? " " + attrs.class : "") });
  // Load only when on screen: there can be hundreds.
  const load = () => invoke("refs_picture", { path }).then((src) => { img.src = src; img.onload = () => img.classList.add("in"); }).catch(() => img.classList.add("in"));
  if ("IntersectionObserver" in window) {
    const io = new IntersectionObserver((es) => { if (es.some((e) => e.isIntersecting)) { io.disconnect(); load(); } }, { rootMargin: "300px" });
    setTimeout(() => io.observe(img), 0);
  } else load();
  return img;
}
const GROUP_SHORT = { "roblox-simulators": "Roblox Sims", "roblox-social": "Roblox Social", "roblox-action": "Roblox Action", mobile: "Mobile", nintendo: "Nintendo", "big-3d": "Big 3D", "pc-indie": "Indie" };
const coverOf = (g) => g.pictures.find((p) => !/-(icon)\./.test(p)) || g.pictures[0];
function openGame(g) {
  const L = g.learn;
  const big = el("div", { class: "ref-big" });
  const showBig = (p) => big.replaceChildren(refPic(p, { alt: g.name }));
  if (g.pictures.length) showBig(coverOf(g));
  sheet(g.name,
    g.pictures.length ? big : el("div", { class: "note" }, "No pictures yet. Press Download Pictures on the Inspiration page."),
    g.pictures.length > 1 ? el("div", { class: "ref-strip" }, g.pictures.map((p) => el("button", { class: "ref-thumb", onclick: () => showBig(p) }, refPic(p)))) : null,
    el("div", { class: "ref-learn" },
      el("p", {}, el("b", {}, "Steal this: "), L.steal),
      el("p", {}, el("b", {}, "UI: "), L.ui),
      el("p", {}, el("b", {}, "Characters and shapes: "), L.characters),
      el("p", {}, el("b", {}, "World: "), L.world)),
    el("div", { class: "note" }, "Borrow the rule, never the art. These pictures are other companies' work, kept on this computer for study."));
}
async function viewInspo() {
  const nav = navSeq;
  await freshStatus();
  let lib;
  try { lib = await invoke("refs_library"); } catch (e) { return showIf(nav, page(head("Inspiration"), el("div", { class: "card" }, String(e)))); }
  const f = (state.inspo ||= { group: "all", tag: "all", q: "" });
  const have = lib.games.filter((g) => g.pictures.length).length;
  const grid = el("div", { class: "ref-grid", id: "ref-grid" });
  const draw = () => {
    const q = f.q.trim().toLowerCase();
    const list = lib.games.filter((g) => (f.group === "all" || g.group === f.group) && (f.tag === "all" || g.tags.includes(f.tag)) && (!q || (g.name + " " + Object.values(g.learn).join(" ")).toLowerCase().includes(q)));
    grid.replaceChildren(...(list.length ? list.map((g) => el("button", { class: "ref-card", "data-game": g.id, onclick: () => openGame(g) },
      el("div", { class: "ref-cover" }, g.pictures.length ? refPic(coverOf(g), { alt: g.name }) : el("span", { class: "note" }, "No pictures yet")),
      el("div", { class: "ref-name" }, g.name),
      el("div", { class: "ref-steal" }, g.learn.steal))) : [el("div", { class: "empty" }, el("div", { class: "empty-title" }, "Nothing matches"), el("div", {}, "Try another filter or fewer words."))]));
  };
  const q = el("input", { type: "search", class: "search", id: "ref-q", placeholder: "Search games and ideas (e.g. pets, buttons, horror)", value: f.q });
  let t; q.addEventListener("input", () => { clearTimeout(t); t = setTimeout(() => { f.q = q.value; draw(); }, 120); });
  const per = el("select", { id: "ref-per" }, [4, 6, 8, 12].map((n) => el("option", { value: n }, `${n} per game`)));
  per.value = "8";
  const status = el("div", { class: "hint", id: "ref-status" }, have
    ? `${lib.total_pictures} pictures from ${have} of ${lib.games.length} games. Download again to get the rest or new games.`
    : `${lib.games.length} popular games ready. Download their pictures once (about 2 MB a game, from Roblox, Steam, the App Store and Wikipedia).`);
  const go = el("button", { class: "btn primary", id: "ref-download", disabled: !!state.refBusy, onclick: async () => {
    go.disabled = true; state.refBusy = true; status.textContent = "Starting...";
    try { await invoke("refs_download", { ids: [], perGame: parseInt(per.value, 10), refresh: false }); } catch (e) { go.disabled = false; state.refBusy = false; toast(String(e), true); }
  } }, have ? "Download Missing Pictures" : "Download Pictures");
  draw();
  showIf(nav, page(
    head("Inspiration", el("span", {}, "UI, characters and worlds from popular games, and the one thing to copy from each")),
    el("div", { class: "card" }, status, el("div", { class: "inline", style: "margin-top:10px" }, per, go,
      el("button", { class: "btn secondary", onclick: () => openNote("16-Market-Research/visual-refs-shape-language.md") }, "Shape Language Sheet"))),
    el("div", { class: "inline", style: "margin:14px 0 8px;flex-wrap:wrap" }, segmented([["all", "All"], ...lib.groups.map((g) => [g.id, GROUP_SHORT[g.id] || g.title])], f.group, (v) => { f.group = v; viewInspo(); })),
    el("div", { class: "inline", style: "margin-bottom:12px" }, segmented([["all", "Everything"], ["ui", "UI"], ["characters", "Characters"], ["world", "Worlds"], ["thumbnail", "Thumbnails"]], f.tag, (v) => { f.tag = v; viewInspo(); }), q),
    grid));
}
T.event.listen("ref-line", (ev) => { const n = $("ref-status"); if (n) n.textContent = `Downloading ${ev.payload.line}`; });
T.event.listen("ref-done", (ev) => {
  state.refBusy = false;
  toast(ev.payload.message, !ev.payload.ok);
  if ((location.hash || "") === "#inspo") viewInspo();
});

// The vault as a second-brain map: the hub in the middle, a spike to each
// section, notes fanned out at the ends. Hover a note to see its links;
// click to open it. Drawn as SVG, no libraries.
async function vaultGraph() {
  const nodes = (await invoke("vault_graph")).nodes.filter((n) => n.id.includes("/")); // root files like CLAUDE.md are not notes
  const NS = "http://www.w3.org/2000/svg";
  const mk = (tag, a) => { const n = document.createElementNS(NS, tag); for (const [k, v] of Object.entries(a)) n.setAttribute(k, v); return n; };
  const secs = [...new Set(nodes.map((n) => n.id.split("/")[0]))].filter((s) => s !== "00-Start-Here" || true).sort();
  const W = 1400, H = 1400, cx = W / 2, cy = H / 2, R1 = 230, R2 = 560;
  const colors = ["#FF453A", "#FF9F0A", "#FFD60A", "#30D158", "#64D2FF", "#0A84FF", "#5E5CE6", "#BF5AF2", "#FF375F", "#AC8E68", "#66D4CF", "#FF6482", "#8E8E93", "#32ADE6", "#A2845E", "#DA8FFF", "#63E6E2", "#FFB340", "#7D7AFF", "#34C759"];
  const pos = {};
  const total = nodes.length;
  let k = 0;
  const svg = mk("svg", { viewBox: `0 0 ${W} ${H}`, class: "vg" });
  const gl = mk("g", {}), gn = mk("g", {});
  svg.append(gl, gn);
  secs.forEach((s, i) => {
    const mine = nodes.filter((n) => n.id.startsWith(s + "/"));
    const a0 = (k / total) * Math.PI * 2, a1 = ((k + mine.length) / total) * Math.PI * 2, am = (a0 + a1) / 2;
    const sx = cx + R1 * Math.cos(am), sy = cy + R1 * Math.sin(am), col = colors[i % colors.length];
    pos[s] = [sx, sy];
    gl.append(mk("line", { x1: cx, y1: cy, x2: sx, y2: sy, stroke: col, "stroke-width": 3, opacity: 0.8 }));
    mine.forEach((n, j) => {
      const a = a0 + ((j + 0.5) / mine.length) * (a1 - a0), r = R2 + (j % 2) * 70;
      const x = cx + r * Math.cos(a), y = cy + r * Math.sin(a);
      pos[n.id] = [x, y, a, col];
      gl.append(mk("line", { x1: sx, y1: sy, x2: x, y2: y, stroke: col, "stroke-width": 1, opacity: 0.35 }));
      k += 1;
    });
    const t = mk("text", { x: sx, y: sy - 14, "text-anchor": "middle", class: "vg-sec", fill: col });
    t.textContent = s.replace(/^\d+-/, "").replace(/-/g, " ");
    gn.append(mk("circle", { cx: sx, cy: sy, r: 11, fill: col }), t);
  });
  // links between notes: drawn faint, lit up on hover
  const cross = [];
  nodes.forEach((n) => n.out.forEach((o) => { if (pos[o] && pos[n.id]) { const [x1, y1] = pos[n.id], [x2, y2] = pos[o]; const l = mk("path", { d: `M${x1},${y1} Q${cx},${cy} ${x2},${y2}`, class: "vg-link", "data-a": n.id, "data-b": o }); cross.push(l); gl.append(l); } }));
  nodes.forEach((n) => {
    const [x, y, a, col] = pos[n.id];
    const g = mk("g", { class: "vg-node", "data-id": n.id });
    const deg = (a * 180) / Math.PI, flip = Math.cos(a) < 0;
    const lab = mk("text", { x: x + (flip ? -9 : 9), y: y + 4, "text-anchor": flip ? "end" : "start", transform: `rotate(${flip ? deg + 180 : deg} ${x} ${y})`, class: "vg-label" });
    lab.textContent = n.id.split("/").pop().replace(/-/g, " ");
    const tip = mk("title", {}); tip.textContent = `${n.id}\n${n.desc}`;
    g.append(mk("circle", { cx: x, cy: y, r: 5, fill: col }), lab, tip);
    g.addEventListener("mouseenter", () => cross.forEach((l) => l.classList.toggle("on", l.dataset.a === n.id || l.dataset.b === n.id)));
    g.addEventListener("mouseleave", () => cross.forEach((l) => l.classList.remove("on")));
    g.addEventListener("click", () => openNote(n.id + ".md"));
    gn.append(g);
  });
  const hub = mk("circle", { cx, cy, r: 26, fill: "#fff" }), ht = mk("text", { x: cx, y: cy + 52, "text-anchor": "middle", class: "vg-sec", fill: "#fff" });
  ht.textContent = "Second brain";
  gn.append(hub, ht);
  return el("div", { class: "card vg-wrap", id: "vault-graph" }, svg);
}

async function viewVault() {
  const nav = navSeq;
  await freshStatus();
  const tree = await invoke("vault_tree");
  const tex = await invoke("textures_status").catch(() => ({ count: 0, groups: [] }));
  if (!state.vaultSection) state.vaultSection = tree[0] && tree[0].name;
  const sec = tree.find((s) => s.name === state.vaultSection) || tree[0];
  if (!sec) { showIf(nav, page(head("Vault"), el("div", { class: "card" }, "The vault is empty or missing. Open Find My Files in Settings to locate it."))); return; }
  const total = tree.reduce((n, s) => n + s.notes.length, 0);
  const verified = tree.reduce((n, s) => n + s.notes.filter((x) => x.status === "verified").length, 0);
  const results = el("div", { id: "vault-results" });
  const q = el("input", { type: "search", id: "vault-q", class: "search", placeholder: "Search every note", value: state.vaultQ || "" });
  let timer;
  const run = async () => {
    state.vaultQ = q.value;
    const text = q.value.trim();
    $("vault-browse").style.display = text ? "none" : "";
    if (!text) { results.replaceChildren(); return; }
    const hits = await invoke("vault_search", { q: text }).catch((e) => { toast(String(e), true); return []; });
    const esc = (t) => t.replace(/[&<>]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;" })[c]);
    results.replaceChildren(hits.length
      ? el("div", { class: "list" }, hits.map((h) => el("div", { class: "row tap", onclick: () => openNote(h.path) },
          el("div", { class: "grow" },
            el("div", { class: "row-title" }, h.title || h.path),
            el("div", { class: "row-sub", html: esc(h.snippet).replace(/\u0001/g, "<mark>").replace(/\u0002/g, "</mark>") })),
          el("span", { class: "pill" }, h.section),
          el("span", { class: "chev", html: ICONS.chev }))))
      : el("div", { class: "empty", style: "padding:40px 20px" }, el("div", { class: "empty-title" }, "No notes match"), el("div", {}, "Try fewer or different words.")));
  };
  q.addEventListener("input", () => { clearTimeout(timer); timer = setTimeout(run, 160); });
  showIf(nav, page(
    head("Vault", el("span", {}, `${total} notes · ${verified} verified · agents load one section at a time · `, runningSubtitle()),
      el("button", { class: "btn secondary", id: "open-obsidian", onclick: () => invoke("app_open", { name: "obsidian" }).catch((e) => toast(String(e), true)) }, "Open in Obsidian")),
    el("div", { class: "inline", style: "margin-bottom:10px" }, segmented([["list", "Notes"], ["graph", "Graph"]], state.vaultView || "list", (v) => { state.vaultView = v; viewVault(); })),
    state.vaultView === "graph" ? await vaultGraph() : null,
    texturePanel(tex),
    q, results,
    el("div", { class: "vault", id: "vault-browse" },
      el("div", { class: "vsecs" }, tree.map((s) => el("button", { class: s.name === sec.name ? "on" : "", onclick: () => { state.vaultSection = s.name; viewVault(); } }, s.name))),
      el("div", { class: "list" }, sec.notes.map((n) =>
        el("div", { class: "row tap", onclick: () => openNote(n.path) },
          el("div", { class: "grow" }, el("div", { class: "row-title" }, n.title || n.path), el("div", { class: "row-sub" }, n.description)),
          el("span", { class: "pill" + (n.status === "verified" ? " white" : n.status === "unverified" ? " red-soft" : "") }, cap(n.status)),
          el("span", { class: "chev", html: ICONS.chev })
        )
      ))
    )
  ));
  if (q.value.trim()) run();
}

// =====================================================================
// 4. Runs: history, new run, run detail
// =====================================================================
async function viewRuns() {
  const nav = navSeq;
  await freshStatus();
  const tabs = segmented([["history", "History"], ["new", "New Run"]], state.runsTab, (v) => { state.runsTab = v; viewRuns(); });
  if (state.runsTab === "new") return viewComposer(tabs);
  const list = await invoke("runs_overview");
  showIf(nav, page(
    head("Runs", runningSubtitle(), tabs),
    list.length
      ? el("div", { class: "list", id: "runs-list" }, list.map((r) =>
          el("div", { class: "row tap", "data-run": r.id, onclick: () => { location.hash = "#run/" + r.id; } },
            el("div", { class: "grow" }, el("div", { class: "row-title" }, cap(r.title)), el("div", { class: "row-sub" }, `${r.kind === "pipeline" ? "Pipeline" : "Single agent"} · ${ago(r.created_at)} · est. ${usd(r.estimate_usd, 4)} · actual ${usd(r.actual_usd, 4)}`)),
            runPill(r.status),
            el("span", { class: "chev", html: ICONS.chev })
          )))
      : el("div", { class: "empty" }, el("div", { class: "ic-big", html: ICONS.list }), el("div", { class: "empty-title" }, "No runs yet"), el("div", {}, "Start one from New Run. You see the cost before anything runs."))
  ));
}
function runPill(s) {
  const label = { done: "Done", running: "Running", failed: "Failed", stopped: "Stopped", paused: "Paused", approved: "Starting", "awaiting-approval": "Needs approval" }[s] || cap(s);
  const cls = s === "running" || s === "approved" ? "pill red" : s === "failed" || s === "stopped" ? "pill red-soft" : s === "done" ? "pill white" : "pill";
  return el("span", { class: cls }, label);
}

async function viewComposer(tabs) {
  const nav = navSeq;
  const c = state.composer;
  const [agents, pipes, projects] = await Promise.all([invoke("agents_list"), invoke("pipelines_list"), invoke("projects_list").catch(() => [])]);
  const agentNames = Object.keys(agents);
  if (!c.agent || !agents[c.agent]) c.agent = agentNames.includes("lead") ? "lead" : agentNames[0];
  if (!c.pipeline || !pipes[c.pipeline]) c.pipeline = Object.keys(pipes)[0];
  const prompt = el("textarea", { id: "prompt", placeholder: "Tell the team what you want, in plain words." });
  prompt.value = c.prompt;
  prompt.addEventListener("input", () => (c.prompt = prompt.value));
  const agentSel = el("select", { id: "agent-select" }, agentNames.map((a) => el("option", { value: a }, a === "bridge-test" ? "Bridge test (connection check)" : cap(a))));
  agentSel.value = c.agent;
  agentSel.addEventListener("change", () => { c.agent = agentSel.value; viewRuns(); });
  const pipeSel = el("select", { id: "pipeline-select" }, Object.keys(pipes).map((p) => el("option", { value: p }, p)));
  pipeSel.value = c.pipeline;
  pipeSel.addEventListener("change", () => { c.pipeline = pipeSel.value; viewRuns(); });
  const projSel = el("select", { id: "project-select" }, el("option", { value: "" }, "Choose a project"), projects.map((p) => el("option", { value: p }, p)));
  projSel.value = c.project;
  projSel.addEventListener("change", () => (c.project = projSel.value));
  const pdef = pipes[c.pipeline];
  const estimateBox = el("div", { id: "estimate" });
  const [models, st] = await Promise.all([invoke("models_list").catch(() => []), invoke("settings_get")]);
  const runModel = st.engine === "ollama" ? null : modelSelect("run-model", models, st.run_model, "Each agent's own model", async (v) => { await invoke("settings_patch", { patch: { run_model: v } }).catch((e) => toast(String(e), true)); estimateBox.replaceChildren(); toast(v ? "Agents will use this model." : "Agents use their own models."); });
  const form = el("div", { class: "card" },
    el("div", { style: "margin-bottom:16px" }, segmented([["agent", "One Agent"], ["pipeline", "Pipeline"]], c.mode, (v) => { c.mode = v; viewRuns(); })),
    c.mode === "agent"
      ? el("div", { class: "field" }, el("label", { for: "agent-select" }, "Agent"), agentSel, el("div", { class: "note" }, agents[c.agent].description + " · " + agents[c.agent].model))
      : el("div", {},
          el("div", { class: "field" }, el("label", { for: "pipeline-select" }, "Pipeline"), pipeSel, el("div", { class: "note" }, pdef.description + " Stages: " + pdef.stages.map((s) => cap(s.agent)).join(", "))),
          pdef.needs_project ? el("div", { class: "field" }, el("label", { for: "project-select" }, "Game project"), projSel, projects.length ? null : el("div", { class: "note" }, "No projects yet. Import one in Settings.")) : null),
    el("div", { class: "field" }, el("label", { for: "prompt" }, "Instruction"), prompt),
    el("div", { class: "inline" }, runModel, el("button", { class: "btn primary", id: "estimate-btn", onclick: () => estimate(estimateBox) }, "Estimate Usage"), el("span", { class: "note" }, "Nothing runs until you approve the cost."))
  );
  showIf(nav, page(head("Runs", runningSubtitle(), tabs), form, estimateBox));
}

// A new estimate replaces the last one nobody approved (so it does not
// linger as "waiting for approval").
function dropPrepared() {
  const p = state.prepared;
  state.prepared = null;
  if (p) invoke(p.kind === "run" ? "run_cancel" : "pipeline_cancel", p.kind === "run" ? { runId: p.id } : { pipelineId: p.id }).catch(() => {});
  if (state.costRun) { invoke("run_cancel", { runId: state.costRun }).catch(() => {}); state.costRun = null; }
}
async function estimate(box) {
  const c = state.composer;
  dropPrepared();
  box.replaceChildren();
  try {
    if (c.mode === "plan") {
      c.mode = "agent"; // one-off: the Runs form must not stay in plan mode
      const p = await invoke("plan_build_prepare", { runId: c.planRun });
      state.prepared = { kind: "pipeline", id: p.pipeline_id, expected: p.expected_usd, low: null, high: p.high_usd, cap: p.cap_usd, assumptions: p.stages.map((s) => `${cap(s.agent)} writes ${s.output}`), note: "Nothing was spent on building until now. Each stage runs under its own cap." };
    } else if (c.mode === "agent") {
      const r = await invoke("run_prepare", { agent: c.agent, prompt: c.prompt });
      state.prepared = { kind: "run", id: r.run_id, expected: r.estimate.expected_usd, low: r.estimate.low_usd, high: r.estimate.high_usd, cap: r.estimate.cap_usd, assumptions: r.estimate.assumptions, note: `Prices checked ${r.estimate.pricing_checked}. ${r.estimate.subscription_note}` };
    } else {
      const p = await invoke("pipeline_prepare", { name: c.pipeline, input: c.prompt, project: c.project || null });
      state.prepared = { kind: "pipeline", id: p.pipeline_id, expected: p.expected_usd, low: null, high: p.high_usd, cap: p.cap_usd, assumptions: p.stages.map((s) => `${cap(s.agent)} writes ${s.output}`), note: "Each stage runs under its own cap. The hard cap is the most this can cost." };
    }
  } catch (e) {
    box.append(el("div", { class: "card", style: "margin-top:16px" }, el("div", { class: "row-title" }, "Can't start this run"), el("p", { class: "note" }, String(e))));
    return;
  }
  const p = state.prepared;
  const sp = state.spend;
  const approve = el("button", { class: "btn primary", id: "approve-run", onclick: () => approveRun(approve) }, `Approve and Run · cap ${usd(p.cap)}`);
  box.append(el("div", { class: "card", style: "margin-top:16px" },
    el("div", { class: "inline", style: "justify-content:space-between" }, el("h2", { style: "margin:0" }, "Usage before you run"), el("span", { class: "pill red-soft" }, "Needs your approval")),
    el("div", { class: "cost-grid" },
      el("div", {}, el("div", { class: "k" }, "Expected"), el("div", { class: "v" }, usd(p.expected, 4))),
      el("div", {}, el("div", { class: "k" }, p.low !== null ? "Range" : "Up to"), el("div", { class: "v small" }, p.low !== null ? `${usd(p.low, 4)} to ${usd(p.high, 4)}` : usd(p.high, 4))),
      el("div", {}, el("div", { class: "k" }, "Hard cap"), el("div", { class: "v" }, usd(p.cap))),
      null),
    el("ul", { class: "assumptions" }, p.assumptions.map((a) => el("li", {}, a))),
    el("div", { class: "note" }, p.note),
    el("div", { class: "inline", style: "margin-top:14px" }, approve,
      el("button", { class: "btn secondary", id: "cancel-run", onclick: async () => {
        await invoke(p.kind === "run" ? "run_cancel" : "pipeline_cancel", p.kind === "run" ? { runId: p.id } : { pipelineId: p.id }).catch(() => {});
        state.prepared = null;
        box.replaceChildren();
      } }, "Cancel"))
  ));
}

async function approveRun(btn) {
  const p = state.prepared;
  btn.disabled = true;
  try {
    // Set before starting: a run that fails at once sends run-done first.
    state.live = { kind: p.kind, id: p.id, estimate: p.expected, cap: p.cap };
    state.logs[p.id] = "";
    if (p.kind === "run") await invoke("run_approve_and_start", { runId: p.id, confirmedCapUsd: p.cap });
    else await invoke("pipeline_approve_and_start", { pipelineId: p.id, confirmedCapUsd: p.cap });
    state.prepared = null;
    state.composer.prompt = "";
    location.hash = "#run/" + p.id;
  } catch (e) {
    state.live = null;
    btn.disabled = false;
    toast(String(e), true);
  }
}

async function viewRunDetail(id) {
  const nav = navSeq;
  await freshStatus();
  let d;
  try {
    d = await invoke("run_detail", { id });
  } catch (e) {
    return showIf(nav, page(el("button", { class: "back", onclick: () => (location.hash = "#runs") }, "‹ Runs"), head("Run not found"), el("div", { class: "card" }, String(e))));
  }
  const live = state.live && state.live.id === id;
  const activeIdx = d.steps.findIndex((s) => s.status === "running" || s.status === "approved");
  showIf(nav, page(
    el("button", { class: "back", onclick: () => (location.hash = "#runs") }, "‹ Runs"),
    head(cap(d.title), el("span", {}, `${d.kind === "pipeline" ? "Pipeline" : "Single agent"} · started ${ago(d.created_at)}`), runPill(d.status), true),
    el("div", { class: "card" },
      el("div", { class: "cost-grid", style: "margin:0" },
        el("div", {}, el("div", { class: "k" }, "Estimated before"), el("div", { class: "v" }, usd(d.estimate_usd, 4))),
        el("div", {}, el("div", { class: "k" }, "Hard cap"), el("div", { class: "v small" }, usd(d.cap_usd))),
        el("div", {}, el("div", { class: "k" }, "Actual after"), el("div", { class: "v", id: "actual-cost" }, usd(d.actual_usd, 4))),
        d.queue_item ? el("div", {}, el("div", { class: "k" }, "Result"), el("button", { class: "link", style: "padding-top:8px", onclick: () => (location.hash = "#queue") }, "In the review queue")) : null)
    ),
    d.stopped_reason ? el("div", { class: "card warn-card", id: "stopped-card" }, el("div", { class: "row-title" }, "Stopped early"), el("p", { class: "note" }, d.stopped_reason)) : null,
    d.status === "paused" ? el("div", { class: "card paused-card", id: "paused-card" },
      el("div", { class: "row-title" }, "Paused, not lost"),
      el("p", { class: "note" }, "Claude stopped partway (usage limit reached, or the app closed). Everything it made so far is kept. It continues by itself from where it stopped" + (d.resume_at ? ` at about ${new Date(d.resume_at).toLocaleTimeString([], { hour: "numeric", minute: "2-digit" })}` : "") + ", and finished steps are not run again."),
      el("div", { class: "inline", style: "gap:8px;flex-wrap:wrap" },
        el("button", { class: "btn primary", id: "continue-now", onclick: (e) => continueNow(id, e.currentTarget) }, "Continue Now"),
        el("button", { class: "btn secondary", id: "continue-local", onclick: (e) => continueLocal(id, e.currentTarget) }, "Continue on Local Model")),
      el("p", { class: "note", style: "margin-top:6px" }, "Continue on Local Model keeps going for free on this computer through Ollama, just slower and weaker. Claude takes over new steps again once its limit resets.")) : null,
    el("div", { class: "section" }, el("div", { class: "section-label" }, "Steps"),
      el("div", { class: "list steps" }, d.steps.map((s, i) =>
        el("div", { class: "step" },
          el("div", { class: "step-num" + (i === activeIdx ? " on" : "") }, String(i + 1)),
          el("div", {},
            el("div", { class: "inline", style: "justify-content:space-between" }, el("div", { class: "row-title" }, cap(s.agent)), runPill(s.status)),
            el("div", { class: "note" }, `est. ${usd(s.estimate_usd, 4)} · cap ${usd(s.cap_usd)} · actual ${usd(s.actual_usd, 4)}`),
            stepUsage(s) ? el("div", { class: "note" }, stepUsage(s)) : null,
            s.stop_reason === "turn_limit" ? el("div", { class: "note warn" }, `Stopped at its ${s.turn_limit || ""} turn limit. Files it wrote are kept.`) : s.stop_reason === "budget_cap" ? el("div", { class: "note warn" }, "Stopped at its cap. Files it wrote are kept.") : null,
            s.error && s.status !== "paused" ? el("div", { class: "log", style: "margin-top:8px;white-space:pre-wrap" }, s.error) : null,
            el("div", { class: "files" }, s.output ? el("button", { class: "link", onclick: () => openText(s.output) }, "Open " + s.output.split("/").pop()) : el("span", { class: "note" }, "Nothing produced yet"),
              s.loaded.length ? el("span", { class: "note" }, `Read ${s.loaded.length} vault file${s.loaded.length === 1 ? "" : "s"}: ` + s.loaded.map((x) => x.split("/").slice(-2).join("/")).slice(0, 6).join(", ") + (s.loaded.length > 6 ? "…" : "")) : null),
            s.result_text ? el("details", { style: "margin-top:8px" }, el("summary", { class: "note", style: "cursor:pointer" }, "What the agent said"), el("div", { class: "log", style: "margin-top:6px" }, s.result_text)) : null,
            s.gate ? el("div", {}, el("div", { class: "note", style: "margin-top:8px" }, s.gate.passed ? "Verified: all checks passed" : "Verification: some checks failed"), checksList(s.gate.checks)) : null
          )
        )))),
    d.status === "running" ? el("div", { class: "inline", style: "gap:12px;margin:8px 0" }, ...agentControls("running", id)) : null,
    live || d.status === "running" ? el("div", { class: "section" }, el("div", { class: "section-label" }, "Live output"), el("pre", { class: "log", id: "live-log" }, state.logs[id] || "Starting…")) : null
  ));
}

// Real numbers from the finished run: time, turns, tokens.
function stepUsage(s) {
  const u = s.usage || {};
  const parts = [];
  if (s.elapsed_ms) { const m = Math.round(s.elapsed_ms / 60000); parts.push(m >= 1 ? `${m} min` : `${Math.max(1, Math.round(s.elapsed_ms / 1000))} s`); }
  if (s.turns != null) parts.push(`${s.turns}${s.turn_limit ? " of " + s.turn_limit : ""} turns`);
  const k = (n) => n >= 1000 ? Math.round(n / 1000) + "k" : String(n);
  const inTok = (u.input_tokens || 0) + (u.cache_read_tokens || 0) + (u.cache_write_tokens || 0);
  if (inTok) parts.push(`${k(inTok)} tokens read (${k(u.cache_read_tokens || 0)} cached)`);
  if (u.output_tokens) parts.push(`${k(u.output_tokens)} written`);
  return parts.join(" · ");
}
async function continueNow(id, btn) {
  if (btn) btn.disabled = true;
  try {
    await invoke("continue_now", { id });
    toast("Continuing where it left off.");
  } catch (e) {
    toast(String(e), true);
    if (btn) btn.disabled = false;
  }
}
// Usage ran out: keep going on the free local model (Ollama) instead of waiting.
async function continueLocal(id, btn) {
  if (btn) btn.disabled = true;
  try {
    const n = await invoke("continue_local", { id: id || null });
    toast(n ? "Carrying on with the local model. Slower, but free." : "Nothing was paused.");
  } catch (e) {
    toast(String(e), true);
    if (btn) btn.disabled = false;
  }
}
T.event.listen("engine-changed", (ev) => {
  if (ev.payload && ev.payload.auto) toast("Claude's limit reset. New steps use Claude again.");
  refreshStatus();
});
// Paused work started again (by the timer or Continue Now).
T.event.listen("work-resumed", (ev) => {
  refreshStatus();
  const h = location.hash || "";
  if (h.startsWith("#run/")) viewRunDetail(h.slice(5));
  else if (h === "#chat") viewChat();
  if (!ev.payload || !ev.payload.manual) toast(`Claude's limit reset. Continuing ${ev.payload && ev.payload.count > 1 ? ev.payload.count + " paused jobs" : "paused work"}.`);
});

function renderLine(line) {
  let v;
  try { v = JSON.parse(line); } catch { return null; }
  if (v.type === "assistant" && v.message && Array.isArray(v.message.content)) {
    return v.message.content.map((c) => {
      if (c.type === "text") return c.text;
      if (c.type === "tool_use") {
        const t = c.input && (c.input.file_path || c.input.path || c.input.pattern || c.input.command || "");
        return `· ${c.name} ${String(t).replace(/^.*\/vault\//, "")}`;
      }
      return null;
    }).filter(Boolean).join("\n");
  }
  if (v.type === "result") return `Finished · ${usd(v.total_cost_usd, 4)} · ${v.num_turns} turns`;
  return null;
}
const LOG_MAX = 200000; // keep the last ~200 KB of a run's live output
function appendLog(id, text) {
  // Runs nobody is watching keep only a short tail (memory).
  const max = location.hash === "#run/" + id ? LOG_MAX : 4000;
  let t = (state.logs[id] || "") + text + "\n";
  if (t.length > max * 1.25) t = t.slice(-max);
  state.logs[id] = t;
  const log = $("live-log");
  if (log && location.hash === "#run/" + id) {
    log.append(text + "\n");
    log._len = (log._len || 0) + text.length + 1;
    if (log._len > LOG_MAX * 1.5) { log.textContent = t; log._len = t.length; }
    if (!log._scrollQueued) { log._scrollQueued = true; requestAnimationFrame(() => { log._scrollQueued = false; log.scrollTop = log.scrollHeight; }); }
  }
}
T.event.listen("run-line", (ev) => {
  const id = ev.payload.pipeline_id || ev.payload.run_id;
  const t = renderLine(ev.payload.line);
  if (t) appendLog(id, t);
});
T.event.listen("pipe-stage", (ev) => {
  const d = ev.payload;
  appendLog(d.pipeline_id, d.state === "done" ? `— ${cap(d.agent)} finished: checks ${d.gate.passed ? "passed" : "failed"}` : `— ${cap(d.agent)} started`);
  if (location.hash === "#run/" + d.pipeline_id) setTimeout(() => viewRunDetail(d.pipeline_id), 50);
});
function finishLive(id, ok, queued) {
  setTimeout(() => { delete state.logs[id]; if (state.apLive) delete state.apLive[id]; }, 60000);
  if (state.live && state.live.id === id) state.live = null;
  refreshStatus();
  toast(ok ? (queued ? "Run finished. It's waiting in the review queue." : "Run finished.") : "Run stopped. Open it to see why.", !ok);
  if (location.hash === "#run/" + id) viewRunDetail(id);
}
T.event.listen("run-done", (ev) => finishLive(ev.payload.run_id, ev.payload.ok, !!ev.payload.queue_item));
T.event.listen("pipe-done", (ev) => finishLive(ev.payload.pipeline_id, ev.payload.status === "done", !!ev.payload.queue_item));

// =====================================================================
// 5. Settings
// =====================================================================
// Local model picker: what this computer can run, what is installed, and a
// Download button that names the model and its size. Nothing downloads alone.
function localModelRows(s, o, save) {
  if (!o) return el("div", { class: "srow" }, el("div", { class: "grow label" }, "Local model"), el("span", { class: "note" }, "Could not check Ollama."));
  const installed = new Set(o.installed);
  const has = (m) => installed.has(m) || installed.has(m + ":latest");
  const names = [...new Set([...o.choices.map((c) => c.model), ...o.installed])];
  const pick = el("select", { id: "ollama-model", style: "width:240px" }, names.map((m) =>
    el("option", { value: m }, `${m}${m === o.recommended.model ? "  (best fit)" : ""}${has(m) ? "" : "  · not downloaded"}`)));
  pick.value = state.ollamaPicked || s.ollama_model || o.recommended.model;
  const info = el("div", { class: "hint", id: "ollama-info" });
  const dl = el("button", { class: "btn secondary", id: "ollama-download" });
  const refresh = () => {
    const c = o.choices.find((x) => x.model === pick.value);
    info.textContent = c ? c.why : "Installed in Ollama.";
    dl.style.display = has(pick.value) || !c || !o.version_ok ? "none" : "";
    dl.textContent = c ? `Download · ${c.size_gb} GB` : "";
  };
  pick.addEventListener("change", () => { state.ollamaPicked = pick.value; refresh(); });
  dl.onclick = async () => {
    dl.disabled = true;
    state.ollamaPull = pick.value;
    state.ollamaPicked = pick.value;
    try { await invoke("ollama_pull", { model: pick.value }); info.textContent = "Starting download..."; } catch (e) { dl.disabled = false; toast(String(e), true); }
  };
  refresh();
  return [
    el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Ollama"), el("div", { class: "hint", id: "ollama-status" }, o.message)),
      el("span", { class: "note" }, [o.version ? `v${o.version}` : "not running", o.memory_gb ? `${o.memory_gb} GB memory` : ""].filter(Boolean).join(" · "))),
    el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Local model"), info),
      el("div", { class: "inline" }, pick, dl, el("button", { class: "btn secondary", id: "ollama-save", onclick: () => save({ ollama_model: pick.value }, "Local model saved.") }, "Use")))];
}
T.event.listen("ollama-line", (ev) => { const n = $("ollama-info"); if (n) n.textContent = `Downloading ${ev.payload.model}: ${ev.payload.line}`; });
T.event.listen("ollama-done", (ev) => {
  state.ollamaPull = null;
  if (ev.payload.error) toast(`Download failed: ${ev.payload.error}`, true); else toast(`${ev.payload.model} is downloaded.`);
  if ((location.hash || "") === "#settings") viewSettings();
});

// One app (Blender or Obsidian) living in the Downloads folder.
async function chooseAppFolder(id, label) {
  const f = await invoke("pick_folder");
  if (!f) return;
  const s = await invoke("settings_get");
  Object.assign(s, await invoke("settings_patch", { patch: { app_dirs: { [id]: f } } }));
  toast(`${label} will install in ${f}.`);
  viewSettings();
}
function appRow(id, name, path, canInstall, hint, dir) {
  const status = el("div", { class: "hint", id: `app-${id}-status` }, path ? `Installed: ${path}` : `${hint} Folder: ${dir || ""}`);
  const install = canInstall ? el("button", { class: path ? "btn secondary" : "btn primary", id: `app-${id}-install`, onclick: async (ev) => {
    ev.target.disabled = true; status.textContent = "Starting...";
    try { await invoke("app_install", { name: id }); } catch (e) { ev.target.disabled = false; toast(String(e), true); }
  } }, path ? "Reinstall" : "Install Here") : null;
  const open = id === "git" || id === "graphify" ? null : path || id === "obsidian" ? el("button", { class: "btn secondary", onclick: () => invoke("app_open", { name: id }).catch((e) => toast(String(e), true)) }, id === "obsidian" ? "Open Vault" : "Open") : null;
  const pick = el("button", { class: "btn secondary", id: `app-${id}-pick`, onclick: () => chooseAppFolder(id, name) }, "Choose Folder");
  return el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, name), status), el("div", { class: "inline" }, pick, install, open));
}
T.event.listen("app-line", (ev) => { const n = $(`app-${ev.payload.app}-status`); if (n) n.textContent = ev.payload.line; });
T.event.listen("app-done", (ev) => {
  if (ev.payload.error) toast(ev.payload.error, true); else toast(ev.payload.message);
  if (ev.payload.app === "git") refreshClaude();
  if ((location.hash || "") === "#settings") viewSettings();
});

async function viewSettings() {
  const nav = navSeq;
  await freshStatus();
  const [s, info, o, sm, bm, td, ap] = await Promise.all([invoke("settings_get"), invoke("studio_info"), invoke("ollama_status").catch(() => null), invoke("studio_mcp_status").catch(() => ({})), invoke("blender_mcp_status").catch(() => ({})), invoke("tools_dir_get").catch(() => ""), invoke("apps_status").catch(() => ({}))]);
  const c = state.claude || {};
  const save = async (patch, msg) => {
    try {
      const next = await invoke("settings_patch", { patch });
      Object.assign(s, next);
      toast(msg || "Saved.");
      refreshStatus();
    } catch (e) {
      toast(String(e), true);
    }
  };
  const hook = el("input", { type: "password", id: "webhook", placeholder: "https://discord.com/api/webhooks/…", value: s.discord_webhook_url });
  const root = el("input", { type: "text", id: "studio-root", placeholder: "New folder, e.g. ~/Documents/BrewixGames", value: s.studio_root });
  const importPath = el("input", { type: "text", id: "import-path", placeholder: "~/workspace/roblox-games/sky-tower-obby" });
  const updLog = el("pre", { class: "log hidden", id: "update-log", style: "margin-top:12px" });
  const updRow = el("div", { class: "srow", id: "update-row" }, el("div", { class: "grow" }, el("div", { class: "label", id: "build-label" }, `Build ${info.build}`), el("div", { class: "hint", id: "update-hint" }, "Downloads the newest BrewixGames, installs it, and restarts. Your projects and review queue are kept.")),
    el("button", { class: "btn secondary", id: "update-check", onclick: () => checkUpdates(updRow, updLog) }, "Check for Updates"));
  const sp = state.spend;
  showIf(nav, page(
    head("Settings", runningSubtitle()),

    el("div", { class: "section-label" }, "Claude"),
    el("div", { class: "list" },
      el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, c.logged_in ? "Connected" : "Not connected"), el("div", { class: "hint", id: "claude-msg" }, c.message || "Checking…")),
        el("button", { class: "btn secondary", onclick: async () => { await refreshClaude(); viewSettings(); } }, "Check Again"),
        c.git_ok === false ? el("button", { class: "btn primary", id: "git-install", onclick: () => { location.hash = "#settings"; setTimeout(() => { const b = $("app-git-install"); if (b) { b.scrollIntoView({ behavior: "smooth", block: "center" }); b.click(); } }, 400); } }, "Install Git") : null,
        !c.installed ? el("button", { class: "btn primary", id: "install-claude", onclick: async (e) => { e.target.disabled = true; e.target.textContent = "Installing…"; try { toast(await invoke("claude_install")); } catch (err) { toast(String(err), true); } await refreshClaude(); viewSettings(); } }, "Install Claude Code") : null,
        c.installed && !(c.logged_in && c.uses_subscription) ? el("button", { class: "btn primary", onclick: async (e) => { e.target.disabled = true; e.target.textContent = "Waiting for browser…"; try { toast(await invoke("claude_sign_in")); } catch (err) { toast(String(err), true); } await refreshClaude(); viewSettings(); } }, "Sign In to Claude") : null),
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Login method"), el("span", { class: "note" }, c.auth_method ? (c.uses_subscription ? "Claude account login · uses your subscription, no API key" : c.auth_method) : "–")),
      el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Claude Code"),
          el("div", { class: "hint" }, c.version ? `${c.version} · ${c.bin}` : "Not found. The Claude desktop app or website alone is not enough. Press Install, or paste where claude is.")),
        el("div", { class: "inline" },
          el("input", { type: "text", id: "claude-path", style: "width:220px", placeholder: "claude location (optional)", value: s.claude_path || "" }),
          el("button", { class: "btn secondary", onclick: async () => { await save({ claude_path: $("claude-path").value.trim() }, "Saved. Checking Claude..."); await freshStatus(); viewSettings(); } }, "Save"),
          el("button", { class: "btn secondary", id: "app-claude-pick", title: (ap.dirs || {}).claude || "", onclick: () => chooseAppFolder("claude", "Claude Code") }, "Choose Folder"),
          el("button", { class: c.version ? "btn secondary" : "btn primary", id: "claude-install", title: "Puts BrewixGames' copy of Claude Code in the Downloads folder (installs it first if needed).", onclick: async (ev) => { ev.target.disabled = true; ev.target.textContent = c.version ? "Copying..." : "Installing..."; try { toast(await invoke("claude_install")); } catch (e) { toast(String(e), true); } await refreshClaude(); viewSettings(); } }, c.version ? "Refresh Copy" : "Install")))
    ),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "AI engine"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Who does the thinking"), el("div", { class: "hint" }, s.engine === "ollama" ? "Local model through Ollama: free and private, but weaker than Claude. Expect more failed checks." : s.engine === "hybrid" ? "Mostly local: every agent runs free on your local model, except Playtest in Roblox Studio and Modeler in Blender (when those are connected), which use Claude. Saves most of your Claude usage." : "Claude, through your Claude subscription. Best quality.")),
          segmented([["claude", "Claude"], ["hybrid", "Mostly local"], ["ollama", "Local (free)"]], s.engine, async (v) => { await save({ engine: v, ollama_model: (($("ollama-model") || {}).value || s.ollama_model || "").trim() }, v === "ollama" ? "Using the local model." : v === "hybrid" ? "Mostly local: Claude only tests in Studio and models in Blender." : "Using Claude."); viewSettings(); })),
        localModelRows(s, o, save))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Apps in your Downloads folder"),
      el("div", { class: "list" },
        appRow("blender", "Blender", ap.blender, ap.can_install, "Portable Blender 5.2 LTS: the program, its settings and the MCP for Blender add-on all stay in its folder.", (ap.dirs || {}).blender),
        appRow("obsidian", "Obsidian", ap.obsidian, ap.can_install, "Obsidian for browsing the vault. Installed into its folder if its installer accepts it (BrewixGames checks).", (ap.dirs || {}).obsidian),
        appRow("graphify", "Graphify (code map)", ap.graphify, true, "Free code map so Builder, Tuner and Playtest can find where things are in your game without reading every file. Installs with uv.", (ap.dirs || {}).graphify),
        ap.can_install ? appRow("git", "Git for Windows", ap.git, ap.can_install, "Claude Code needs Git on Windows to run (and to reach Roblox Studio). Free, installed into this folder.", (ap.dirs || {}).git) : null,
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Roblox Studio"),
          el("div", { class: "hint" }, "Roblox always installs Studio in your user folder on C:, with no option to choose. Moving it with folder tricks is reported to break Studio on updates, so BrewixGames leaves it where Roblox puts it."))))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Roblox Studio"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Connect agents to Studio"),
            el("div", { class: "hint" }, s.studio_mcp === "studio" ? "Builder, Scenic, Frame, Playtest and Ask Claude can work inside your open Studio place. Publishing, uploads, purchases, web requests and DataStore writes are blocked."
              : s.studio_mcp === "mock" ? "Test mode: agents talk to a pretend Studio built into BrewixGames. Nothing touches Roblox."
              : "Off. Agents only write files in your projects folder.")),
          el("button", { class: "btn secondary", id: "studio-test", onclick: async (e) => { e.target.disabled = true; e.target.textContent = "Testing..."; try { toast(await invoke("studio_mcp_test")); } catch (err) { toast(String(err), true); } e.target.disabled = false; e.target.textContent = "Test Connection"; } }, "Test Connection"),
          segmented([["off", "Off"], ["studio", "Roblox Studio"], ["mock", "Test"]], s.studio_mcp, async (v) => { await save({ studio_mcp: v, studio_mcp_path: ($("studio-mcp-path") || {}).value || s.studio_mcp_path }, v === "off" ? "Studio connection off." : v === "mock" ? "Using the test Studio." : "Connected to Roblox Studio."); viewSettings(); })),
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Studio MCP server"),
            el("div", { class: "hint" }, sm.found ? `Found: ${sm.found}. In Studio, also turn it on: Assistant, the ... menu, Manage MCP Servers, Enable Studio as MCP server.` : "Not found. Update Roblox Studio and open it once; the MCP server is built in. Then in Studio: Assistant, the ... menu, Manage MCP Servers, Enable Studio as MCP server. No Roblox login is stored.")),
          el("div", { class: "inline" }, el("input", { type: "text", id: "studio-mcp-path", style: "width:240px", placeholder: "custom path (optional)", value: s.studio_mcp_path }),
            el("button", { class: "btn secondary", onclick: () => save({ studio_mcp_path: $("studio-mcp-path").value.trim() }, "Path saved.") }, "Save"))))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Figma"),
      el("div", { class: "list" },
        el("div", { class: "srow" },
          el("div", { class: "grow" }, el("div", { class: "label" }, "Figma (optional login)"),
            el("div", { class: "hint" }, s.figma_mcp ? "Frame and Planner can draw screens straight into your Figma file. One-time setup: open a terminal, run: claude mcp add --transport http Figma https://mcp.figma.com/mcp, then start claude, type /mcp and sign in to Figma. No API key. Figma's free plan allows only a few reads a month." : "Off: agents make SVG mockups and you drag them into Figma with Edit in Figma on the Game Plan page (no login, no limits). Turn on to let agents draw in Figma directly with your Figma login.")),
          segmented([["off", "Off"], ["on", "On"]], s.figma_mcp ? "on" : "off", async (v) => { await save({ figma_mcp: v === "on" }, v === "on" ? "Figma login connection on." : "Figma connection off."); viewSettings(); })))),
    el("div", { class: "section" }, el("div", { class: "section-label" }, "Blender"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Connect Modeler to Blender"),
            el("div", { class: "hint" }, s.blender_mcp === "blender" ? "Modeler and Ask Claude can build models in your open Blender and export FBX for Studio. Free: only Poly Haven (CC0) assets, no paid generators, telemetry off."
              : s.blender_mcp === "mock" ? "Test mode: Modeler talks to a pretend Blender built into BrewixGames."
              : "Off. Modeler writes Blender scripts you can run yourself.")),
          segmented([["off", "Off"], ["blender", "Blender"], ["mock", "Test"]], s.blender_mcp, async (v) => { await save({ blender_mcp: v }, v === "off" ? "Blender connection off." : v === "mock" ? "Using the test Blender." : "Connected to Blender."); viewSettings(); })),
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "What you need (all free)"),
            el("div", { class: "hint" }, `Blender from blender.org, uv from astral.sh/uv${bm.uvx ? " (found)" : " (not found)"}, then press Get the Blender Add-on and in Blender choose Edit, Preferences, Add-ons, Install from Disk, pick that file, and click Connect in the add-on's panel. BrewixGames runs ${bm.package || "the server"} with telemetry off.`)),
          bm.uvx ? null : el("button", { class: "btn primary", id: "uv-install", onclick: async (ev) => { ev.target.disabled = true; ev.target.textContent = "Installing uv..."; try { toast(await invoke("uv_install")); } catch (e) { toast(String(e), true); } viewSettings(); } }, "Install uv"),
          el("button", { class: "btn secondary", id: "blender-addon", disabled: !bm.uvx, onclick: async (ev) => { ev.target.disabled = true; try { const p = await invoke("blender_addon_get"); toast(`Add-on saved to ${p}`); } catch (e) { toast(String(e), true); } ev.target.disabled = false; } }, "Get the Blender Add-on")))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Notifications"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Discord webhook"), el("div", { class: "hint" }, "Stored on this computer only, never in the repo.")),
          el("div", { class: "inline", style: "width:430px" }, hook,
            el("button", { class: "btn secondary", id: "save-webhook", onclick: () => save({ discord_webhook_url: hook.value.trim() }, hook.value.trim() ? "Webhook saved." : "Webhook removed.") }, "Save"),
            el("button", { class: "btn secondary", id: "send-test", onclick: async () => { try { toast(await invoke("notify_test")); } catch (e) { toast(String(e), true); } } }, "Send Test"))),
        [["needs_approval", "Something needs my approval"], ["finished", "A build or run finished"], ["failed", "A run failed or stopped"], ["updates", "An app update was installed"]].map(([k, label]) =>
          el("div", { class: "srow" }, el("div", { class: "grow label" }, label), sw(s.notify[k], (v) => save({ notify: { [k]: v } }), "n-" + k))))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Studio"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Downloads folder"),
            el("div", { class: "hint" }, `Everything BrewixGames downloads goes here: its copy of Claude Code, uv and its packages, and the Blender add-on. Now: ${td}`)),
          el("div", { class: "inline", style: "width:430px" }, el("input", { type: "text", id: "tools-dir", placeholder: "Default: tools inside the studio folder", value: s.tools_dir || "" }),
            el("button", { class: "btn secondary", id: "tools-dir-pick", onclick: async () => { const f = await invoke("pick_folder"); if (f) { $("tools-dir").value = f; await save({ tools_dir: f }, "Downloads folder set."); viewSettings(); } } }, "Choose Folder"),
            el("button", { class: "btn secondary", id: "tools-dir-save", onclick: () => save({ tools_dir: $("tools-dir").value.trim() }, "Downloads folder saved. Press Install next to Claude Code and uv to put them there.") }, "Save"))),
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Studio folder"), el("div", { class: "hint" }, `In use: ${info.root}. Vault: ${info.vault}. Lost something? `, el("a", { class: "link", href: "#files", id: "find-files" }, "Find My Files"))),
          el("div", { class: "inline", style: "width:430px" }, root, el("button", { class: "btn secondary", id: "studio-move", onclick: async (ev) => { ev.target.disabled = true; ev.target.textContent = "Moving..."; try { toast(await invoke("studio_move", { dest: root.value.trim() })); viewSettings(); } catch (e) { toast(String(e), true); } ev.target.disabled = false; ev.target.textContent = "Move Here"; } }, "Move Here"))),
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Import a game project"), el("div", { class: "hint" }, "Copies a Rojo project into projects/ so agents can work on it.")),
          el("div", { class: "inline", style: "width:430px" }, importPath, el("button", { class: "btn secondary", onclick: async () => { try { toast(await invoke("project_import", { path: importPath.value.trim() })); } catch (e) { toast(String(e), true); } } }, "Import"))))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Updates"),
      el("div", { class: "list" }, updRow), updLog)
  ));
}

async function checkUpdates(row, log) {
  const hint = $("update-hint");
  hint.textContent = "Checking GitHub…";
  try {
    const st = await invoke("update_check");
    hint.textContent = `${st.message} You have ${st.local} on ${st.branch}${st.behind ? `; latest is ${st.remote}` : ""}.`;
    const old = $("update-apply");
    if (old) old.remove();
    if (st.can_update)
      row.append(el("button", { class: "btn primary", id: "update-apply", onclick: async (e) => {
        e.target.disabled = true;
        log.classList.remove("hidden");
        log.textContent = "";
        try { await invoke("update_apply"); } catch (err) { log.textContent += String(err) + "\n"; e.target.disabled = false; }
      } }, "Update and Restart"));
  } catch (e) {
    hint.textContent = String(e);
  }
}
T.event.listen("update-line", (ev) => {
  const log = $("update-log");
  if (!log) return;
  log.classList.remove("hidden");
  log.textContent += ev.payload.line + "\n";
  log.scrollTop = log.scrollHeight;
  if (ev.payload.error) toast(ev.payload.line, true);
});

// =====================================================================
// Shared: project picker and a cost card for one-agent runs
// =====================================================================
state.project = "";
function studioButton() {
  return el("button", { class: "btn secondary", id: "open-studio", title: "Build this game into a place file and open it in Roblox Studio. Nothing is published.", onclick: async (ev) => {
    if (!state.project) { toast("Choose a game first.", true); return; }
    ev.target.disabled = true; ev.target.textContent = "Building...";
    try { const p = await invoke("project_open_in_studio", { project: state.project }); toast(`Opened in Studio: ${p.split(/[\\/]/).pop()}. Nothing was published.`); }
    catch (e) { toast(String(e), true); }
    ev.target.disabled = false; ev.target.textContent = "Open in Studio";
  } }, "Open in Studio");
}

async function projectPicker(onChange) {
  // The game is picked once, at the top of the sidebar.
  state.projects = await invoke("projects_list").catch(() => []);
  syncGame();
  return null;
}
async function projectPickerOld(onChange) {
  const projects = await invoke("projects_list").catch(() => []);
  if (!state.project || !projects.includes(state.project)) state.project = projects[0] || "";
  const sel = el("select", { id: "project-pick", style: "width:260px" }, projects.length ? projects.map((p) => el("option", { value: p }, p.replace("projects/", ""))) : el("option", { value: "" }, "No projects yet"));
  sel.value = state.project;
  sel.addEventListener("change", () => { state.project = sel.value; onChange(); });
  return sel;
}
async function costCard(box, agent, prompt, label) {
  dropPrepared();
  box.replaceChildren();
  let r;
  try {
    r = await invoke("run_prepare", { agent, prompt });
    state.costRun = r.run_id;
  } catch (e) {
    box.append(el("div", { class: "card", style: "margin-top:16px" }, el("div", { class: "row-title" }, "Can't start this"), el("p", { class: "note" }, String(e))));
    return;
  }
  const e = r.estimate;
  const go = el("button", { class: "btn primary", id: "approve-run", onclick: async () => {
    go.disabled = true;
    try {
      state.costRun = null;
      await invoke("run_approve_and_start", { runId: r.run_id, confirmedCapUsd: e.cap_usd });
      state.live = { kind: "run", id: r.run_id, estimate: e.expected_usd, cap: e.cap_usd };
      state.logs[r.run_id] = "";
      location.hash = "#run/" + r.run_id;
    } catch (err) { go.disabled = false; toast(String(err), true); }
  } }, e.expected_usd > 0 ? `${label} · cap ${usd(e.cap_usd)}` : `${label} · free`);
  box.append(el("div", { class: "card", style: "margin-top:16px" },
    el("div", { class: "inline", style: "justify-content:space-between" }, el("h2", { style: "margin:0" }, "Usage before you run"), el("span", { class: "pill red-soft" }, "Needs your approval")),
    el("div", { class: "cost-grid" },
      el("div", {}, el("div", { class: "k" }, "Expected"), el("div", { class: "v" }, usd(e.expected_usd, 2))),
      el("div", {}, el("div", { class: "k" }, "Hard cap"), el("div", { class: "v" }, usd(e.cap_usd)))),
    el("div", { class: "note" }, e.subscription_note),
    el("div", { class: "inline", style: "margin-top:14px" }, go,
      el("button", { class: "btn secondary", id: "cost-cancel", onclick: async () => { await invoke("run_cancel", { runId: r.run_id }).catch(() => {}); box.replaceChildren(); } }, "Cancel"))));
}

// =====================================================================
// Ask Claude
// =====================================================================
state.chatBusy = null; // run id while a reply is being written
state.chatStatus = "";
// Attachments for Ask Claude: images and documents as files, a video as
// frames (Claude reads pictures, not video), links stay in the text.
async function videoFrames(file, count = 6) {
  const url = URL.createObjectURL(file);
  const v = el("video", { muted: true, preload: "auto" }); v.src = url;
  // Some formats never load and never error: give up after a while.
  const within = (ms, p) => Promise.race([p, new Promise((_, bad) => setTimeout(() => bad(new Error(`Could not read ${file.name}. Try saving it as an MP4.`)), ms))]);
  try {
    await within(15000, new Promise((ok, bad) => { v.onloadedmetadata = ok; v.onerror = () => bad(new Error("That video format cannot be read here. Try an MP4.")); }));
  } catch (e) { URL.revokeObjectURL(url); throw e; }
  const out = [], c = document.createElement("canvas");
  const scale = Math.min(1, 1280 / (v.videoWidth || 1280));
  c.width = Math.round((v.videoWidth || 1280) * scale); c.height = Math.round((v.videoHeight || 720) * scale);
  for (let k = 0; k < count; k++) {
    const t = (v.duration || 1) * (k + 0.5) / count;
    try { await within(8000, new Promise((ok) => { v.onseeked = ok; v.currentTime = t; })); } catch (e) { if (out.length) break; URL.revokeObjectURL(url); throw e; }
    c.getContext("2d").drawImage(v, 0, 0, c.width, c.height);
    const mm = String(Math.floor(t / 60)).padStart(2, "0"), ss = String(Math.floor(t % 60)).padStart(2, "0");
    out.push({ name: `${file.name.replace(/\.[^.]+$/, "")}-frame-${k + 1}-of-${count}-at-${mm}m${ss}s.jpg`, data: c.toDataURL("image/jpeg", 0.85) });
  }
  URL.revokeObjectURL(url);
  return out;
}
const fileData = (f) => new Promise((ok, bad) => { const r = new FileReader(); r.onload = () => ok(r.result); r.onerror = bad; r.readAsDataURL(f); });
async function attachFiles(files, tray) {
  for (const f of files) {
    try {
      if (f.type.startsWith("video/")) {
        toast(`Taking frames from ${f.name}...`);
        for (const fr of await videoFrames(f)) state.chatAtts.push({ path: await invoke("chat_attach", { name: fr.name, data: fr.data }), name: fr.name, image: fr.data });
      } else {
        const data = await fileData(f);
        state.chatAtts.push({ path: await invoke("chat_attach", { name: f.name || "pasted.png", data }), name: f.name || "pasted image", image: f.type.startsWith("image/") ? data : null });
      }
    } catch (e) { toast(String(e), true); }
  }
  drawTray(tray);
}
function drawTray(tray) {
  tray.replaceChildren(...state.chatAtts.map((a, i) => el("div", { class: "att" },
    a.image ? el("img", { src: a.image, alt: a.name }) : el("span", { class: "att-name" }, a.name),
    el("button", { class: "att-x", title: "Remove", onclick: () => { state.chatAtts.splice(i, 1); drawTray(tray); } }, "×"))));
  tray.style.display = state.chatAtts.length ? "" : "none";
}
function msgAttachments(paths) {
  if (!paths || !paths.length) return null;
  return el("div", { class: "msg-atts" }, paths.map((p) => {
    const name = p.split(/[\\/]/).pop();
    if (/\.(png|jpe?g|webp|gif)$/i.test(p)) { const img = el("img", { class: "lazy", alt: name, title: name }); invoke("chat_file", { path: p }).then((d) => { img.src = d; img.onload = () => img.classList.add("in"); }).catch(() => img.remove()); return img; }
    return el("span", { class: "pill" }, name);
  }));
}

async function viewChat() {
  const nav = navSeq;
  await freshStatus();
  state.chatAtts = state.chatAtts || [];
  const [{ chat, cap_usd, engine }, models, st, ol, projects] = await Promise.all([invoke("chat_get"), invoke("models_list").catch(() => []), invoke("settings_get"), invoke("ollama_status").catch(() => null), invoke("projects_list").catch(() => [])]);
  const cur = engine !== "claude" ? `ollama:${st.ollama_model}` : `claude:${st.chat_model}`;
  const local = ol && ol.installed ? ol.installed : [];
  const modelPick = el("select", { id: "chat-model", class: "model-select", title: "Which model answers" },
    el("optgroup", { label: "Claude (your subscription)" }, el("option", { value: "claude:", selected: cur === "claude:" }, "Default"), models.map((m) => el("option", { value: `claude:${m.id}`, selected: cur === `claude:${m.id}` }, MODEL_NAMES[m.id] || m.id))),
    el("optgroup", { label: local.length ? "Free on this computer (Ollama)" : "Ollama: no local models yet (Settings)" }, local.map((m) => el("option", { value: `ollama:${m}`, selected: cur === `ollama:${m}` }, `${m} · free, local`))));
  modelPick.addEventListener("change", async () => { try { await invoke("chat_model_set", { choice: modelPick.value }); toast(modelPick.value.startsWith("ollama:") ? "Using your free local model." : "Using Claude."); } catch (e) { toast(String(e), true); } viewChat(); });
  // The game Ask Claude helps with is the one picked at the top of the sidebar.
  state.chatGame = state.game || "";
  const who = engine !== "claude" ? "Local model" : "Claude";
  const input = el("textarea", { id: "chat-input", placeholder: "Ask anything, paste a link, or drop images, videos or files here.", style: "min-height:64px" });
  const tray = el("div", { class: "att-tray", id: "chat-tray" });
  const fileIn = el("input", { type: "file", multiple: true, class: "hidden", accept: "image/*,video/*,.pdf,.txt,.md,.json,.lua,.luau,.csv,.log" });
  fileIn.addEventListener("change", () => { attachFiles([...fileIn.files], tray); fileIn.value = ""; });
  const sendText = async (text) => {
    if (!text.trim() && !state.chatAtts.length) return;
    if (!text.trim()) text = "What do you see? What should I take from this for my game?";
    send.disabled = true;
    const atts = state.chatAtts.splice(0);
    drawTray(tray);
    const empty = list.querySelector(".empty");
    if (empty) empty.remove();
    // Show it at once: your message, then a reply bubble that fills in live.
    const mine = el("div", { class: "bubble me" }, el("div", {}, text), atts.length ? el("div", { class: "msg-atts" }, atts.map((a) => a.image ? el("img", { src: a.image, alt: a.name }) : el("span", { class: "pill" }, a.name))) : null);
    const typing = el("div", { class: "bubble them typing", id: "chat-typing" }, atts.length ? `Looking at ${atts.length} attachment${atts.length > 1 ? "s" : ""}…` : "Thinking…");
    list.append(mine, typing);
    list.scrollTop = list.scrollHeight;
    input.value = ""; state.chatDraft = ""; state.chatLive = "";
    try {
      state.chatBusy = await invoke("chat_send", { text, project: state.chatGame || null, attachments: atts.map((a) => a.path) });
      state.chatStatus = "Thinking…";
    } catch (e) { mine.remove(); typing.remove(); input.value = text; state.chatAtts.push(...atts); drawTray(tray); send.disabled = false; toast(String(e), true); }
  };
  const send = el("button", { class: "btn primary", id: "chat-send", disabled: !!state.chatBusy, onclick: () => sendText(input.value) }, cap_usd > 0 ? "Send" : "Send · free");
  // Enter sends; Shift+Enter makes a new line.
  input.addEventListener("keydown", (e) => { if (e.key === "Enter" && !e.shiftKey && !e.isComposing) { e.preventDefault(); if (!send.disabled) send.click(); } });
  input.addEventListener("paste", (e) => { const fs = [...(e.clipboardData || {}).files || []]; if (fs.length) { e.preventDefault(); attachFiles(fs, tray); } });
  // One-tap actions for the chosen game.
  const quick = el("div", { class: "chips", id: "chat-quick" });
  const QUICK = [
    ["Notes for the team", (g) => `Review my game ${g} and write notes for the team on my behalf: what is working, what is not, and what to do next, most important first. Save them to projects/${g}/docs/notes-from-andrew.md so the build agents follow them.`],
    ["What should I do next?", (g) => `Look at where my game ${g} is now (brief, plan, score, latest checks) and tell me the 3 most useful next steps, in plain words.`],
    ["Is it fun yet?", (g) => `Be honest: from what is built in ${g}, what would a 10-year-old enjoy in the first minute, and what would make them leave? Short.`],
  ];
  const drawQuick = () => quick.replaceChildren(...(state.chatGame ? QUICK.map(([label, make]) => el("button", { class: "chip quick", onclick: () => !state.chatBusy && sendText(make(state.chatGame)) }, label)) : [el("span", { class: "note" }, "Pick a game at the top of the sidebar to get notes and next steps for it.")]));
  drawQuick();
  const msgs = chat.messages.map((m) =>
    el("div", { class: "bubble " + (m.role === "you" ? "me" : "them") + (m.failed ? " failed" : "") },
      m.role === "you" ? el("div", {}, m.text) : markdown(m.text),
      m.role === "you" ? msgAttachments(m.attachments) : null,
      m.role !== "you" ? el("div", { class: "bubble-meta" }, who, m.cost_usd != null ? ` · ${usd(m.cost_usd, 2)}` : "", m.run_id ? el("button", { class: "link", style: "margin-left:8px", onclick: () => (location.hash = "#run/" + m.run_id) }, "Details") : null,
        m.failed && m.run_id && /usage limit is reached/.test(m.text) && !state.chatBusy ? el("button", { class: "link chat-continue", style: "margin-left:8px", onclick: (e) => { state.chatBusy = m.run_id; continueNow(m.run_id, e.currentTarget).then(() => viewChat()); } }, "Continue Now") : null) : null));
  if (state.chatBusy) msgs.push(el("div", { class: "bubble them typing" + (state.chatLive ? " live" : ""), id: "chat-typing" }, state.chatLive || state.chatStatus || "Thinking…"));
  const list = el("div", { class: "chat-list", id: "chat-list" }, msgs.length ? msgs :
    el("div", { class: "empty", style: "padding:56px 20px" }, el("div", { class: "ic-big", html: ICONS.chat }), el("div", { class: "empty-title" }, `Ask ${who} anything`),
      el("div", {}, "Pick a game to talk about it. Drop screenshots, videos, files or links and it will look at them. Each message counts toward your plan usage.")));
  const pageEl = el("div", { class: "page chat-page" },
    head("Ask Claude", runningSubtitle(), el("div", { class: "inline" }, chat.messages.length ? el("button", { class: "btn secondary", onclick: async () => { await invoke("chat_reset"); viewChat(); } }, "New Chat") : null)),
    list,
    el("div", { class: "card chat-compose" }, quick, tray, input,
      el("div", { class: "inline", style: "justify-content:space-between;margin-top:10px" },
        el("div", { class: "inline" }, el("button", { class: "btn secondary", id: "chat-attach", title: "Images, videos, PDFs or text files", onclick: () => fileIn.click() }, "Attach"), fileIn, el("span", { class: "note" }, "Enter sends · Shift+Enter new line")),
        el("div", { class: "inline" }, modelPick, send))));
  dropZone(pageEl, (fs) => attachFiles(fs, tray), /^(image|video)\/|^application\/pdf$|^text\/|^application\/json$|^$/);
  showIf(nav, pageEl);
  drawTray(tray);
  if (state.chatDraft) input.value = state.chatDraft;
  input.addEventListener("input", () => { state.chatDraft = input.value; });
  list.scrollTop = list.scrollHeight;
  input.focus();
}
T.event.listen("chat-line", (ev) => {
  const n = $("chat-typing");
  let v; try { v = JSON.parse(ev.payload.line); } catch { return; }
  // Word-by-word text while Claude writes.
  const d = v.type === "stream_event" && v.event && v.event.delta;
  if (d && d.type === "text_delta") {
    state.chatLive = (state.chatLive || "") + d.text;
    if (n) { n.classList.add("live"); n.textContent = state.chatLive; const l = $("chat-list"); if (l && l.scrollHeight - l.scrollTop - l.clientHeight < 120) l.scrollTop = l.scrollHeight; }
    return;
  }
  if (v.type === "stream_event") return;
  const t = renderLine(ev.payload.line);
  if (!t) return;
  // Tool steps (reading a file, searching) show while no text is streaming.
  const last = t.split("\n").pop();
  if (last.startsWith("· ") || last.startsWith("Finished")) {
    state.chatStatus = last.slice(0, 140);
    if (n && !state.chatLive) n.textContent = state.chatStatus;
  }
});
T.event.listen("chat-done", () => {
  state.chatBusy = null;
  state.chatStatus = "";
  state.chatLive = "";
  refreshStatus();
  if ((location.hash || "") === "#chat") viewChat();
  else toast("Claude replied in Ask Claude.");
});

// =====================================================================
// Simulator
// =====================================================================
function ring(score, grade) {
  const r = 52, c = 2 * Math.PI * r, v = Math.max(0, Math.min(100, score));
  return el("div", { class: "ring", html: `<svg viewBox="0 0 120 120"><circle cx="60" cy="60" r="${r}" fill="none" stroke="#2C2C2E" stroke-width="9"/><circle cx="60" cy="60" r="${r}" fill="none" stroke="#FF3B30" stroke-width="9" stroke-linecap="round" stroke-dasharray="${(c * v) / 100} ${c}" transform="rotate(-90 60 60)"/><text x="60" y="58" text-anchor="middle" fill="#fff" font-size="30" font-weight="600" font-family="-apple-system,Inter,sans-serif">${Math.round(v)}</text><text x="60" y="80" text-anchor="middle" fill="#8E8E93" font-size="13" font-family="-apple-system,Inter,sans-serif">Grade ${String(grade || "").replace(/[<>&]/g, "")}</text></svg>` });
}
async function viewSimulator() {
  const nav = navSeq;
  await freshStatus();
  const pick = await projectPicker(viewSimulator);
  const box = el("div", {});
  const run = el("button", { class: "btn primary", id: "sim-run", disabled: !state.project, onclick: () => costCard(box, "simulator", `Simulate players of different ages on ${state.project}.`, "Run Simulation") }, "Simulate Players");
  let sims = [];
  if (state.project) sims = await invoke("simulations", { project: state.project }).catch(() => []);
  const s = sims[0];
  let body;
  if (!s) {
    body = el("div", { class: "empty" }, el("div", { class: "ic-big", html: ICONS.pulse }), el("div", { class: "empty-title" }, state.project ? "No simulation yet" : "Import a game first"),
      el("div", {}, state.project ? "Simulated players aged 7 to adult walk your game and say how long they would play, where they would quit, and what to fix." : "Add a game project in Settings, then simulate it here."));
  } else {
    const maxMin = Math.max(10, ...s.personas.map((p) => p.session_minutes));
    body = el("div", {},
      el("div", { class: "card sim-top" }, ring(s.rating.score, s.rating.grade),
        el("div", { class: "grow" }, el("h2", {}, "Rating"), el("p", { class: "prose", style: "margin:0 0 10px" }, s.rating.summary),
          el("div", { class: "cost-grid", style: "margin:0" },
            el("div", {}, el("div", { class: "k" }, "Typical first session"), el("div", { class: "v small" }, `${Number(s.predicted?.median_session_minutes ?? 0).toFixed(1)} min`)),
            el("div", {}, el("div", { class: "k" }, "Come back next day"), el("div", { class: "v small" }, `${Math.round(100 * (s.predicted?.day1_return_share ?? 0))}%`)),
            el("div", {}, el("div", { class: "k" }, "Roblox median"), el("div", { class: "v small" }, "9.8 min · 10%"))))),
      el("div", { class: "section" }, el("div", { class: "section-label" }, "Who stays and who leaves"),
        el("div", { class: "list" }, s.personas.map((p) =>
          el("div", { class: "srow", style: "align-items:flex-start" },
            el("div", { style: "width:120px;flex:none" }, el("div", { class: "label" }, `Age ${p.age_band}`), el("div", { class: "hint" }, p.device)),
            el("div", { class: "grow" },
              el("div", { class: "inline" }, el("div", { class: "bar", style: "flex:1;height:6px" }, el("div", { style: `width:${(100 * p.session_minutes) / maxMin}%` })), el("span", { class: "note", style: "width:64px;text-align:right" }, `${Number(p.session_minutes).toFixed(1)} min`)),
              el("div", { class: "hint", style: "margin-top:6px;white-space:normal" }, `Leaves: ${String(p.quit_point || "").replace(/[.\s]+$/, "")}. ${p.quit_reason}`)),
            el("div", { style: "flex:none;text-align:right;width:96px" }, el("span", { class: "pill " + (p.would_return === "yes" ? "white" : p.would_return === "no" ? "red-soft" : "") }, p.would_return === "yes" ? "Returns" : p.would_return === "no" ? "Gone" : "Maybe"), el("div", { class: "hint", style: "margin-top:4px" }, `Fun ${p.fun}/10`)))))),
      el("div", { class: "section" }, el("div", { class: "section-label" }, "Fix these first"),
        el("div", { class: "list" }, s.improvements.map((i, k) =>
          el("div", { class: "step" }, el("div", { class: "step-num" + (i.impact === "high" ? " on" : "") }, String(k + 1)),
            el("div", {}, el("div", { class: "inline", style: "justify-content:space-between" }, el("div", { class: "row-title" }, i.title), el("span", { class: "pill" + (i.impact === "high" ? " red" : "") }, cap(i.impact || "") + " impact")),
              el("div", { class: "note", style: "margin-top:4px" }, i.why), el("div", { style: "margin-top:6px;font-size:14px" }, i.how),
              el("button", { class: "link", style: "margin-top:8px", onclick: () => { state.composer = { ...state.composer, mode: "pipeline", pipeline: "improve-build", project: state.project, prompt: `${i.title}. ${i.how}` }; state.runsTab = "new"; location.hash = "#runs"; } }, "Have the team build this")))))),
      el("p", { class: "note", style: "margin-top:14px" }, `${s.method} Simulated ${ago(s.at)} for ${usd(s.cost_usd, 2)}.`, el("button", { class: "link", style: "margin-left:8px", onclick: () => (location.hash = "#run/" + s.run_id) }, "Details")));
  }
  showIf(nav, page(head("Simulator", runningSubtitle(), el("div", { class: "inline" }, pick, studioButton(), run)), box, body));
}

// =====================================================================
// Thumbnails
// =====================================================================
async function viewThumbnails() {
  const nav = navSeq;
  await freshStatus();
  const pick = await projectPicker(viewThumbnails);
  const box = el("div", {});
  let data = { screenshots: [], thumbnails: [], run: null };
  if (state.project) data = await invoke("thumbnails", { project: state.project }).catch(() => data);
  const make = el("button", { class: "btn primary", id: "thumb-run", disabled: !state.project, onclick: () => costCard(box, "splash", `Make the launch thumbnails and icon for ${state.project}. Follow the MrBeast rule sheet. 12 comps. Use the captures in ${state.project}/screenshots.`, "Make Thumbnails") }, "Make Thumbnails");
  const shotPath = el("input", { type: "text", id: "shot-path", placeholder: "~/Desktop/my-game-screenshot.png" });
  const shots = el("div", { class: "card" },
    el("div", { class: "inline", style: "justify-content:space-between" }, el("h2", { style: "margin:0" }, "Game screenshots"), el("span", { class: "note" }, `${data.screenshots.length} added`)),
    el("p", { class: "note" }, "Thumbnails must show your real game. Take screenshots in Roblox Studio, then add them here. Without one, you get layout sketches only and they cannot reach A+."),
    el("div", { class: "inline" }, shotPath, el("button", { class: "btn secondary", onclick: async () => { try { toast(await invoke("screenshot_add", { project: state.project, path: shotPath.value })); viewThumbnails(); } catch (e) { toast(String(e), true); } } }, "Add Screenshot")),
    data.screenshots.length ? el("div", { class: "files" }, data.screenshots.map((n) => el("span", { class: "pill" }, n))) : null);
  let grid;
  if (!data.thumbnails.length) {
    grid = el("div", { class: "empty" }, el("div", { class: "ic-big", html: ICONS.image }), el("div", { class: "empty-title" }, "No thumbnails yet"),
      el("div", {}, "One subject, one strong emotion, one prize or danger, three words or fewer. The same rules the biggest YouTube thumbnails follow, applied to your game."));
  } else {
    const cells = data.thumbnails.map((t) => {
      const im = lazyImg(t.path, { alt: t.name });
      return el("div", { class: "tcell" + (t.icon ? " icon" : "") }, im,
        el("div", { class: "tfoot" }, el("span", { class: "inline", style: "gap:6px" }, el("span", { class: "pill " + (t.grade === "A+" ? "red" : t.grade === "A" ? "white" : "") , title: t.checks.map((c) => (c.passed ? "✓ " : "✕ ") + c.name).join("\n") }, t.grade), el("span", {}, t.name)),
          el("button", { class: "link", onclick: () => exportPng(t.path, im.src).catch((e) => toast(String(e), true)) }, "Export PNG")));
    });
    const best = data.thumbnails.filter((t) => !t.icon)[0];
    grid = el("div", { class: "section" },
      el("div", { class: "inline", style: "justify-content:space-between" }, el("div", { class: "section-label" }, `Latest set · made ${ago(data.run.at)} for ${usd(data.run.cost_usd, 2)}`), el("button", { class: "link", onclick: () => (location.hash = "#run/" + data.run.run_id) }, "Ranking and details")),
      el("div", { class: "tgrid big" }, cells),
      el("p", { class: "note", style: "margin-top:12px" }, `Grades check the rules a program can measure (size, 3 words or fewer, 3 colours or fewer, outlined text, real game capture). ${best && best.grade !== "A+" ? "None is A+ yet: add a real screenshot and make a new set. " : ""}The real test is your play-through rate after launch.`));
  }
  showIf(nav, page(head("Thumbnails", runningSubtitle(), el("div", { class: "inline" }, pick, studioButton(), make)), box, shots, grid));
}

document.querySelectorAll("[data-icon]").forEach((n) => (n.innerHTML = ICONS[n.dataset.icon]));

// ---------- start ----------
refreshClaude();
refreshStatus();
route();
invoke("studio_info").then((i) => { const v = $("app-version"); if (v && i.build) v.textContent = i.build.startsWith("v") ? i.build : ""; }).catch(() => {});
