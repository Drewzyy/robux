@echo off
setlocal
rem Opens BrewixGames. Installs it first if it is not on this PC yet.
for %%P in (
  "%LOCALAPPDATA%\BrewixGames\brewixgames.exe"
  "%LOCALAPPDATA%\BrewixGames\BrewixGames.exe"
  "%LOCALAPPDATA%\Programs\BrewixGames\brewixgames.exe"
  "%ProgramFiles%\BrewixGames\brewixgames.exe"
  "%ProgramFiles%\BrewixGames\BrewixGames.exe"
) do (
  if exist %%P (
    start "" %%P
    exit /b 0
  )
)
echo  BrewixGames is not installed yet. Running setup...
call "%~dp0INSTALL_WINDOWS.bat"
