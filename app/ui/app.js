// Robux Studio front end. Plain JS, no build step.
// All logic that matters (approval, caps, gates) lives in Rust; this file only
// shows state and forwards clicks.

const T = window.__TAURI__;
const invoke = (cmd, args) => T.core.invoke(cmd, args);
const $ = (id) => document.getElementById(id);

function el(tag, attrs = {}, ...kids) {
  const n = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (k === "class") n.className = v;
    else if (k.startsWith("on")) n.addEventListener(k.slice(2), v);
    else n.setAttribute(k, v);
  }
  for (const k of kids.flat()) {
    if (k == null) continue;
    n.append(typeof k === "string" || typeof k === "number" ? document.createTextNode(String(k)) : k);
  }
  return n;
}
const usd = (x, d = 2) => (x == null ? "n/a" : "$" + Number(x).toFixed(d));

// ---------- tabs ----------
document.querySelectorAll("nav button").forEach((b) =>
  b.addEventListener("click", () => {
    document.querySelectorAll("nav button").forEach((x) => x.classList.toggle("active", x === b));
    document.querySelectorAll(".tab").forEach((t) => t.classList.toggle("active", t.id === "tab-" + b.dataset.tab));
    if (b.dataset.tab === "queue") loadQueue();
    if (b.dataset.tab === "runs") loadRuns();
  })
);

function showModal(title, text) {
  $("modal-title").textContent = title;
  $("modal-text").textContent = text;
  $("modal").classList.remove("hidden");
}
$("modal-close").addEventListener("click", () => $("modal").classList.add("hidden"));

// ---------- environment ----------
async function loadInfo() {
  const info = await invoke("studio_info");
  const env = $("env");
  env.replaceChildren();
  if (info.root_error) env.append(el("div", { class: "bad" }, info.root_error));
  else env.append(el("div", {}, "Studio: " + info.root));
  if (info.claude_error) env.append(el("div", { class: "bad" }, info.claude_error));
  else env.append(el("div", {}, "Claude Code: " + info.claude_bin));
}

// ---------- review queue ----------
async function loadQueue() {
  const list = $("queue-list");
  list.replaceChildren();
  let items;
  try {
    items = await invoke("queue_list");
  } catch (e) {
    list.append(el("div", { class: "card err" }, String(e)));
    return;
  }
  const pending = items.filter((i) => i.status === "pending").length;
  $("pending-count").textContent = pending ? String(pending) : "";
  if (!items.length) {
    list.append(el("div", { class: "empty", id: "queue-empty" }, "Nothing is waiting for your review."));
    return;
  }
  for (const it of items) list.append(queueCard(it));
}

function queueCard(it) {
  const note = el("input", { type: "text", placeholder: "Note (optional): why yes or no" });
  const decide = async (verdict) => {
    try {
      await invoke("queue_decide", { id: it.id, verdict, note: note.value });
      loadQueue();
    } catch (e) {
      alert(String(e));
    }
  };
  const checks = el(
    "ul",
    { class: "checks" },
    (it.gate.checks || []).map((c) => el("li", { class: c.passed ? "pass" : "fail" }, c.name + (c.detail ? ": " + c.detail : "")))
  );
  const arts = el(
    "div",
    { class: "artifacts" },
    (it.artifacts || []).map((p) =>
      el("a", {
        onclick: async () => {
          try {
            showModal(p, await invoke("queue_artifact", { id: it.id, path: p }));
          } catch (e) {
            showModal(p, "Could not open: " + e);
          }
        },
      }, p)
    )
  );
  const card = el(
    "div",
    { class: "card" },
    el(
      "div",
      { class: "item-head" },
      el("span", { class: "item-title" }, it.title),
      el("span", { class: "pill " + it.status }, it.status),
      el("span", { class: "pill" }, it.kind),
      el("span", { class: "pill " + (it.gate.passed ? "pass" : "fail") }, it.gate.passed ? "gate passed" : "gate failed")
    ),
    el("div", { class: "meta" }, `by ${it.created_by} · ${it.created_at}` + (it.run_id ? ` · ${it.run_id}` : "")),
    it.summary ? el("div", {}, it.summary) : null,
    checks,
    arts
  );
  if (it.status === "pending") {
    const approve = el("button", { class: "approve", onclick: () => decide("approved") }, "Approve");
    if (!it.gate.passed) {
      approve.disabled = true;
      approve.title = "A failed gate cannot be approved. Reject and rerun.";
    }
    card.append(el("div", { class: "actions" }, note, approve, el("button", { class: "reject", onclick: () => decide("rejected") }, "Reject")));
  } else if (it.decision) {
    card.append(el("div", { class: "meta" }, `${it.decision.verdict} by ${it.decision.by} at ${it.decision.at}` + (it.decision.note ? ` · "${it.decision.note}"` : "")));
  }
  return card;
}
$("refresh-queue").addEventListener("click", loadQueue);

