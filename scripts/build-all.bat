@echo off
setlocal enabledelayedexpansion

REM MCP Manager - 全平台编译脚本 (Windows 版本)
REM 支持 macOS (arm64/x64), Windows (arm64/x64), Linux (arm64/x64)

echo.
echo 🚀 MCP Manager 全平台编译脚本
echo.

REM 获取版本号
for /f "tokens=2 delims=:" %%a in ('findstr "version" package.json') do (
    set "version_line=%%a"
)
set "version=!version_line:"=!"
set "version=!version:~1,-1!"
set "version=!version: =!"

set "PROJECT_NAME=cc-custom-mcp"
set "BUILD_DIR=dist"

echo 版本: %version%
echo 项目: %PROJECT_NAME%
echo.

REM 检查依赖
echo 📋 检查编译依赖...

REM 检查 Rust
rustc --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Rust 未安装，请先安装 Rust
    exit /b 1
)

REM 检查 Tauri CLI
cargo tauri --version >nul 2>&1
if errorlevel 1 (
    echo ⚠️  Tauri CLI 未安装，正在安装...
    cargo install tauri-cli
)

REM 检查 Node.js
node --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Node.js 未安装，请先安装 Node.js
    exit /b 1
)

echo ✅ 依赖检查完成
echo.

REM 安装 Rust 目标平台
echo 🎯 安装 Rust 目标平台...

rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu

echo ✅ 目标平台安装完成
echo.

REM 清理构建目录
echo 🧹 清理构建目录...
if exist %BUILD_DIR% rmdir /s /q %BUILD_DIR%
mkdir %BUILD_DIR%
echo ✅ 构建目录清理完成
echo.

REM 安装前端依赖
echo 📦 安装前端依赖...
npm install
echo ✅ 前端依赖安装完成
echo.

REM 编译函数
set "success_count=0"
set "total_count=6"

echo 🏗️  开始全平台编译...
echo.

REM macOS ARM64
echo 🔨 编译 macOS (arm64)...
mkdir %BUILD_DIR%\macos-arm64
cargo tauri build --target aarch64-apple-darwin
if errorlevel 1 (
    echo ❌ macOS (arm64) 编译失败
) else (
    echo ✅ macOS (arm64) 编译成功
    set /a success_count+=1
    if exist src-tauri\target\aarch64-apple-darwin\release\bundle\macos (
        xcopy src-tauri\target\aarch64-apple-darwin\release\bundle\macos\* %BUILD_DIR%\macos-arm64\ /E /I /Y >nul
    )
)
echo.

REM macOS x64
echo 🔨 编译 macOS (x64)...
mkdir %BUILD_DIR%\macos-x64
cargo tauri build --target x86_64-apple-darwin
if errorlevel 1 (
    echo ❌ macOS (x64) 编译失败
) else (
    echo ✅ macOS (x64) 编译成功
    set /a success_count+=1
    if exist src-tauri\target\x86_64-apple-darwin\release\bundle\macos (
        xcopy src-tauri\target\x86_64-apple-darwin\release\bundle\macos\* %BUILD_DIR%\macos-x64\ /E /I /Y >nul
    )
)
echo.

REM Windows ARM64
echo 🔨 编译 Windows (arm64)...
mkdir %BUILD_DIR%\windows-arm64
cargo tauri build --target aarch64-pc-windows-msvc
if errorlevel 1 (
    echo ❌ Windows (arm64) 编译失败
) else (
    echo ✅ Windows (arm64) 编译成功
    set /a success_count+=1
    if exist src-tauri\target\aarch64-pc-windows-msvc\release\%PROJECT_NAME%.exe (
        copy src-tauri\target\aarch64-pc-windows-msvc\release\%PROJECT_NAME%.exe %BUILD_DIR%\windows-arm64\ >nul
    )
    if exist src-tauri\target\aarch64-pc-windows-msvc\release\bundle\msi (
        xcopy src-tauri\target\aarch64-pc-windows-msvc\release\bundle\msi\* %BUILD_DIR%\windows-arm64\ /E /I /Y >nul
    )
)
echo.

REM Windows x64
echo 🔨 编译 Windows (x64)...
mkdir %BUILD_DIR%\windows-x64
cargo tauri build --target x86_64-pc-windows-msvc
if errorlevel 1 (
    echo ❌ Windows (x64) 编译失败
) else (
    echo ✅ Windows (x64) 编译成功
    set /a success_count+=1
    if exist src-tauri\target\x86_64-pc-windows-msvc\release\%PROJECT_NAME%.exe (
        copy src-tauri\target\x86_64-pc-windows-msvc\release\%PROJECT_NAME%.exe %BUILD_DIR%\windows-x64\ >nul
    )
    if exist src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi (
        xcopy src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\* %BUILD_DIR%\windows-x64\ /E /I /Y >nul
    )
)
echo.

REM Linux ARM64
echo 🔨 编译 Linux (arm64)...
mkdir %BUILD_DIR%\linux-arm64
cargo tauri build --target aarch64-unknown-linux-gnu
if errorlevel 1 (
    echo ❌ Linux (arm64) 编译失败
) else (
    echo ✅ Linux (arm64) 编译成功
    set /a success_count+=1
    if exist src-tauri\target\aarch64-unknown-linux-gnu\release\%PROJECT_NAME% (
        copy src-tauri\target\aarch64-unknown-linux-gnu\release\%PROJECT_NAME% %BUILD_DIR%\linux-arm64\ >nul
    )
    if exist src-tauri\target\aarch64-unknown-linux-gnu\release\bundle\deb (
        xcopy src-tauri\target\aarch64-unknown-linux-gnu\release\bundle\deb\* %BUILD_DIR%\linux-arm64\ /E /I /Y >nul
    )
)
echo.

REM Linux x64
echo 🔨 编译 Linux (x64)...
mkdir %BUILD_DIR%\linux-x64
cargo tauri build --target x86_64-unknown-linux-gnu
if errorlevel 1 (
    echo ❌ Linux (x64) 编译失败
) else (
    echo ✅ Linux (x64) 编译成功
    set /a success_count+=1
    if exist src-tauri\target\x86_64-unknown-linux-gnu\release\%PROJECT_NAME% (
        copy src-tauri\target\x86_64-unknown-linux-gnu\release\%PROJECT_NAME% %BUILD_DIR%\linux-x64\ >nul
    )
    if exist src-tauri\target\x86_64-unknown-linux-gnu\release\bundle\deb (
        xcopy src-tauri\target\x86_64-unknown-linux-gnu\release\bundle\deb\* %BUILD_DIR%\linux-x64\ /E /I /Y >nul
    )
)
echo.

REM 编译统计
echo 📊 编译统计:
echo ✅ 成功: %success_count%/%total_count%
echo.

if %success_count% equ %total_count% (
    echo 🎉 所有平台编译成功！
) else (
    echo ⚠️  部分平台编译失败，请检查错误信息
)
echo.

REM 创建发布包
echo 📦 创建发布包...
cd %BUILD_DIR%

for /d %%d in (*) do (
    echo 📦 打包 %%d...
    powershell -command "Compress-Archive -Path '%%d\*' -DestinationPath '%PROJECT_NAME%-%version%-%%d.zip' -Force"
)

cd ..
echo ✅ 发布包创建完成
echo.

REM 显示结果
echo 📁 编译结果:
echo 构建目录: %BUILD_DIR%
echo.
dir %BUILD_DIR%
echo.
echo 🎉 编译完成！
echo 版本: %version%
echo 时间: %date% %time%

pause
