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
function show(node) {
  $("view").replaceChildren(node);
  $("view").scrollTop = 0;
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
async function refreshStatus() {
  try {
    state.spend = await invoke("spend");
  } catch {
    state.spend = null;
  }
  const sp = state.spend;
  if (sp) {
    const used = sp.spent_usd + sp.reserved_usd;
    $("spend-line").textContent = sp.budget_usd > 0 ? `This month ${usd(sp.spent_usd)} of ${usd(sp.budget_usd)}` : `This month ${usd(sp.spent_usd)} (no budget)`;
    $("spend-bar").style.width = sp.budget_usd > 0 ? Math.min(100, (100 * used) / sp.budget_usd) + "%" : "0%";
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
setInterval(refreshStatus, 5000);

function runningSubtitle() {
  const sp = state.spend;
  const run = state.live ? state.live.id : sp && sp.running.length ? sp.running[0] : null;
  return el("span", {},
    "BrewixGames · ",
    run ? el("span", {}, "Current run ", el("b", {}, run), ` · est. ${usd(state.live ? state.live.estimate : sp.running_expected_usd, 4)}`) : "No run in progress",
    sp ? ` · ${usd(sp.spent_usd)} spent this month` + (sp.budget_usd > 0 ? ` of ${usd(sp.budget_usd)}` : "") : ""
  );
}

// ---------- router ----------
const routes = { chat: viewChat, simulator: viewSimulator, thumbnails: viewThumbnails, queue: viewQueue, agents: viewAgents, vault: viewVault, runs: viewRuns, run: viewRunDetail, settings: viewSettings };
function route() {
  const h = location.hash.replace(/^#/, "") || "queue";
  const [name, ...rest] = h.split("/");
  const fn = routes[name] || viewQueue;
  const navName = name === "run" ? "runs" : routes[name] ? name : "queue";
  document.querySelectorAll("#nav a").forEach((a) => a.classList.toggle("active", a.dataset.route === navName));
  closeSheet();
  Promise.resolve(fn(decodeURIComponent(rest.join("/")))).catch((e) => show(page(head("Something went wrong"), el("div", { class: "card" }, String(e)))));
}
window.addEventListener("hashchange", route);

// =====================================================================
// 1. Review queue
// =====================================================================
async function viewQueue() {
  await refreshStatus();
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
  show(page(head("Review Queue", runningSubtitle(), right), body));
}

async function thumbFor(item) {
  const img = (item.artifacts || []).find((a) => /\.(svg|png|jpe?g)$/.test(a));
  if (img) {
    try {
      return el("div", { class: "thumb" }, el("img", { src: await invoke("artifact_image", { path: img }), alt: "" }));
    } catch {}
  }
  return el("div", { class: "thumb", html: ICONS[KIND_ICON[item.kind] || "doc"] });
}

function statusPill(item) {
  if (item.status === "approved") return el("span", { class: "pill white" }, "Approved");
  if (item.status === "rejected") return el("span", { class: "pill" }, "Rejected");
  return item.gate.passed ? el("span", { class: "pill red-soft" }, "Pending") : el("span", { class: "pill red" }, "Gate failed");
}

async function queueCard(item) {
  const by = item.created_by.startsWith("pipeline:") ? `Pipeline ${item.created_by.slice(9)}` : cap(item.created_by);
  const sentences = (item.summary || "").split(/(?<=\.)\s/);
  const firstLine = sentences.find((x) => x.startsWith("Stages:")) || sentences[0];
  const actions = item.status === "pending"
    ? el("div", { class: "qactions" },
        el("button", { class: "btn primary", disabled: !item.gate.passed, title: item.gate.passed ? "Record your approval. Nothing is published." : "A failed gate cannot be approved. Reject it and rerun.", onclick: () => decide(item, "approved") }, "Approve"),
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
  try {
    await invoke("queue_decide", { id: item.id, verdict, note });
    toast(verdict === "approved" ? "Approved. Nothing was published." : "Rejected.");
    closeSheet();
    viewQueue();
  } catch (e) {
    toast(String(e), true);
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
    ? el("div", { class: "tgrid" }, await Promise.all(images.map(async (p) => {
        let src = "";
        try { src = await invoke("artifact_image", { path: p }); } catch {}
        return el("div", { class: "tcell" }, el("img", { src, alt: p }),
          el("div", { class: "tfoot" }, el("span", {}, p.split("/").pop()), p.endsWith(".svg") ? el("button", { class: "link", onclick: () => exportPng(p, src).catch((e) => toast(String(e), true)) }, "Export PNG") : null));
      })))
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

async function itemSheet(item) {
  const note = el("input", { type: "text", placeholder: "Note (optional): why yes or no" });
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
  waiting: () => el("span", { class: "pill red-soft" }, "Waiting for approval"),
  blocked: () => el("span", { class: "pill white" }, "Blocked"),
};
async function viewAgents() {
  await refreshStatus();
  const agents = await invoke("agents_status");
  const working = agents.filter((a) => a.status === "working").length;
  show(page(
    head("Agents", el("span", {}, `${agents.length} agents · ${working ? working + " working" : "none working"} · `, runningSubtitle())),
    el("div", { class: "list", id: "agent-list" }, agents.map((a) =>
      el("div", { class: "row tap" + (a.status === "working" ? " active-agent" : ""), "data-agent": a.name, onclick: () => agentSheet(a) },
        el("div", { class: "grow" }, el("div", { class: "row-title" }, cap(a.name)), el("div", { class: "row-sub" }, a.description)),
        AGENT_STATUS[a.status](),
        el("span", { class: "chev", html: ICONS.chev })
      )
    ))
  ));
}
function agentSheet(a) {
  sheet(cap(a.name),
    el("p", { class: "prose" }, a.description),
    el("div", { class: "list" },
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Status"), AGENT_STATUS[a.status]()),
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
async function viewVault() {
  await refreshStatus();
  const tree = await invoke("vault_tree");
  if (!state.vaultSection) state.vaultSection = tree[0] && tree[0].name;
  const sec = tree.find((s) => s.name === state.vaultSection) || tree[0];
  const total = tree.reduce((n, s) => n + s.notes.length, 0);
  const verified = tree.reduce((n, s) => n + s.notes.filter((x) => x.status === "verified").length, 0);
  show(page(
    head("Vault", el("span", {}, `${total} notes · ${verified} verified · agents load one section at a time · `, runningSubtitle())),
    el("div", { class: "vault" },
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
}

// =====================================================================
// 4. Runs: history, new run, run detail
// =====================================================================
async function viewRuns() {
  await refreshStatus();
  const tabs = segmented([["history", "History"], ["new", "New Run"]], state.runsTab, (v) => { state.runsTab = v; viewRuns(); });
  if (state.runsTab === "new") return viewComposer(tabs);
  const list = await invoke("runs_overview");
  show(page(
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
  const label = { done: "Done", running: "Running", failed: "Failed", stopped: "Stopped", approved: "Starting", "awaiting-approval": "Needs approval" }[s] || cap(s);
  const cls = s === "running" || s === "approved" ? "pill red" : s === "failed" || s === "stopped" ? "pill red-soft" : s === "done" ? "pill white" : "pill";
  return el("span", { class: cls }, label);
}

async function viewComposer(tabs) {
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
  const form = el("div", { class: "card" },
    el("div", { style: "margin-bottom:16px" }, segmented([["agent", "One Agent"], ["pipeline", "Pipeline"]], c.mode, (v) => { c.mode = v; viewRuns(); })),
    c.mode === "agent"
      ? el("div", { class: "field" }, el("label", { for: "agent-select" }, "Agent"), agentSel, el("div", { class: "note" }, agents[c.agent].description + " · " + agents[c.agent].model))
      : el("div", {},
          el("div", { class: "field" }, el("label", { for: "pipeline-select" }, "Pipeline"), pipeSel, el("div", { class: "note" }, pdef.description + " Stages: " + pdef.stages.map((s) => cap(s.agent)).join(", "))),
          pdef.needs_project ? el("div", { class: "field" }, el("label", { for: "project-select" }, "Game project"), projSel, projects.length ? null : el("div", { class: "note" }, "No projects yet. Import one in Settings.")) : null),
    el("div", { class: "field" }, el("label", { for: "prompt" }, "Instruction"), prompt),
    el("div", { class: "inline" }, el("button", { class: "btn primary", id: "estimate-btn", onclick: () => estimate(estimateBox) }, "Estimate Cost"), el("span", { class: "note" }, "Nothing runs until you approve the cost."))
  );
  show(page(head("Runs", runningSubtitle(), tabs), form, estimateBox));
}

async function estimate(box) {
  const c = state.composer;
  box.replaceChildren();
  try {
    if (c.mode === "agent") {
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
    el("div", { class: "inline", style: "justify-content:space-between" }, el("h2", { style: "margin:0" }, "Cost before you run"), el("span", { class: "pill red-soft" }, "Needs your approval")),
    el("div", { class: "cost-grid" },
      el("div", {}, el("div", { class: "k" }, "Expected"), el("div", { class: "v" }, usd(p.expected, 4))),
      el("div", {}, el("div", { class: "k" }, p.low !== null ? "Range" : "Up to"), el("div", { class: "v small" }, p.low !== null ? `${usd(p.low, 4)} to ${usd(p.high, 4)}` : usd(p.high, 4))),
      el("div", {}, el("div", { class: "k" }, "Hard cap"), el("div", { class: "v" }, usd(p.cap))),
      sp && sp.budget_usd > 0 ? el("div", {}, el("div", { class: "k" }, "Budget left this month"), el("div", { class: "v small" }, usd(Math.max(0, sp.budget_usd - sp.spent_usd - sp.reserved_usd)))) : null),
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
    if (p.kind === "run") await invoke("run_approve_and_start", { runId: p.id, confirmedCapUsd: p.cap });
    else await invoke("pipeline_approve_and_start", { pipelineId: p.id, confirmedCapUsd: p.cap });
    state.live = { kind: p.kind, id: p.id, estimate: p.expected, cap: p.cap };
    state.logs[p.id] = "";
    state.prepared = null;
    state.composer.prompt = "";
    location.hash = "#run/" + p.id;
  } catch (e) {
    btn.disabled = false;
    toast(String(e), true);
  }
}

async function viewRunDetail(id) {
  await refreshStatus();
  let d;
  try {
    d = await invoke("run_detail", { id });
  } catch (e) {
    return show(page(el("button", { class: "back", onclick: () => (location.hash = "#runs") }, "‹ Runs"), head("Run not found"), el("div", { class: "card" }, String(e))));
  }
  const live = state.live && state.live.id === id;
  const activeIdx = d.steps.findIndex((s) => s.status === "running" || s.status === "approved");
  show(page(
    el("button", { class: "back", onclick: () => (location.hash = "#runs") }, "‹ Runs"),
    head(cap(d.title), el("span", {}, `${d.kind === "pipeline" ? "Pipeline" : "Single agent"} · started ${ago(d.created_at)}`), runPill(d.status), true),
    el("div", { class: "card" },
      el("div", { class: "cost-grid", style: "margin:0" },
        el("div", {}, el("div", { class: "k" }, "Estimated before"), el("div", { class: "v" }, usd(d.estimate_usd, 4))),
        el("div", {}, el("div", { class: "k" }, "Hard cap"), el("div", { class: "v small" }, usd(d.cap_usd))),
        el("div", {}, el("div", { class: "k" }, "Actual after"), el("div", { class: "v", id: "actual-cost" }, usd(d.actual_usd, 4))),
        d.queue_item ? el("div", {}, el("div", { class: "k" }, "Result"), el("button", { class: "link", style: "padding-top:8px", onclick: () => (location.hash = "#queue") }, "In the review queue")) : null)
    ),
    el("div", { class: "section" }, el("div", { class: "section-label" }, "Steps"),
      el("div", { class: "list steps" }, d.steps.map((s, i) =>
        el("div", { class: "step" },
          el("div", { class: "step-num" + (i === activeIdx ? " on" : "") }, String(i + 1)),
          el("div", {},
            el("div", { class: "inline", style: "justify-content:space-between" }, el("div", { class: "row-title" }, cap(s.agent)), runPill(s.status)),
            el("div", { class: "note" }, `est. ${usd(s.estimate_usd, 4)} · cap ${usd(s.cap_usd)} · actual ${usd(s.actual_usd, 4)}`),
            el("div", { class: "files" }, s.output ? el("button", { class: "link", onclick: () => openText(s.output) }, "Open " + s.output.split("/").pop()) : el("span", { class: "note" }, "Nothing produced yet"),
              s.loaded.length ? el("span", { class: "note" }, `Read ${s.loaded.length} vault file${s.loaded.length === 1 ? "" : "s"}: ` + s.loaded.map((x) => x.split("/").slice(-2).join("/")).slice(0, 6).join(", ") + (s.loaded.length > 6 ? "…" : "")) : null),
            s.result_text ? el("details", { style: "margin-top:8px" }, el("summary", { class: "note", style: "cursor:pointer" }, "What the agent said"), el("div", { class: "log", style: "margin-top:6px" }, s.result_text)) : null,
            s.gate ? el("div", {}, el("div", { class: "note", style: "margin-top:8px" }, s.gate.passed ? "Verified: all checks passed" : "Verification: some checks failed"), checksList(s.gate.checks)) : null
          )
        )))),
    live || d.status === "running" ? el("div", { class: "section" }, el("div", { class: "section-label" }, "Live output"), el("pre", { class: "log", id: "live-log" }, state.logs[id] || "Starting…")) : null
  ));
}

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
function appendLog(id, text) {
  state.logs[id] = (state.logs[id] || "") + text + "\n";
  const log = $("live-log");
  if (log && location.hash === "#run/" + id) {
    log.textContent = state.logs[id];
    log.scrollTop = log.scrollHeight;
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
async function viewSettings() {
  await refreshStatus();
  const [s, info] = await Promise.all([invoke("settings_get"), invoke("studio_info")]);
  const c = state.claude || {};
  const save = async (patch, msg) => {
    try {
      const next = await invoke("settings_save", { value: { ...s, ...patch, notify: { ...s.notify, ...(patch.notify || {}) } } });
      Object.assign(s, next);
      toast(msg || "Saved.");
      refreshStatus();
    } catch (e) {
      toast(String(e), true);
    }
  };
  const budget = el("input", { type: "number", min: "0", step: "1", id: "budget", value: String(s.monthly_budget_usd) });
  const hook = el("input", { type: "password", id: "webhook", placeholder: "https://discord.com/api/webhooks/…", value: s.discord_webhook_url });
  const root = el("input", { type: "text", id: "studio-root", placeholder: "Auto-detected", value: s.studio_root });
  const importPath = el("input", { type: "text", id: "import-path", placeholder: "~/workspace/roblox-games/sky-tower-obby" });
  const updLog = el("pre", { class: "log hidden", id: "update-log", style: "margin-top:12px" });
  const updRow = el("div", { class: "srow", id: "update-row" }, el("div", { class: "grow" }, el("div", { class: "label", id: "build-label" }, `Build ${info.build}`), el("div", { class: "hint", id: "update-hint" }, "Downloads the newest BrewixGames, installs it, and restarts. Your projects and review queue are kept.")),
    el("button", { class: "btn secondary", id: "update-check", onclick: () => checkUpdates(updRow, updLog) }, "Check for Updates"));
  const sp = state.spend;
  show(page(
    head("Settings", runningSubtitle()),

    el("div", { class: "section-label" }, "Claude"),
    el("div", { class: "list" },
      el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, c.logged_in ? "Connected" : "Not connected"), el("div", { class: "hint", id: "claude-msg" }, c.message || "Checking…")),
        el("button", { class: "btn secondary", onclick: async () => { await refreshClaude(); viewSettings(); } }, "Check Again"),
        !c.installed ? el("button", { class: "btn primary", id: "install-claude", onclick: async (e) => { e.target.disabled = true; e.target.textContent = "Installing…"; try { toast(await invoke("claude_install")); } catch (err) { toast(String(err), true); } await refreshClaude(); viewSettings(); } }, "Install Claude Code") : null,
        c.installed && !(c.logged_in && c.uses_subscription) ? el("button", { class: "btn primary", onclick: async (e) => { e.target.disabled = true; e.target.textContent = "Waiting for browser…"; try { toast(await invoke("claude_sign_in")); } catch (err) { toast(String(err), true); } await refreshClaude(); viewSettings(); } }, "Sign In to Claude") : null),
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Login method"), el("span", { class: "note" }, c.auth_method ? (c.uses_subscription ? "Claude account login · uses your subscription, no API key" : c.auth_method) : "–")),
      el("div", { class: "srow" }, el("div", { class: "grow label" }, "Claude Code"), el("span", { class: "note" }, c.version ? `${c.version} · ${c.bin}` : "–"))
    ),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "AI engine"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Who does the thinking"), el("div", { class: "hint" }, s.engine === "ollama" ? "Local model through Ollama: free and private, but weaker than Claude. Expect more failed checks." : "Claude, through your Claude subscription. Best quality.")),
          segmented([["claude", "Claude"], ["ollama", "Local (free)"]], s.engine, async (v) => { await save({ engine: v, ollama_model: $("ollama-model").value.trim() }, v === "ollama" ? "Using the local model." : "Using Claude."); viewSettings(); })),
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Local model"), el("div", { class: "hint" }, "Needs Ollama 0.14 or newer running on this computer, and the model already downloaded in Ollama.")),
          el("div", { class: "inline", style: "width:320px" }, el("input", { type: "text", id: "ollama-model", placeholder: "for example qwen3-coder", value: s.ollama_model }), el("button", { class: "btn secondary", onclick: () => save({ ollama_model: $("ollama-model").value.trim() }, "Local model saved.") }, "Save"))))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Budget"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Monthly budget (USD)"), el("div", { class: "hint" }, "Runs stop and ask once this month's spend would pass this. 0 means no budget.")),
          el("div", { class: "inline", style: "width:220px" }, budget, el("button", { class: "btn secondary", id: "save-budget", onclick: () => save({ monthly_budget_usd: Number(budget.value) }, "Budget saved.") }, "Save"))),
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "This month"), el("div", { class: "bar", style: "margin-top:8px" }, el("div", { style: `width:${sp && sp.budget_usd > 0 ? Math.min(100, (100 * (sp.spent_usd + sp.reserved_usd)) / sp.budget_usd) : 0}%` }))),
          el("span", { class: "note" }, sp ? `${usd(sp.spent_usd)} spent · ${usd(sp.reserved_usd)} reserved` : "–")))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Notifications"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Discord webhook"), el("div", { class: "hint" }, "Stored on this computer only, never in the repo.")),
          el("div", { class: "inline", style: "width:430px" }, hook,
            el("button", { class: "btn secondary", id: "save-webhook", onclick: () => save({ discord_webhook_url: hook.value.trim() }, hook.value.trim() ? "Webhook saved." : "Webhook removed.") }, "Save"),
            el("button", { class: "btn secondary", id: "send-test", onclick: async () => { try { toast(await invoke("notify_test")); } catch (e) { toast(String(e), true); } } }, "Send Test"))),
        [["needs_approval", "Something needs my approval"], ["finished", "A build or run finished"], ["failed", "A run failed or stopped"], ["budget", "The budget stopped a run"], ["updates", "An app update was installed"]].map(([k, label]) =>
          el("div", { class: "srow" }, el("div", { class: "grow label" }, label), sw(s.notify[k], (v) => save({ notify: { [k]: v } }), "n-" + k))))),

    el("div", { class: "section" }, el("div", { class: "section-label" }, "Studio"),
      el("div", { class: "list" },
        el("div", { class: "srow" }, el("div", { class: "grow" }, el("div", { class: "label" }, "Studio folder"), el("div", { class: "hint" }, `In use: ${info.root}. Vault: ${info.vault}.`)),
          el("div", { class: "inline", style: "width:430px" }, root, el("button", { class: "btn secondary", onclick: () => save({ studio_root: root.value.trim() }, "Studio folder saved. Restart to apply.") }, "Save"))),
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
async function projectPicker(onChange) {
  const projects = await invoke("projects_list").catch(() => []);
  if (!state.project || !projects.includes(state.project)) state.project = projects[0] || "";
  const sel = el("select", { id: "project-pick", style: "width:260px" }, projects.length ? projects.map((p) => el("option", { value: p }, p.replace("projects/", ""))) : el("option", { value: "" }, "No projects yet"));
  sel.value = state.project;
  sel.addEventListener("change", () => { state.project = sel.value; onChange(); });
  return sel;
}
async function costCard(box, agent, prompt, label) {
  box.replaceChildren();
  let r;
  try {
    r = await invoke("run_prepare", { agent, prompt });
  } catch (e) {
    box.append(el("div", { class: "card", style: "margin-top:16px" }, el("div", { class: "row-title" }, "Can't start this"), el("p", { class: "note" }, String(e))));
    return;
  }
  const e = r.estimate;
  const go = el("button", { class: "btn primary", id: "approve-run", onclick: async () => {
    go.disabled = true;
    try {
      await invoke("run_approve_and_start", { runId: r.run_id, confirmedCapUsd: e.cap_usd });
      state.live = { kind: "run", id: r.run_id, estimate: e.expected_usd, cap: e.cap_usd };
      state.logs[r.run_id] = "";
      location.hash = "#run/" + r.run_id;
    } catch (err) { go.disabled = false; toast(String(err), true); }
  } }, e.expected_usd > 0 ? `${label} · cap ${usd(e.cap_usd)}` : `${label} · free`);
  box.append(el("div", { class: "card", style: "margin-top:16px" },
    el("div", { class: "inline", style: "justify-content:space-between" }, el("h2", { style: "margin:0" }, "Cost before you run"), el("span", { class: "pill red-soft" }, "Needs your approval")),
    el("div", { class: "cost-grid" },
      el("div", {}, el("div", { class: "k" }, "Expected"), el("div", { class: "v" }, usd(e.expected_usd, 2))),
      el("div", {}, el("div", { class: "k" }, "Hard cap"), el("div", { class: "v" }, usd(e.cap_usd)))),
    el("div", { class: "note" }, e.subscription_note),
    el("div", { class: "inline", style: "margin-top:14px" }, go,
      el("button", { class: "btn secondary", onclick: async () => { await invoke("run_cancel", { runId: r.run_id }).catch(() => {}); box.replaceChildren(); } }, "Cancel"))));
}

// =====================================================================
// Ask Claude
// =====================================================================
state.chatBusy = null; // run id while a reply is being written
state.chatStatus = "";
async function viewChat() {
  await refreshStatus();
  const { chat, cap_usd, engine } = await invoke("chat_get");
  const who = engine === "ollama" ? "Local model" : "Claude";
  const input = el("textarea", { id: "chat-input", placeholder: "Ask a question, or say what to fix.", style: "min-height:64px" });
  const send = el("button", { class: "btn primary", id: "chat-send", disabled: !!state.chatBusy, onclick: async () => {
    const text = input.value;
    send.disabled = true;
    try {
      state.chatBusy = await invoke("chat_send", { text });
      state.chatStatus = "Thinking…";
      viewChat();
    } catch (e) { send.disabled = false; toast(String(e), true); }
  } }, cap_usd > 0 ? `Send · up to ${usd(cap_usd)}` : "Send · free");
  input.addEventListener("keydown", (e) => { if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) send.click(); });
  const msgs = chat.messages.map((m) =>
    el("div", { class: "bubble " + (m.role === "you" ? "me" : "them") + (m.failed ? " failed" : "") },
      m.role === "you" ? el("div", {}, m.text) : markdown(m.text),
      m.role !== "you" ? el("div", { class: "bubble-meta" }, who, m.cost_usd != null ? ` · ${usd(m.cost_usd, 2)}` : "", m.run_id ? el("button", { class: "link", style: "margin-left:8px", onclick: () => (location.hash = "#run/" + m.run_id) }, "Details") : null) : null));
  if (state.chatBusy) msgs.push(el("div", { class: "bubble them typing", id: "chat-typing" }, state.chatStatus || "Thinking…"));
  const list = el("div", { class: "chat-list", id: "chat-list" }, msgs.length ? msgs :
    el("div", { class: "empty", style: "padding:64px 20px" }, el("div", { class: "ic-big", html: ICONS.chat }), el("div", { class: "empty-title" }, `Ask ${who} anything`),
      el("div", {}, "It can answer questions about your games and fix things in your projects. Each message counts toward your budget.")));
  show(el("div", { class: "page chat-page" },
    head("Ask Claude", runningSubtitle(), chat.messages.length ? el("button", { class: "btn secondary", onclick: async () => { await invoke("chat_reset"); viewChat(); } }, "New Chat") : null),
    list,
    el("div", { class: "card chat-compose" }, input, el("div", { class: "inline", style: "justify-content:space-between;margin-top:10px" }, el("span", { class: "note" }, "Send is your approval for that message. ⌘ Return to send."), send))));
  list.scrollTop = list.scrollHeight;
  input.focus();
}
T.event.listen("chat-line", (ev) => {
  const t = renderLine(ev.payload.line);
  if (!t) return;
  state.chatStatus = t.split("\n").pop().slice(0, 140);
  const n = $("chat-typing");
  if (n) n.textContent = state.chatStatus;
});
T.event.listen("chat-done", () => {
  state.chatBusy = null;
  state.chatStatus = "";
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
  await refreshStatus();
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
  show(page(head("Simulator", runningSubtitle(), el("div", { class: "inline" }, pick, run)), box, body));
}

// =====================================================================
// Thumbnails
// =====================================================================
async function viewThumbnails() {
  await refreshStatus();
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
    const cells = await Promise.all(data.thumbnails.map(async (t) => {
      let src = "";
      try { src = await invoke("artifact_image", { path: t.path }); } catch {}
      return el("div", { class: "tcell" + (t.icon ? " icon" : "") }, el("img", { src, alt: t.name }),
        el("div", { class: "tfoot" }, el("span", { class: "inline", style: "gap:6px" }, el("span", { class: "pill " + (t.grade === "A+" ? "red" : t.grade === "A" ? "white" : "") , title: t.checks.map((c) => (c.passed ? "✓ " : "✕ ") + c.name).join("\n") }, t.grade), el("span", {}, t.name)),
          el("button", { class: "link", onclick: () => exportPng(t.path, src).catch((e) => toast(String(e), true)) }, "Export PNG")));
    }));
    const best = data.thumbnails.filter((t) => !t.icon)[0];
    grid = el("div", { class: "section" },
      el("div", { class: "inline", style: "justify-content:space-between" }, el("div", { class: "section-label" }, `Latest set · made ${ago(data.run.at)} for ${usd(data.run.cost_usd, 2)}`), el("button", { class: "link", onclick: () => (location.hash = "#run/" + data.run.run_id) }, "Ranking and details")),
      el("div", { class: "tgrid big" }, cells),
      el("p", { class: "note", style: "margin-top:12px" }, `Grades check the rules a program can measure (size, 3 words or fewer, 3 colours or fewer, outlined text, real game capture). ${best && best.grade !== "A+" ? "None is A+ yet: add a real screenshot and make a new set. " : ""}The real test is your play-through rate after launch.`));
  }
  show(page(head("Thumbnails", runningSubtitle(), el("div", { class: "inline" }, pick, make)), box, shots, grid));
}

document.querySelectorAll("[data-icon]").forEach((n) => (n.innerHTML = ICONS[n.dataset.icon]));

// ---------- start ----------
refreshClaude();
refreshStatus();
route();
