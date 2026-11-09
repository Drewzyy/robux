#!/usr/bin/env python3
"""Drive the real Robux Studio window through WebDriver and check it works.

Linux only (uses tauri-driver + WebKitWebDriver under a virtual display).
On macOS, run the app by hand; Tauri has no WebDriver support there.

  xvfb-run -a python3 scripts/gui_smoke.py --phase0
  xvfb-run -a python3 scripts/gui_smoke.py --phase2

--phase2 adds two fixture items to the queue (one passed gate, one failed
gate), checks every queue card renders with its gate checks, approves the
passing fixture in the window, checks the failed one's Approve button is
disabled, opens the pipeline estimate screen and cancels it (no run), then
removes the fixtures. Real items are not touched.

--phase0 does one real bridge-test run (about $0.001 on Haiku), approved in
the UI at the cap the UI shows. Screenshots go to docs/screenshots/.
"""
import os
import subprocess
import sys
import time
from pathlib import Path

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.ui import Select, WebDriverWait

ROOT = Path(__file__).resolve().parent.parent
APP = ROOT / "app/src-tauri/target/debug/robux-studio"
SHOTS = ROOT / "docs/screenshots"


def main() -> int:
    phase0 = "--phase0" in sys.argv
    SHOTS.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, ROBUX_STUDIO_ROOT=str(ROOT))
    driver_proc = subprocess.Popen(
        ["tauri-driver"], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL
    )
    time.sleep(2)
    caps = {"browserName": "wry", "tauri:options": {"application": str(APP)}}
    opts = webdriver.ChromeOptions()  # only used as a capability carrier
    for k, v in caps.items():
        opts.set_capability(k, v)
    d = webdriver.Remote(command_executor="http://127.0.0.1:4444", options=opts)
    d.set_window_size(1180, 800)
    wait = WebDriverWait(d, 20)
    ok = True
    try:
        # 1. App launches and the review queue renders.
        wait.until(EC.presence_of_element_located((By.ID, "queue-list")))
        time.sleep(1.5)
        items = d.find_elements(By.CSS_SELECTOR, "#queue-list .card")
        empties = d.find_elements(By.ID, "queue-empty")
        print(f"queue: {len(items)} item(s), empty-state shown: {bool(empties)}")
        d.save_screenshot(str(SHOTS / "01-review-queue.png"))
        env_text = d.find_element(By.ID, "env").text
        print("env panel:", env_text.replace("\n", " | "))

        if "--phase2" in sys.argv:
            ok = phase2(d, wait) and ok

        if phase0:
            # 2. Estimate shown before the run.
            d.find_element(By.CSS_SELECTOR, "nav button[data-tab=run]").click()
            wait.until(EC.visibility_of_element_located((By.ID, "agent-select")))
            time.sleep(1)
            Select(d.find_element(By.ID, "agent-select")).select_by_value("bridge-test")
            d.find_element(By.ID, "prompt").send_keys("hello from the app window")
            d.find_element(By.ID, "estimate-btn").click()
            approve = wait.until(
                EC.element_to_be_clickable((By.CSS_SELECTOR, "#estimate button.approve"))
            )
            print("estimate card:", d.find_element(By.ID, "estimate").text.replace("\n", " | ")[:400])
            d.save_screenshot(str(SHOTS / "02-estimate-before-run.png"))

            # 3. Approve at the shown cap; output appears in the app.
            approve.click()
            res = WebDriverWait(d, 180).until(
                EC.presence_of_element_located((By.ID, "result-text"))
            )
            time.sleep(1)
            print("result in app:", res.text)
            print("summary:", d.find_element(By.ID, "run-summary").text.replace("\n", " | "))
            d.save_screenshot(str(SHOTS / "03-run-output.png"))
            ok = res.text.startswith("BRIDGE_OK")
    finally:
        d.quit()
        driver_proc.terminate()
    print("GUI SMOKE:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


def fixture(id_, passed):
    import json
    item = {
        "id": id_, "kind": "build", "title": f"GUI fixture ({'gate passed' if passed else 'gate failed'})",
        "summary": "Test fixture created by scripts/gui_smoke.py. Safe to ignore.",
        "created_at": "2026-11-09T00:00:00Z", "created_by": "gui_smoke", "run_id": None,
        "artifacts": [], "gate": {"passed": passed, "checks": [{"name": "fixture check", "passed": passed, "detail": ""}]},
        "status": "pending", "decision": None,
    }
    (ROOT / "queue/items" / f"{id_}.json").write_text(json.dumps(item))


def phase2(d, wait):
    import json
    ok = True
    fixture("q-fixture-pass", True)
    fixture("q-fixture-fail", False)
    try:
        d.find_element(By.ID, "refresh-queue").click()
        time.sleep(1.5)
        cards = d.find_elements(By.CSS_SELECTOR, "#queue-list .card")
        titles = [c.find_element(By.CSS_SELECTOR, ".item-title").text for c in cards]
        print("queue cards:", titles)
        real = [t for t in titles if "fixture" not in t]
        print("real (non-fixture) items:", real)
        d.save_screenshot(str(SHOTS / "04-queue-with-items.png"))
        for c in cards:
            t = c.find_element(By.CSS_SELECTOR, ".item-title").text
            if "fixture" in t:
                continue
            checks = c.find_elements(By.CSS_SELECTOR, ".checks li")
            arts = c.find_elements(By.CSS_SELECTOR, ".artifacts a")
            print(f"  '{t}': {len(checks)} gate checks shown, {len(arts)} artifacts")
        # failed gate: approve disabled
        for c in cards:
            if "gate failed" in c.find_element(By.CSS_SELECTOR, ".item-title").text and "fixture" in c.text:
                btn = c.find_element(By.CSS_SELECTOR, "button.approve")
                print("failed-gate fixture approve disabled:", not btn.is_enabled())
                ok = ok and not btn.is_enabled()
        # approve the passing fixture in the UI
        for c in cards:
            if "fixture (gate passed)" in c.text:
                c.find_element(By.CSS_SELECTOR, "input[type=text]").send_keys("approved by gui_smoke")
                c.find_element(By.CSS_SELECTOR, "button.approve").click()
                break
        time.sleep(1.5)
        item = json.loads((ROOT / "queue/items/q-fixture-pass.json").read_text())
        print("fixture after click:", item["status"], item["decision"])
        ok = ok and item["status"] == "approved"
        # pipeline estimate screen, then cancel (no run)
        d.find_element(By.CSS_SELECTOR, "nav button[data-tab=run]").click()
        time.sleep(0.8)
        Select(d.find_element(By.ID, "mode-select")).select_by_value("pipeline")
        Select(d.find_element(By.ID, "pipeline-select")).select_by_value("pitch-to-build")
        d.find_element(By.ID, "prompt").clear()
        d.find_element(By.ID, "prompt").send_keys("gui smoke: estimate only")
        d.find_element(By.ID, "estimate-btn").click()
        btn = wait.until(EC.element_to_be_clickable((By.CSS_SELECTOR, "#estimate button.approve")))
        print("pipeline estimate:", d.find_element(By.ID, "estimate").text.replace("\n", " | ")[:300])
        d.save_screenshot(str(SHOTS / "05-pipeline-estimate.png"))
        d.find_element(By.CSS_SELECTOR, "#estimate button.ghost").click()
        time.sleep(1)
        d.find_element(By.CSS_SELECTOR, "nav button[data-tab=queue]").click()
        time.sleep(1)
    finally:
        for f in ("q-fixture-pass", "q-fixture-fail"):
            (ROOT / "queue/items" / f"{f}.json").unlink(missing_ok=True)
        # drop the fixture decision line, keep real decisions
        log = ROOT / "queue/decisions.jsonl"
        if log.exists():
            keep = [l for l in log.read_text().splitlines() if "q-fixture-" not in l]
            log.write_text("".join(k + "\n" for k in keep))
        # remove cancelled estimate-only pipeline folders
        import shutil
        for p in (ROOT / "runs").glob("pipe-pitch-to-build-*"):
            pj = p / "pipeline.json"
            rq = p / "request.json"
            for f in (pj, rq):
                if f.exists() and '"cancelled"' in f.read_text() and "gui smoke" in f.read_text():
                    shutil.rmtree(p, ignore_errors=True)
    return ok


if __name__ == "__main__":
    sys.exit(main())
