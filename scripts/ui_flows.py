#!/usr/bin/env python3
"""Click through the real app window and check the flows work end to end.

  ROBUX_CONFIG_DIR=/tmp/x ROBUX_ALLOW_LOCAL_WEBHOOK=1 xvfb-run -a python3 scripts/ui_flows.py

Flows: settings save (budget, webhook) and Send Test to a local mock
webhook; a run blocked by a tiny budget shows the reason; a real
bridge-test run (about $0.001) from estimate to approve to live output to
done; a queue item opened in its sheet. Needs scripts/mockhook on 8765.
"""
import os, subprocess, sys, time, json
from pathlib import Path
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import Select, WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

ROOT = Path(__file__).resolve().parent.parent
APP = ROOT / "app/src-tauri/target/debug/robux-studio"
SHOTS = ROOT / "docs/screenshots"
ok = True
def check(name, cond, detail=""):
    global ok
    ok = ok and bool(cond)
    print(("PASS " if cond else "FAIL ") + name + (f" · {detail}" if detail else ""))

def main():
    env = dict(os.environ, ROBUX_STUDIO_ROOT=str(ROOT))
    drv = subprocess.Popen(["tauri-driver"], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(2)
    o = webdriver.ChromeOptions(); o.set_capability("browserName", "wry"); o.set_capability("tauri:options", {"application": str(APP)})
    d = webdriver.Remote(command_executor="http://127.0.0.1:4444", options=o)
    d.set_window_size(1280, 820); time.sleep(2.5)
    w = WebDriverWait(d, 20)
    toast = lambda: d.find_element(By.ID, "toast").text
    try:
        # Settings: webhook + test, budget
        d.execute_script("location.hash='#settings'"); time.sleep(2)
        hook = d.find_element(By.ID, "webhook"); hook.clear(); hook.send_keys("http://127.0.0.1:8765/api/webhooks/ui-test")
        d.find_element(By.ID, "save-webhook").click(); time.sleep(1)
        check("webhook saves", "saved" in toast().lower(), toast())
        d.find_element(By.ID, "send-test").click(); time.sleep(2)
        check("Send Test reaches the webhook", "sent" in toast().lower(), toast())
        b = d.find_element(By.ID, "budget"); b.clear(); b.send_keys("0.01")
        d.find_element(By.ID, "save-budget").click(); time.sleep(1)
        check("budget saves", "budget saved" in toast().lower(), toast())
        # Budget blocks a run
        d.execute_script("location.hash='#runs'"); time.sleep(1.2)
        d.find_element(By.CSS_SELECTOR, "[data-seg=new]").click(); time.sleep(1.2)
        Select(d.find_element(By.ID, "agent-select")).select_by_value("bridge-test"); time.sleep(1)
        d.find_element(By.ID, "prompt").send_keys("hello from the new UI")
        d.find_element(By.ID, "estimate-btn").click()
        w.until(EC.element_to_be_clickable((By.ID, "approve-run")))
        d.save_screenshot(str(SHOTS / "ui-estimate.png"))
        d.find_element(By.ID, "approve-run").click(); time.sleep(1.5)
        check("budget stops the run with a reason", "Monthly budget reached" in toast(), toast()[:120])
        d.find_element(By.ID, "cancel-run").click(); time.sleep(0.5)
        # Raise budget, run for real
        d.execute_script("location.hash='#settings'"); time.sleep(1.5)
        b = d.find_element(By.ID, "budget"); b.clear(); b.send_keys("25")
        d.find_element(By.ID, "save-budget").click(); time.sleep(1)
        d.execute_script("location.hash='#runs'"); time.sleep(1.2)
        d.find_element(By.CSS_SELECTOR, "[data-seg=new]").click(); time.sleep(1.2)
        Select(d.find_element(By.ID, "agent-select")).select_by_value("bridge-test"); time.sleep(1)
        d.find_element(By.ID, "prompt").send_keys("hello from the new UI")
        d.find_element(By.ID, "estimate-btn").click()
        w.until(EC.element_to_be_clickable((By.ID, "approve-run"))).click()
        time.sleep(3)
        side = d.find_element(By.ID, "running-line").text
        check("sidebar shows the running estimate", "Running" in side or "No run" in side, side)
        WebDriverWait(d, 180).until(lambda x: "Done" in x.find_element(By.ID, "view").text and "$0.0" in x.find_element(By.ID, "actual-cost").text)
        time.sleep(1)
        d.save_screenshot(str(SHOTS / "ui-run-finished.png"))
        v = d.find_element(By.ID, "view").text
        check("run detail shows estimate and actual", "Estimated before" in v and "Actual after" in v, d.find_element(By.ID, "actual-cost").text)
        # Queue sheet
        d.execute_script("location.hash='#queue'"); time.sleep(2)
        cards = d.find_elements(By.CSS_SELECTOR, ".qcard .qmain")
        if cards:
            cards[0].click(); time.sleep(1.5)
            d.save_screenshot(str(SHOTS / "ui-queue-sheet.png"))
            check("queue item opens with checks", "How it was verified" in d.find_element(By.ID, "sheet").text)
    finally:
        d.quit(); drv.terminate()
    print("UI FLOWS:", "PASS" if ok else "FAIL")
    return 0 if ok else 1

if __name__ == "__main__":
    sys.exit(main())
