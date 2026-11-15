@echo off
setlocal
rem Allow Windows build runners to use Zed's installed Node without a global PATH change.
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
for /d %%D in ("%LOCALAPPDATA%\Zed\node\node-*-win-x64") do (
  if exist "%%~D\node.exe" if exist "%%~D\npm.cmd" (
    set "MUSIC_MINE_NODE=%%~D"
    goto run
  )
)
echo Node.js could not be found. Install Node.js or set NODE_HOME to its folder. 1>&2
exit /b 1
:run
set "PATH=%MUSIC_MINE_NODE%;%PATH%"
call "%MUSIC_MINE_NODE%\npm.cmd" %*
exit /b %errorlevel%
