@echo off
setlocal
title BrewixGames setup
echo.
echo  BrewixGames setup
echo  -----------------
echo  This downloads the newest BrewixGames from GitHub and installs it for
echo  your Windows user. No admin rights needed. Nothing is published anywhere.
echo.

call "%~dp0UPDATE_WINDOWS.bat" /fromsetup
if errorlevel 1 goto :fail

echo.
where claude >nul 2>nul
if errorlevel 1 (
  echo  Claude Code is not installed. BrewixGames uses it to run the agents
  echo  with your Claude subscription. No API key is needed.
  choice /C YN /M "  Install Claude Code now"
  if not errorlevel 2 powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://claude.ai/install.ps1 | iex"
) else (
  echo  Claude Code found.
)

echo.
where ollama >nul 2>nul
if errorlevel 1 (
  echo  Optional: Ollama runs free local AI models on this PC instead of Claude.
  echo  It only installs the program. Models are downloaded later, only when
  echo  you pick one in BrewixGames Settings.
  choice /C YN /M "  Install Ollama now"
  if not errorlevel 2 winget install --id Ollama.Ollama -e --accept-source-agreements --accept-package-agreements
) else (
  echo  Ollama found.
)

echo.
echo  Done. Opening BrewixGames...
call "%~dp0RUN_WINDOWS.bat"
exit /b 0

:fail
echo.
echo  Setup did not finish. Check your internet connection and try again.
pause
exit /b 1
