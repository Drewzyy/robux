#!/usr/bin/env python3
"""Drive the real Robux Studio window through WebDriver and check it works.

Linux only (uses tauri-driver + WebKitWebDriver under a virtual display).
On macOS, run the app by hand; Tauri has no WebDriver support there.

  xvfb-run -a python3 scripts/gui_smoke.py --phase0
  xvfb-run -a python3 scripts/gui_smoke.py --queue-only

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


if __name__ == "__main__":
    sys.exit(main())
