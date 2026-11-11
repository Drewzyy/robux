@echo off
setlocal
title BrewixGames update
rem Downloads the newest BrewixGames-windows-setup.exe from GitHub Releases
rem and installs it over the current copy. Your games, runs, and settings in
rem %USERPROFILE%\BrewixGames are kept; the vault is backed up before refresh.
set "SETUP=%TEMP%\BrewixGames-windows-setup.exe"
echo  Finding the newest BrewixGames release...
powershell -NoProfile -ExecutionPolicy Bypass -Command ^
  "$ErrorActionPreference='Stop';" ^
  "$r = Invoke-RestMethod -Headers @{'User-Agent'='BrewixGames'} 'https://api.github.com/repos/Drewzyy/robux/releases/latest';" ^
  "$a = $r.assets | Where-Object name -eq 'BrewixGames-windows-setup.exe';" ^
  "if (-not $a) { throw ('Release ' + $r.tag_name + ' has no Windows installer yet.') }" ^
  "Write-Host ('  Downloading ' + $r.tag_name + '...');" ^
  "Invoke-WebRequest -UseBasicParsing $a.browser_download_url -OutFile $env:TEMP\BrewixGames-windows-setup.exe"
if errorlevel 1 (
  echo  Could not download the installer.
  if /I not "%~1"=="/fromsetup" pause
  exit /b 1
)
taskkill /IM brewixgames.exe /F >nul 2>nul
echo  Installing...
start "" /wait "%SETUP%" /P
if errorlevel 1 (
  echo  The installer reported a problem.
  if /I not "%~1"=="/fromsetup" pause
  exit /b 1
)
del "%SETUP%" >nul 2>nul
echo  BrewixGames is up to date.
if /I not "%~1"=="/fromsetup" call "%~dp0RUN_WINDOWS.bat"
exit /b 0
