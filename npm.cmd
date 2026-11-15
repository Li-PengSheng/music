@echo off
setlocal
rem Node is provided by fnm. NODE_HOME wins, then node on PATH, then fnm's default alias.
if defined NODE_HOME if exist "%NODE_HOME%\npm.cmd" (
  set "MUSIC_MINE_NODE=%NODE_HOME%"
  goto run
)
for /f "delims=" %%P in ('where node 2^>nul') do (
  echo %%P | findstr /I /C:"WindowsApps" >nul
  if errorlevel 1 (
    set "MUSIC_MINE_NODE=%%~dpP"
    goto run
  )
)
if defined FNM_DIR if exist "%FNM_DIR%\aliases\default\npm.cmd" (
  set "MUSIC_MINE_NODE=%FNM_DIR%\aliases\default"
  goto run
)
echo Node.js could not be found. Install it with fnm or set NODE_HOME to its folder. 1>&2
exit /b 1
:run
set "PATH=%MUSIC_MINE_NODE%;%PATH%"
call "%MUSIC_MINE_NODE%\npm.cmd" %*
exit /b %errorlevel%
