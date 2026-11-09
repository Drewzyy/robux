#!/usr/bin/env python3
"""Screenshot every screen of the real app window (Linux, WebDriver).

  xvfb-run -a python3 scripts/ui_shots.py [route ...]

Uses a throwaway settings folder (ROBUX_CONFIG_DIR) so real settings are
untouched. Screenshots go to docs/screenshots/ui-<route>.png.
"""
import os, subprocess, sys, time, json
from pathlib import Path
from selenium import webdriver
from selenium.webdriver.common.by import By

ROOT = Path(__file__).resolve().parent.parent
APP = ROOT / "app/src-tauri/target/debug/brewixgames"
SHOTS = ROOT / "docs/screenshots"
ROUTES = sys.argv[1:] or ["queue", "agents", "vault", "runs", "runs-new", "run-detail", "settings"]

def main():
    SHOTS.mkdir(parents=True, exist_ok=True)
    cfg = Path(os.environ.get("ROBUX_CONFIG_DIR", "/tmp/rs-ui-config"))
    cfg.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, ROBUX_STUDIO_ROOT=str(ROOT), ROBUX_CONFIG_DIR=str(cfg))
    drv = subprocess.Popen(["tauri-driver"], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(2)
    opts = webdriver.ChromeOptions()
    opts.set_capability("browserName", "wry")
    opts.set_capability("tauri:options", {"application": str(APP)})
    d = webdriver.Remote(command_executor="http://127.0.0.1:4444", options=opts)
    d.set_window_size(1280, 820)
    time.sleep(2.5)
    try:
        for r in ROUTES:
            if r == "runs-new":
                d.execute_script("location.hash='#runs'"); time.sleep(1)
                d.find_element(By.CSS_SELECTOR, "[data-seg=new]").click()
            elif r == "run-detail":
                d.execute_script("location.hash='#runs'"); time.sleep(1.2)
                d.find_element(By.CSS_SELECTOR, "[data-seg=history]").click(); time.sleep(1.2)
                pipe = d.find_elements(By.CSS_SELECTOR, "#runs-list .row[data-run^=pipe]")
                if pipe: pipe[0].click(); time.sleep(2); d.save_screenshot(str(SHOTS / "ui-run-detail.png")); print("[run-detail]", d.find_element(By.ID, "view").text.replace(chr(10), " | ")[:600]); continue
                rows = d.find_elements(By.CSS_SELECTOR, "#runs-list .row")
                if rows: rows[0].click()
            else:
                d.execute_script(f"location.hash='#{r}'")
            time.sleep(2.2)
            d.save_screenshot(str(SHOTS / f"ui-{r}.png"))
            if r == "settings":
                d.execute_script("document.getElementById('view').scrollTop = 99999"); time.sleep(0.6)
                d.save_screenshot(str(SHOTS / "ui-settings-bottom.png"))
            txt = d.find_element(By.ID, "view").text.replace("\n", " | ")
            side = d.find_element(By.ID, "status").text.replace("\n", " | ")
            print(f"[{r}] {txt[:260]}\n   sidebar: {side}")
    finally:
        d.quit(); drv.terminate()

if __name__ == "__main__":
    main()