// ---------- run an agent ----------
let agents = {};
let current = null; // prepared RunRequest

async function loadAgents() {
  agents = await invoke("agents_list");
  const sel = $("agent-select");
  sel.replaceChildren(...Object.keys(agents).map((k) => el("option", { value: k }, k)));
  showAgentDesc();
}
function showAgentDesc() {
  const a = agents[$("agent-select").value];
  $("agent-desc").textContent = a ? `${a.description} (model: ${a.model})` : "";
}
$("agent-select").addEventListener("change", showAgentDesc);

$("estimate-btn").addEventListener("click", async () => {
  const box = $("estimate");
  box.classList.remove("hidden");
  box.replaceChildren();
  try {
    current = await invoke("run_prepare", { agent: $("agent-select").value, prompt: $("prompt").value });
  } catch (e) {
    box.append(el("div", { class: "err" }, String(e)));
    return;
  }
  const e = current.estimate;
  const approveBtn = el("button", { class: "approve", onclick: () => approveAndRun(approveBtn, cancelBtn) }, `Approve and run (cap ${usd(e.cap_usd)})`);
  const cancelBtn = el("button", {
    class: "ghost",
    onclick: async () => {
      await invoke("run_cancel", { runId: current.run_id }).catch(() => {});
      box.classList.add("hidden");
      current = null;
    },
  }, "Cancel");
  box.append(
    el("div", { class: "item-head" }, el("span", { class: "item-title" }, "Cost before you run"), el("span", { class: "pill pending" }, "awaiting your approval")),
    el(
      "div",
      { class: "est-grid" },
      el("div", {}, el("span", {}, "Expected"), el("div", { class: "money" }, usd(e.expected_usd, 4))),
      el("div", {}, el("span", {}, "Range"), el("div", {}, `${usd(e.low_usd, 4)} to ${usd(e.high_usd, 4)}`)),
      el("div", {}, el("span", {}, "Hard cap"), el("div", { class: "money" }, usd(e.cap_usd)))
    ),
    el("ul", { class: "checks" }, e.assumptions.map((a) => el("li", {}, a))),
    el("div", { class: "meta" }, `Prices checked ${e.pricing_checked}. ${e.subscription_note}`),
    el("div", { class: "actions" }, approveBtn, cancelBtn)
  );
});

async function approveAndRun(approveBtn, cancelBtn) {
  approveBtn.disabled = true;
  cancelBtn.disabled = true;
  $("output-wrap").classList.remove("hidden");
  $("output").textContent = "";
  $("run-summary").classList.add("hidden");
  try {
    await invoke("run_approve_and_start", { runId: current.run_id, confirmedCapUsd: current.estimate.cap_usd });
    $("output").textContent = `Approved at cap ${usd(current.estimate.cap_usd)}. Running ${current.run_id}...\n`;
  } catch (e) {
    $("output").textContent = "Could not start: " + e;
    approveBtn.disabled = false;
    cancelBtn.disabled = false;
  }
}

