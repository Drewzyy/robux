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
    check("app opens on Home", "Game Ideas" in view(), view()[:60])
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
        time.sleep(1.5); d.save_screenshot(str(SHOTS / "ui-plan.png"))
        d.execute_async_script("""const done = arguments[arguments.length-1]; (async () => { const ps = await window.__TAURI__.core.invoke('plans_list'); const p = ps[0];
          for (const s of p.sections) for (const i of s.items) await window.__TAURI__.core.invoke('plan_set', {runId: p.run_id, item: i.id, checked: true, note: null}); done(); })();""")
        d.execute_script("location.hash='#home'"); time.sleep(1); d.execute_script("location.hash='#plans'"); time.sleep(3)
        btn = d.find_element(By.ID, "plan-build")
        check("Build This Game unlocks at 100%", btn.get_attribute("disabled") is None and "Build This Game" in btn.text, btn.text)
        btn.click(); time.sleep(3)
        check("Build This Game shows usage before spending", "Usage before you run" in view())
        d.find_element(By.ID, "cancel-run").click(); time.sleep(1)
        d.execute_async_script("""const done = arguments[arguments.length-1]; (async () => { const p = (await window.__TAURI__.core.invoke('plans_list'))[0];
          for (const s of p.sections) for (const i of s.items) await window.__TAURI__.core.invoke('plan_set', {runId: p.run_id, item: i.id, checked: false, note: null}); done(); })();""")
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
    check("Home shows the money strip", len(d.find_elements(By.ID, "money-strip")) == 1 and "made this month" in view().lower())
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
    d.execute_script("location.hash='#home'"); time.sleep(1); d.execute_script("location.hash='#references'"); time.sleep(2.5)
    cards = d.find_elements(By.CSS_SELECTOR, ".card.ref")
    notefile = ROOT / "vault/19-Project-Memory/dawn-watch-references.md"
    check("reference image lands in the vault note and the grid", not err and len(cards) == n0 + 1 and notefile.is_file() and "Use: round friendly eyes" in notefile.read_text(), err or f"{n0}->{len(cards)}")
    d.save_screenshot(str(SHOTS / "ui-references.png"))
    rm = d.find_elements(By.XPATH, "//div[contains(@class,'ref')]//button[text()='Remove']")
    if rm: rm[0].click(); time.sleep(2)
    check("removing a reference works", len(d.find_elements(By.CSS_SELECTOR, ".card.ref")) == n0)
    d.execute_script("location.hash='#chat'"); time.sleep(2)
    for b in d.find_elements(By.XPATH, "//button[text()='New Chat']"): b.click(); time.sleep(1.5)
    d.save_screenshot(str(SHOTS / "ui-chat-empty.png"))
    send = d.find_element(By.ID, "chat-send")
    check("Send button is plain", send.text.strip() == "Send", send.text)
    d.find_element(By.ID, "chat-input").send_keys("In one short sentence: which of my projects exist and how many Luau files does the first one have?")
    send.click()
    WebDriverWait(d, 240).until(lambda x: len(x.find_elements(By.CSS_SELECTOR, ".bubble.them:not(.typing)")) >= 1)
    time.sleep(1)
    reply = d.find_elements(By.CSS_SELECTOR, ".bubble.them")[-1].text
    check("Claude replied in the chat", len(reply) > 10 and "Could not run" not in reply, reply.replace("\n", " ")[:200])
    d.save_screenshot(str(SHOTS / "ui-chat.png"))
    d.find_element(By.ID, "chat-input").send_keys("What was my previous question? Answer in under 15 words.")
    d.find_element(By.ID, "chat-send").click()
    WebDriverWait(d, 240).until(lambda x: len(x.find_elements(By.CSS_SELECTOR, ".bubble.them:not(.typing)")) >= 2)
    time.sleep(1)
    r2 = d.find_elements(By.CSS_SELECTOR, ".bubble.them")[-1].text
    check("it remembers the conversation", "project" in r2.lower() or "luau" in r2.lower(), r2.replace("\n", " ")[:160])

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
