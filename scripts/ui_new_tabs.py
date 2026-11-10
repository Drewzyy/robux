#!/usr/bin/env python3
"""Click through Ask Claude, Simulator and Thumbnails in the real window.
Sends one real chat message (capped). xvfb-run -a python3 scripts/ui_new_tabs.py"""
import os, subprocess, sys, time
from pathlib import Path
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
ROOT = Path(__file__).resolve().parent.parent
SHOTS = ROOT / "docs/screenshots"
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
    d.execute_script("location.hash='#chat'"); time.sleep(2)
    for b in d.find_elements(By.XPATH, "//button[text()='New Chat']"): b.click(); time.sleep(1.5)
    d.save_screenshot(str(SHOTS / "ui-chat-empty.png"))
    send = d.find_element(By.ID, "chat-send")
    check("Send button shows the cap", "up to $" in send.text, send.text)
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
    check("simulation shows cost before running", "Cost before you run" in view())
    d.find_element(By.XPATH, "//button[text()='Cancel']").click(); time.sleep(0.5)

    d.execute_script("location.hash='#thumbnails'"); time.sleep(3)
    grades = [g.text for g in d.find_elements(By.CSS_SELECTOR, ".tcell .pill")]
    check("thumbnails listed with grades", len(grades) >= 10, ", ".join(grades))
    d.execute_script("document.getElementById('shot-path').value='/etc/passwd'")
    d.find_element(By.XPATH, "//button[text()='Add Screenshot']").click(); time.sleep(1)
    check("non-image screenshot refused", "png or .jpg" in d.find_element(By.ID, "toast").text.lower(), d.find_element(By.ID, "toast").text)
    d.save_screenshot(str(SHOTS / "ui-thumbnails.png"))
    d.execute_script("location.hash='#settings'"); time.sleep(2)
    d.execute_script("document.getElementById('view').scrollTop=260"); time.sleep(0.5)
    d.save_screenshot(str(SHOTS / "ui-settings-engine.png"))
finally:
    d.quit(); drv.terminate()
print("NEW TABS:", "PASS" if ok else "FAIL"); sys.exit(0 if ok else 1)
