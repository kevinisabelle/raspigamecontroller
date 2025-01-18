@echo off
REM Batch file to build Zig project for armv7l target

REM Path to your Zig executable
set ZIG_PATH=zig

REM Path to the sysroot (replace with your sysroot location)
set SYSROOT=sysroot

REM Path to your Zig source file
set SOURCE=src\gamepad.zig

REM Build output directory
set OUTPUT_DIR=bin

REM Ensure output directory exists
if not exist "%OUTPUT_DIR%" mkdir "%OUTPUT_DIR%"

REM Run Zig build command
%ZIG_PATH% build-exe ^
    -target arm-linux-gnueabihf ^
    --sysroot "%SYSROOT%" ^
    -lc -lbluetooth -lpigpio ^
    -I "%SYSROOT%\usr\include" ^
    -L "%SYSROOT%\usr\lib" ^
    -L "%SYSROOT%\lib" ^
    -L "%SYSROOT%\usr\lib\arm-linux-gnueabihf" ^
    -femit-bin="%OUTPUT_DIR%\main" ^
    -freference-trace=4 ^
    %SOURCE%

REM Check build result
if %ERRORLEVEL% EQU 0 (
    echo Build succeeded!
) else (
    echo Build failed. Check the error messages above.
)
pause