function renderStreamLine(line) {
  let v;
  try {
    v = JSON.parse(line);
  } catch {
    return line;
  }
  if (v.type === "assistant" && v.message && Array.isArray(v.message.content)) {
    return v.message.content
      .map((c) => {
        if (c.type === "text") return c.text;
        if (c.type === "tool_use") {
          const t = c.input && (c.input.file_path || c.input.pattern || c.input.skill || c.input.command || "");
          return `[tool] ${c.name} ${t}`;
        }
        return null;
      })
      .filter(Boolean)
      .join("\n");
  }
  if (v.type === "result") return `[done] ${v.subtype} · cost ${usd(v.total_cost_usd, 4)} · turns ${v.num_turns}`;
  return null;
}

T.event.listen("run-line", (ev) => {
  if (!current || ev.payload.run_id !== current.run_id) return;
  const text = renderStreamLine(ev.payload.line);
  if (text) {
    const out = $("output");
    out.textContent += text + "\n";
    out.scrollTop = out.scrollHeight;
  }
});

T.event.listen("run-done", async (ev) => {
  const p = ev.payload;
  if (!current || p.run_id !== current.run_id) return;
  const s = $("run-summary");
  s.classList.remove("hidden");
  s.replaceChildren();
  s.append(...[
    el("div", { class: "item-head" }, el("span", { class: "item-title" }, "Run finished"), el("span", { class: "pill " + (p.ok ? "done" : "failed") }, p.ok ? "ok" : "failed")),
    el("div", { class: "est-grid" },
      el("div", {}, el("span", {}, "Estimated"), el("div", {}, usd(current.estimate.expected_usd, 4))),
      el("div", {}, el("span", {}, "Actual"), el("div", { class: "money" }, usd(p.actual_cost_usd, 4))),
      el("div", {}, el("span", {}, "Turns"), el("div", {}, p.num_turns ?? "n/a"))
    ),
    p.result_text ? el("pre", { id: "result-text" }, p.result_text) : null,
    p.error ? el("div", { class: "err" }, p.error) : null,
    p.stderr_tail && !p.ok ? el("pre", {}, p.stderr_tail) : null,
  ].filter(Boolean));
  try {
    const loads = await invoke("run_loads", { runId: p.run_id });
    s.append(el("div", { class: "meta" }, loads.length ? "Loaded: " + loads.map((l) => `${l.tool} ${l.target}`).join(" · ") : "Loaded: nothing (no file reads logged)"));
  } catch {}
  loadQueue();
});

// ---------- run history ----------
async function loadRuns() {
  const wrap = $("runs-list");
  wrap.replaceChildren();
  const rs = await invoke("runs_list").catch((e) => {
    wrap.append(el("div", { class: "err" }, String(e)));
    return [];
  });
  if (!rs.length) {
    wrap.append(el("div", { class: "empty" }, "No runs yet."));
    return;
  }
  wrap.append(
    el("table", {},
      el("tr", {}, ["Run", "Agent", "Status", "Estimate", "Cap", "Actual", "Loads"].map((h) => el("th", {}, h))),
      rs.map((r) =>
        el("tr", {},
          el("td", {}, r.run_id),
          el("td", {}, r.agent),
          el("td", {}, el("span", { class: "pill " + r.status }, r.status)),
          el("td", {}, usd(r.estimate.expected_usd, 4)),
          el("td", {}, usd(r.estimate.cap_usd)),
          el("td", {}, usd(r.actual_cost_usd, 4)),
          el("td", {}, el("a", {
            class: "artifacts",
            onclick: async () => {
              const l = await invoke("run_loads", { runId: r.run_id }).catch(() => []);
              showModal("Files loaded by " + r.run_id, l.length ? l.map((x) => `${x.ts}  ${x.tool}  ${x.target}`).join("\n") : "Nothing logged.");
            },
          }, "view"))
        )
      )
    )
  );
}
$("refresh-runs").addEventListener("click", loadRuns);

// ---------- start ----------
loadInfo();
loadQueue();
loadAgents().catch((e) => ($("agent-desc").textContent = String(e)));
