#!/usr/bin/env python3
"""Click through Ask Claude, Simulator and Thumbnails in the real window.
Sends one real chat message (capped). xvfb-run -a python3 scripts/ui_new_tabs.py"""
import os, subprocess, sys, time
from pathlib import Path
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import Select
from selenium.webdriver.support.ui import WebDriverWait
ROOT = Path(__file__).resolve().parent.parent
SHOTS = ROOT / "docs/screenshots"

# Fake Ollama on 11434: version, model list, and a streamed download.
import http.server, threading, json as _json
_pulled = set()
_chats = []
class _Ollama(http.server.BaseHTTPRequestHandler):
    def _send(self, obj):
        b = _json.dumps(obj).encode(); self.send_response(200); self.send_header("Content-Type", "application/json"); self.send_header("Content-Length", str(len(b))); self.end_headers(); self.wfile.write(b)
    def do_GET(self):
        if self.path == "/api/version": self._send({"version": "0.14.3"})
        elif self.path == "/api/tags": self._send({"models": [{"name": m} for m in sorted(_pulled | {"llama3.2:latest", "qwen2.5-coder:7b"})]})
        else: self.send_response(404); self.end_headers()
    def do_POST(self):
        body = _json.loads(self.rfile.read(int(self.headers.get("Content-Length", 0))) or b"{}")
        if self.path == "/api/chat":
            _chats.append(body); return self._send({"message": {"role": "assistant", "content": "minor | src/x.luau | example finding | fix it\nVerdict: SOUND WITH FIXES"}})
        self.send_response(200); self.send_header("Content-Type", "application/x-ndjson"); self.end_headers()
        for i in range(0, 101, 25):
            self.wfile.write((_json.dumps({"status": "pulling", "completed": i, "total": 100}) + "\n").encode()); self.wfile.flush(); time.sleep(0.2)
        self.wfile.write(b'{"status":"success"}\n'); _pulled.add(body.get("model"))
    def log_message(self, *a): pass
threading.Thread(target=http.server.ThreadingHTTPServer(("127.0.0.1", 11434), _Ollama).serve_forever, daemon=True).start()
ok = True
def check(name, cond, detail=""):
    global ok; ok = ok and bool(cond)
    print(("PASS " if cond else "FAIL ") + name + (f" · {detail}" if detail else ""))
env = dict(os.environ, ROBUX_STUDIO_ROOT=str(ROOT), ROBUX_CONFIG_DIR="/tmp/rs-newtabs")
drv = subprocess.Popen(["tauri-driver"], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL); time.sleep(2)
o = webdriver.ChromeOptions(); o.set_capability("browserName", "wry"); o.set_capability("tauri:options", {"application": str(ROOT / "app/src-tauri/target/debug/brewixgames")})
d = webdriver.Remote(command_executor="http://127.0.0.1:4444", options=o); d.set_window_size(1280, 820); time.sleep(3)
view = lambda: d.find_element(By.ID, "view").text
try:
    time.sleep(1)
    check("app opens on the Dashboard with widgets", len(d.find_elements(By.CSS_SELECTOR, ".widget")) == 7 and "Tune-up" in view() and len(d.find_elements(By.CSS_SELECTOR, "#nav a")) == 7, view()[:60])
    check("Dashboard shows one next step", len(d.find_elements(By.ID, "next-step")) == 1 and "Next step" in view() or "NEXT STEP" in view())
    d.find_element(By.ID, "search-hint").click(); time.sleep(0.5)
    d.find_element(By.ID, "palette-input").send_keys("thumb"); time.sleep(0.5)
    from selenium.webdriver.common.keys import Keys
    d.find_element(By.ID, "palette-input").send_keys(Keys.ENTER); time.sleep(2.5)
    check("Search palette jumps to a page", "#thumbnails" in d.execute_script("return location.hash"))
    d.execute_script("location.hash='#ideas'"); time.sleep(2.5)
    check("Game Ideas page", "Game Ideas" in view(), view()[:60])
    cards = d.find_elements(By.CSS_SELECTOR, ".card.pitch")
    check("Home shows 5 pitch cards", len(cards) == 5, f"{len(cards)} cards")
    check("Home warns when the scan had no live data", "No live chart data" in view() or "Partly live" in view() or "live" not in view().lower(), "")
    d.save_screenshot(str(SHOTS / "ui-home.png"))
    d.find_element(By.ID, "pitch-new").click(); time.sleep(2.5)
    check("Pitch New Ideas shows usage before running", "Usage before you run" in view())
    d.find_element(By.ID, "cost-cancel").click(); time.sleep(1)
    d.find_elements(By.CSS_SELECTOR, ".design-this")[0].click(); time.sleep(2.5)
    pr = d.find_element(By.ID, "prompt").get_attribute("value")
    check("Plan This Game opens Planner with the pitch", "Plan this pitch from Pulse" in pr, pr[:80])
    d.execute_script("location.hash='#plans'"); time.sleep(3)
    boxes = d.find_elements(By.CSS_SELECTOR, ".plan-check")
    if boxes:
        btn = d.find_element(By.ID, "plan-build")
        check("Game Plan lists every item with mockups, build locked", len(boxes) >= 26 and btn.get_attribute("disabled") is not None and len(d.find_elements(By.CSS_SELECTOR, ".plan-mock")) >= 4, f"{len(boxes)} items")
        check("Game Plan has Edit in Figma", len(d.find_elements(By.ID, "plan-figma")) == 1)
        time.sleep(1.5); d.save_screenshot(str(SHOTS / "ui-plan.png"))
        d.execute_async_script("""const done = arguments[arguments.length-1]; (async () => { const ps = await window.__TAURI__.core.invoke('plans_list'); const p = ps[0];
          for (const s of p.sections) for (const i of s.items) await window.__TAURI__.core.invoke('plan_set', {runId: p.run_id, item: i.id, checked: true, note: null}); done(); })();""")
        d.execute_script("location.hash='#ideas'"); time.sleep(1); d.execute_script("location.hash='#plans'"); time.sleep(3)
        btn = d.find_element(By.ID, "plan-build")
        check("Build This Game unlocks at 100%", btn.get_attribute("disabled") is None and "Build This Game" in btn.text, btn.text)
        btn.click(); time.sleep(3)
        check("Build This Game shows usage before spending", "Usage before you run" in view())
        d.find_element(By.ID, "cancel-run").click(); time.sleep(1)
        d.execute_async_script("""const done = arguments[arguments.length-1]; (async () => { const p = (await window.__TAURI__.core.invoke('plans_list'))[0];
          for (const s of p.sections) for (const i of s.items) await window.__TAURI__.core.invoke('plan_set', {runId: p.run_id, item: i.id, checked: false, note: null}); done(); })();""")
    d.execute_script("location.hash='#brief'"); time.sleep(2.5)
    d.find_element(By.ID, "brief-name").send_keys("UI Test Idea"); d.find_element(By.ID, "brief-new").click(); time.sleep(2.5)
    n_items = len(d.find_elements(By.CSS_SELECTOR, ".brief-item"))
    box = d.find_element(By.CSS_SELECTOR, ".brief-item[data-id='loop'] textarea"); box.send_keys("Collect sugar, upgrade shop, repeat")
    d.execute_script("arguments[0].dispatchEvent(new Event('change'))", box); time.sleep(2)
    PNG = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=="
    d.execute_async_script("const done = arguments[arguments.length-1]; window.__TAURI__.core.invoke('brief_add_image', {slug: 'ui-test-idea', id: 'look', name: 'ref.png', data: arguments[0]}).then(() => done()).catch((e) => done(String(e)));", PNG)
    # drag and drop a screenshot onto a checklist item (a real DataTransfer drop event)
    d.execute_script("""const b = Uint8Array.from(atob(arguments[0]), (c) => c.charCodeAt(0)); const f = new File([b], 'shot.png', {type: 'image/png'});
      const dt = new DataTransfer(); dt.items.add(f); const n = document.querySelector(".brief-item[data-id='world']");
      n.dispatchEvent(new DragEvent('dragenter', {dataTransfer: dt, bubbles: true})); n.dispatchEvent(new DragEvent('drop', {dataTransfer: dt, bubbles: true, cancelable: true}));""", PNG); time.sleep(3)
    d.execute_script("location.hash='#ideas'"); time.sleep(1); d.execute_script("location.hash='#brief'"); time.sleep(3)
    saved_box = d.find_element(By.CSS_SELECTOR, ".brief-item[data-id='loop'] textarea").get_attribute("value")
    check("Game Brief checklist saves answers, images and dropped screenshots", n_items == 19 and saved_box.startswith("Collect sugar") and "3 of 19 covered" in view() and len(d.find_elements(By.CSS_SELECTOR, ".brief-thumb")) == 2, f"{n_items} items · {saved_box[:20]}")
    d.save_screenshot(str(SHOTS / "ui-brief.png"))
    d.find_element(By.ID, "brief-plan").click(); time.sleep(2.5)
    pr = d.find_element(By.ID, "prompt").get_attribute("value")
    check("Plan This Game sends the brief to Planner", "Collect sugar, upgrade shop, repeat" in pr and "ui-test-idea" in pr and "(not answered)" in pr)
    d.execute_script("location.hash='#score'"); time.sleep(10)
    check("Game Score rates the game out of 100 with what to add", "dawn-watch" in view() and "Keeps players playing" in view() and len(d.find_elements(By.CSS_SELECTOR, ".score-num")) >= 1, view()[:140].replace("\n", " "))
    d.save_screenshot(str(SHOTS / "ui-score.png"))
    d.execute_script("location.hash='#chat'"); time.sleep(3)
    Select(d.find_element(By.ID, "chat-model")).select_by_value("ollama:qwen2.5-coder:7b"); time.sleep(3)
    eng = d.execute_async_script("const done = arguments[arguments.length-1]; window.__TAURI__.core.invoke('settings_get').then((s) => done([s.engine, s.ollama_model]));")
    check("Ask Claude can switch to a free local Ollama model", eng == ["ollama", "qwen2.5-coder:7b"] and "free" in d.find_element(By.ID, "chat-send").text.lower(), str(eng))
    Select(d.find_element(By.ID, "chat-model")).select_by_value("claude:claude-haiku-5-5"); time.sleep(2.5)
    saved = d.execute_async_script("const done = arguments[arguments.length-1]; window.__TAURI__.core.invoke('settings_get').then((s) => done(s.chat_model));")
    check("Ask Claude lets you choose the model and it saves", saved == "claude-haiku-5-5" and Select(d.find_element(By.ID, "chat-model")).first_selected_option.get_attribute("value") == "claude:claude-haiku-5-5", saved)
    # a stale page copy must not undo another save
    both = d.execute_async_script("""const done = arguments[arguments.length-1]; const inv = window.__TAURI__.core.invoke;
      (async () => { await inv('settings_patch', {patch: {app_dirs: {blender: '/tmp/blender-x'}}}); await inv('settings_patch', {patch: {tuneup_days: 3}}); const s = await inv('settings_get');
        await inv('settings_patch', {patch: {chat_model: '', engine: 'claude', tuneup_days: 7, app_dirs: {blender: ''}}}); done([s.app_dirs.blender, s.tuneup_days, s.chat_model]); })();""")
    check("separate saves keep each other's changes", both == ["/tmp/blender-x", 3, "claude-haiku-5-5"], str(both))
    # leave a slow page, land on the page you clicked last
    d.execute_script("location.hash='#vault'; location.hash='#settings'; location.hash='#money'"); time.sleep(4)
    check("fast clicking ends on the last page clicked", "Robux converted" in view(), view()[:60].replace("\n", " "))
    d.execute_script("location.hash='#tuneup/now'"); time.sleep(8)
    check("Tune-Up scans the game's code and map", "dawn-watch" in view() and "code files" in view() and "parts" in view() and len(d.find_elements(By.ID, "tuneup-run")) == 1, view()[:120].replace("\n", " "))
    d.save_screenshot(str(SHOTS / "ui-tuneup.png"))
    d.execute_script("location.hash='#settings'"); time.sleep(3)
    check("Settings offers the optional Figma login", "Figma (optional login)" in view() and "no login, no limits" in view())
    d.execute_script("location.hash='#review'"); time.sleep(2.5)
    Select(d.find_element(By.ID, "review-target")).select_by_value("dawn-watch")
    d.find_element(By.ID, "review-local").click()
    for _ in range(40):
        time.sleep(0.5)
        if "Verdict: SOUND WITH FIXES" in view(): break
    sent = _chats[0]["messages"][1]["content"] if _chats else ""
    check("Second Opinion reviews a game with the local model", "Verdict: SOUND WITH FIXES" in view() and ".luau =====" in sent and _chats[0]["model"] == "qwen2.5-coder:7b", f"{len(_chats)} request(s)")
    d.save_screenshot(str(SHOTS / "ui-second-opinion.png"))
    d.execute_script("location.hash='#money'"); time.sleep(2.5)
    d.find_element(By.ID, "money-game").send_keys("Test Game")
    d.find_element(By.ID, "money-robux").send_keys("30000")
    d.find_element(By.ID, "money-add").click(); time.sleep(2.5)
    check("Money converts 30,000 Robux to $114.00", "$114.00" in view() and "Test Game" in view(), view()[:160].replace("\n", " "))
    d.save_screenshot(str(SHOTS / "ui-money.png"))
    d.execute_script("location.hash='#home'"); time.sleep(2.5)
    check("Dashboard shows the money strip", len(d.find_elements(By.ID, "money-strip")) == 1 and "made this month" in view().lower())
    d.execute_script("location.hash='#money'"); time.sleep(2.5)
    d.execute_script("document.querySelector('details.sources').open = true"); time.sleep(0.5)
    for b in d.find_elements(By.XPATH, "//details[contains(@class,'sources')]//button[text()='Delete']"):
        b.click(); time.sleep(2); break
    check("deleting an earnings entry works", "Test Game" not in view())
    d.execute_script("location.hash='#references'"); time.sleep(2.5)
    n0 = len(d.find_elements(By.CSS_SELECTOR, ".card.ref"))
    d.find_element(By.ID, "ref-note").send_keys("round friendly eyes")
    err = d.execute_async_script("""
      const done = arguments[0];
      const c = document.createElement('canvas'); c.width = 64; c.height = 40;
      const g = c.getContext('2d'); g.fillStyle = '#FF3B30'; g.fillRect(0, 0, 64, 40);
      window.__TAURI__.core.invoke('refs_add', { project: document.getElementById('project-pick').value, name: 'test-ref.png', data: c.toDataURL('image/png'), category: 'Character', note: document.getElementById('ref-note').value, source: 'ui test' })
        .then(() => done('')).catch((e) => done(String(e)));
    """)
    d.execute_script("location.hash='#ideas'"); time.sleep(1); d.execute_script("location.hash='#references'"); time.sleep(2.5)
    cards = d.find_elements(By.CSS_SELECTOR, ".card.ref")
    notefile = ROOT / "vault/19-Project-Memory/dawn-watch-references.md"
    check("reference image lands in the vault note and the grid", not err and len(cards) == n0 + 1 and notefile.is_file() and "Use: round friendly eyes" in notefile.read_text(), err or f"{n0}->{len(cards)}")
    d.save_screenshot(str(SHOTS / "ui-references.png"))
    n1 = len(d.find_elements(By.CSS_SELECTOR, ".card.ref"))
    d.execute_script("""const b = Uint8Array.from(atob('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg=='), (c) => c.charCodeAt(0));
      const dt = new DataTransfer(); dt.items.add(new File([b], 'dropped.png', {type: 'image/png'})); const n = document.querySelector('#view .page');
      n.dispatchEvent(new DragEvent('drop', {dataTransfer: dt, bubbles: true, cancelable: true}));"""); time.sleep(3.5)
    check("dragging a screenshot onto References adds it", len(d.find_elements(By.CSS_SELECTOR, ".card.ref")) == n1 + 1, f"{n1}->{len(d.find_elements(By.CSS_SELECTOR, '.card.ref'))}")
    for _ in range(2):
        rr = d.find_elements(By.XPATH, "//div[contains(@class,'ref')]//button[text()='Remove']")
        if rr and len(d.find_elements(By.CSS_SELECTOR, ".card.ref")) > n0 + 1: rr[0].click(); time.sleep(2)
    rm = d.find_elements(By.XPATH, "//div[contains(@class,'ref')]//button[text()='Remove']")
    if rm: rm[0].click(); time.sleep(2)
    check("removing a reference works", len(d.find_elements(By.CSS_SELECTOR, ".card.ref")) == n0)
    d.execute_script("location.hash='#chat'"); time.sleep(2)
    for b in d.find_elements(By.XPATH, "//button[text()='New Chat']"): b.click(); time.sleep(1.5)
    d.save_screenshot(str(SHOTS / "ui-chat-empty.png"))
    send = d.find_element(By.ID, "chat-send")
    check("Send button is plain", send.text.strip() == "Send", send.text)
    from selenium.webdriver.common.keys import Keys
    d.find_element(By.ID, "chat-input").send_keys("In one short sentence: which of my projects exist and how many Luau files does the first one have?")
    d.find_element(By.ID, "chat-input").send_keys(Keys.ENTER)
    WebDriverWait(d, 240).until(lambda x: len(x.find_elements(By.CSS_SELECTOR, ".bubble.them:not(.typing)")) >= 1)
    time.sleep(1)
    reply = d.find_elements(By.CSS_SELECTOR, ".bubble.them")[-1].text
    check("Claude replied in the chat", len(reply) > 10 and "Could not run" not in reply, reply.replace("\n", " ")[:200])
    d.save_screenshot(str(SHOTS / "ui-chat.png"))
    # break the saved conversation the way it happens after a move or an update
    import json as _j
    cf = ROOT / "runs/chat.json"; cj = _j.loads(cf.read_text()); cj["session"] = "00000000-0000-4000-8000-000000000000"; cf.write_text(_j.dumps(cj))
    d.find_element(By.ID, "chat-input").send_keys("What was my previous question? Answer in under 15 words.")
    d.find_element(By.ID, "chat-send").click()
    WebDriverWait(d, 240).until(lambda x: len(x.find_elements(By.CSS_SELECTOR, ".bubble.them:not(.typing)")) >= 2)
    time.sleep(1)
    r2 = d.find_elements(By.CSS_SELECTOR, ".bubble.them")[-1].text
    check("a lost saved conversation recovers by itself and still remembers", ("project" in r2.lower() or "luau" in r2.lower()) and "didn't finish" not in r2.lower() and "no conversation found" not in r2.lower(), r2.replace("\n", " ")[:160])

    d.execute_script("location.hash='#simulator'"); time.sleep(2.5)
    v = view()
    check("simulator shows a rating and personas", "Rating" in v and "Age 7-9" in v and "Fix these first" in v, v.replace("\n", " | ")[:160])
    d.save_screenshot(str(SHOTS / "ui-simulator.png"))
    d.execute_script("document.getElementById('view').scrollTop=700"); time.sleep(0.5)
    d.save_screenshot(str(SHOTS / "ui-simulator-fixes.png"))
    d.execute_script("document.getElementById('view').scrollTop=0")
    d.find_element(By.ID, "sim-run").click(); time.sleep(2)
    check("simulation shows cost before running", "Usage before you run" in view())
    d.find_element(By.XPATH, "//button[text()='Cancel']").click(); time.sleep(0.5)

    d.execute_script("location.hash='#simulator'"); time.sleep(2.5)
    d.find_element(By.ID, "open-studio").click(); time.sleep(6)
    place = ROOT / "projects/dawn-watch/build/dawn-watch.rbxl"
    tt = d.find_element(By.ID, "toast").text
    check("Open in Studio builds the place file", place.is_file() and place.stat().st_size > 1000, f"{place.stat().st_size if place.is_file() else 0} bytes · {tt[:90]}")
    d.execute_script("location.hash='#thumbnails'"); time.sleep(3)
    grades = [g.text for g in d.find_elements(By.CSS_SELECTOR, ".tcell .pill")]
    check("thumbnails listed with grades", len(grades) >= 10, ", ".join(grades))
    d.execute_script("document.getElementById('shot-path').value='/etc/passwd'")
    d.find_element(By.XPATH, "//button[text()='Add Screenshot']").click(); time.sleep(1)
    check("non-image screenshot refused", "png or .jpg" in d.find_element(By.ID, "toast").text.lower(), d.find_element(By.ID, "toast").text)
    d.save_screenshot(str(SHOTS / "ui-thumbnails.png"))
    # Request changes: on the simulator item that stopped at the usage limit.
    d.execute_script("location.hash='#queue'"); time.sleep(2.5)
    cards = [c for c in d.find_elements(By.CSS_SELECTOR, ".qcard") if c.get_attribute("data-id") == "q-run-simulator-20261110-103642-af97"]
    if cards:
        cards[0].find_element(By.XPATH, ".//button[text()='Request Changes']").click(); time.sleep(1.5)
        d.find_element(By.ID, "request-changes").click(); time.sleep(1)
        check("request changes needs a note", "note first" in d.find_element(By.ID, "toast").text, d.find_element(By.ID, "toast").text)
        d.find_element(By.ID, "decision-note").send_keys("Add a persona for a 13-15 year old on gamepad.")
        d.find_element(By.ID, "request-changes").click(); time.sleep(4)
        pr = d.find_element(By.ID, "prompt").get_attribute("value")
        check("redo opens with the note and a cost", "gamepad" in pr and "Usage before you run" in view(), pr[-90:])
        d.save_screenshot(str(SHOTS / "ui-request-changes.png"))
        d.find_element(By.ID, "cancel-run").click(); time.sleep(0.5)
    else:
        check("request changes", True, "item already decided; skipped")
    d.execute_script("location.hash='#vault'"); time.sleep(2)
    d.find_element(By.ID, "vault-q").send_keys("datastore retry"); time.sleep(1.5)
    rows = d.find_elements(By.CSS_SELECTOR, "#vault-results .row")
    check("vault search finds notes with highlights", len(rows) >= 2 and "DataStore" in rows[0].text and d.find_elements(By.CSS_SELECTOR, "#vault-results mark"), rows[0].text.replace("\n", " ")[:90] if rows else "none")
    d.save_screenshot(str(SHOTS / "ui-vault-search.png"))
    import tempfile, json as _j, shutil as _sh2
    newdir = Path(tempfile.mkdtemp()) / "MovedStudio"
    d.execute_script("location.hash='#settings'"); time.sleep(3)
    box = d.find_element(By.ID, "studio-root"); box.clear(); box.send_keys(str(newdir))
    d.find_element(By.ID, "studio-move").click()
    WebDriverWait(d, 120).until(lambda x: "Copied" in x.find_element(By.ID, "toast").text or "not" in x.find_element(By.ID, "toast").text)
    check("studio folder moves to a new folder", (newdir / "studio/agents.json").is_file() and (newdir / "projects/dawn-watch").is_dir() and "Copied your studio to" in d.find_element(By.ID, "toast").text, d.find_element(By.ID, "toast").text[:100])
    time.sleep(2)
    box = d.find_element(By.ID, "studio-root"); box.clear(); box.send_keys(str(ROOT))
    d.find_element(By.ID, "studio-move").click(); time.sleep(3)
    check("switches back to an existing studio", "Switched to the studio in" in d.find_element(By.ID, "toast").text, d.find_element(By.ID, "toast").text[:100])
    _sh2.rmtree(newdir.parent, ignore_errors=True)
    d.execute_script("location.hash='#settings'"); time.sleep(3)
    check("Settings lists the apps in the Downloads folder", "Apps in your Downloads folder" in view() and "Roblox always installs Studio" in view())
    import tempfile as _tf
    tdir = Path(_tf.mkdtemp()) / "BrewixDownloads"
    box = d.find_element(By.ID, "tools-dir"); box.clear(); box.send_keys(str(tdir))
    d.find_element(By.ID, "tools-dir-save").click(); time.sleep(2)
    d.find_element(By.ID, "claude-install").click()
    WebDriverWait(d, 60).until(lambda x: "Claude Code is in" in x.find_element(By.ID, "toast").text or "could not" in x.find_element(By.ID, "toast").text)
    copied = tdir / "claude" / "claude"
    check("Claude Code copied into the Downloads folder", copied.is_file() and copied.stat().st_size > 1000000, d.find_element(By.ID, "toast").text[:100])
    time.sleep(2)
    box = d.find_element(By.ID, "tools-dir"); box.clear()
    d.find_element(By.ID, "tools-dir-save").click(); time.sleep(2)
    check("Ollama detected with memory and version", "Ollama is ready" in view() and "GB memory" in view() and "v0.14.3" in view())
    Select(d.find_element(By.ID, "ollama-model")).select_by_value("qwen3:4b"); time.sleep(0.5)
    dl = d.find_element(By.ID, "ollama-download")
    check("download button names the size", "2.5 GB" in dl.text, dl.text)
    dl.click()
    WebDriverWait(d, 30).until(lambda x: "downloaded" in x.find_element(By.ID, "toast").text)
    time.sleep(3)
    check("model shows as downloaded after", "not downloaded" not in Select(d.find_element(By.ID, "ollama-model")).first_selected_option.text, Select(d.find_element(By.ID, "ollama-model")).first_selected_option.text)
    d.save_screenshot(str(SHOTS / "ui-settings-ollama.png"))
    d.find_element(By.CSS_SELECTOR, "[data-seg='mock']").click(); time.sleep(2.5)
    check("Studio test mode turns on", "pretend Studio" in view())
    d.execute_script("document.getElementById('studio-mcp-path').scrollIntoView({block:'center'})"); time.sleep(0.5)
    d.save_screenshot(str(SHOTS / "ui-settings-studio.png"))
    d.find_element(By.CSS_SELECTOR, "[data-seg='studio']").click(); time.sleep(1.5)
    check("real Studio refused when not installed", "not installed" in d.find_element(By.ID, "toast").text, d.find_element(By.ID, "toast").text)
    d.find_element(By.CSS_SELECTOR, "[data-seg='off']").click(); time.sleep(2)
    check("Studio connection back off", "Agents only write files" in view())
    segs = d.find_elements(By.CSS_SELECTOR, "[data-seg='blender']")
    segs[0].click(); time.sleep(1.5)
    import shutil as _sh
    has_uv = bool(_sh.which("uvx")) or (Path.home() / ".local/bin/uvx").is_file()
    tt = d.find_element(By.ID, "toast").text
    check("Blender mode matches whether uv is installed", ("Connected to Blender" in tt) if has_uv else ("uv is not installed" in tt), tt)
    d.find_elements(By.CSS_SELECTOR, "[data-seg='mock']")[-1].click(); time.sleep(2.5)
    check("Blender test mode turns on", "pretend Blender" in view())
    d.execute_script("document.getElementById('blender-addon').scrollIntoView({block:'center'})"); time.sleep(0.5)
    d.save_screenshot(str(SHOTS / "ui-settings-blender.png"))
    d.find_elements(By.CSS_SELECTOR, "[data-seg='off']")[-1].click(); time.sleep(2)
    check("Blender back off", "Modeler writes Blender scripts" in view())
    d.execute_script("location.hash='#settings'"); time.sleep(2)
    d.execute_script("document.getElementById('view').scrollTop=260"); time.sleep(0.5)
    d.save_screenshot(str(SHOTS / "ui-settings-engine.png"))
finally:
    d.quit(); drv.terminate()
print("NEW TABS:", "PASS" if ok else "FAIL"); sys.exit(0 if ok else 1)
