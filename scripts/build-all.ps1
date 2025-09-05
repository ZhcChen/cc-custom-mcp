# MCP Manager - 全平台编译脚本 (PowerShell 版本)
# 支持 macOS (arm64/x64), Windows (arm64/x64), Linux (arm64/x64)

param(
    [switch]$SkipDeps,
    [switch]$CleanOnly,
    [string]$Target = "all"
)

# 设置错误处理
$ErrorActionPreference = "Stop"

# 项目信息
$PROJECT_NAME = "cc-custom-mcp"
$VERSION = (Get-Content package.json | ConvertFrom-Json).version
$BUILD_DIR = "dist"

# 颜色函数
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    
    $colorMap = @{
        "Red" = "Red"
        "Green" = "Green"
        "Yellow" = "Yellow"
        "Blue" = "Cyan"
        "White" = "White"
    }
    
    Write-Host $Message -ForegroundColor $colorMap[$Color]
}

Write-ColorOutput "🚀 MCP Manager 全平台编译脚本" "Blue"
Write-ColorOutput "版本: $VERSION" "Blue"
Write-ColorOutput "项目: $PROJECT_NAME" "Blue"
Write-Host ""

# 检查依赖
function Test-Dependencies {
    Write-ColorOutput "📋 检查编译依赖..." "Yellow"
    
    # 检查 Rust
    try {
        $null = Get-Command rustc -ErrorAction Stop
    } catch {
        Write-ColorOutput "❌ Rust 未安装，请先安装 Rust" "Red"
        exit 1
    }
    
    # 检查 Tauri CLI
    try {
        $null = Get-Command cargo-tauri -ErrorAction Stop
    } catch {
        Write-ColorOutput "⚠️  Tauri CLI 未安装，正在安装..." "Yellow"
        cargo install tauri-cli
    }
    
    # 检查 Node.js
    try {
        $null = Get-Command node -ErrorAction Stop
    } catch {
        Write-ColorOutput "❌ Node.js 未安装，请先安装 Node.js" "Red"
        exit 1
    }
    
    Write-ColorOutput "✅ 依赖检查完成" "Green"
}

# 安装 Rust 目标平台
function Install-RustTargets {
    Write-ColorOutput "🎯 安装 Rust 目标平台..." "Yellow"
    
    $targets = @(
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "aarch64-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-unknown-linux-gnu",
        "x86_64-unknown-linux-gnu"
    )
    
    foreach ($target in $targets) {
        rustup target add $target
    }
    
    Write-ColorOutput "✅ 目标平台安装完成" "Green"
}

# 清理构建目录
function Clear-BuildDirectory {
    Write-ColorOutput "🧹 清理构建目录..." "Yellow"
    
    if (Test-Path $BUILD_DIR) {
        Remove-Item $BUILD_DIR -Recurse -Force
    }
    New-Item -ItemType Directory -Path $BUILD_DIR -Force | Out-Null
    
    Write-ColorOutput "✅ 构建目录清理完成" "Green"
}

# 安装前端依赖
function Install-Dependencies {
    Write-ColorOutput "📦 安装前端依赖..." "Yellow"
    npm install
    Write-ColorOutput "✅ 前端依赖安装完成" "Green"
}

# 编译单个目标
function Build-Target {
    param(
        [string]$Target,
        [string]$Platform,
        [string]$Arch
    )
    
    Write-ColorOutput "🔨 编译 $Platform ($Arch)..." "Blue"
    
    $outputDir = "$BUILD_DIR\$Platform-$Arch"
    New-Item -ItemType Directory -Path $outputDir -Force | Out-Null
    
    try {
        cargo tauri build --target $Target
        Write-ColorOutput "✅ $Platform ($Arch) 编译成功" "Green"
        
        # 复制编译结果
        switch ($Platform) {
            "macos" {
                $sourceDir = "src-tauri\target\$Target\release\bundle\macos"
                if (Test-Path $sourceDir) {
                    Copy-Item "$sourceDir\*" $outputDir -Recurse -Force
                }
            }
            "windows" {
                $exePath = "src-tauri\target\$Target\release\$PROJECT_NAME.exe"
                if (Test-Path $exePath) {
                    Copy-Item $exePath $outputDir -Force
                }
                $msiDir = "src-tauri\target\$Target\release\bundle\msi"
                if (Test-Path $msiDir) {
                    Copy-Item "$msiDir\*" $outputDir -Recurse -Force
                }
            }
            "linux" {
                $binPath = "src-tauri\target\$Target\release\$PROJECT_NAME"
                if (Test-Path $binPath) {
                    Copy-Item $binPath $outputDir -Force
                }
                $debDir = "src-tauri\target\$Target\release\bundle\deb"
                if (Test-Path $debDir) {
                    Copy-Item "$debDir\*" $outputDir -Recurse -Force
                }
                $appImageDir = "src-tauri\target\$Target\release\bundle\appimage"
                if (Test-Path $appImageDir) {
                    Copy-Item "$appImageDir\*" $outputDir -Recurse -Force
                }
            }
        }
        
        return $true
    } catch {
        Write-ColorOutput "❌ $Platform ($Arch) 编译失败: $($_.Exception.Message)" "Red"
        return $false
    }
}

# 主编译函数
function Build-All {
    Write-ColorOutput "🏗️  开始全平台编译..." "Yellow"
    
    $successCount = 0
    $totalCount = 6
    
    $buildTargets = @(
        @{ Target = "aarch64-apple-darwin"; Platform = "macos"; Arch = "arm64" },
        @{ Target = "x86_64-apple-darwin"; Platform = "macos"; Arch = "x64" },
        @{ Target = "aarch64-pc-windows-msvc"; Platform = "windows"; Arch = "arm64" },
        @{ Target = "x86_64-pc-windows-msvc"; Platform = "windows"; Arch = "x64" },
        @{ Target = "aarch64-unknown-linux-gnu"; Platform = "linux"; Arch = "arm64" },
        @{ Target = "x86_64-unknown-linux-gnu"; Platform = "linux"; Arch = "x64" }
    )
    
    foreach ($buildTarget in $buildTargets) {
        if (Build-Target $buildTarget.Target $buildTarget.Platform $buildTarget.Arch) {
            $successCount++
        }
    }
    
    Write-Host ""
    Write-ColorOutput "📊 编译统计:" "Blue"
    Write-ColorOutput "✅ 成功: $successCount/$totalCount" "Green"
    
    if ($successCount -eq $totalCount) {
        Write-ColorOutput "🎉 所有平台编译成功！" "Green"
    } else {
        Write-ColorOutput "⚠️  部分平台编译失败，请检查错误信息" "Yellow"
    }
}

# 创建发布包
function New-ReleasePackages {
    Write-ColorOutput "📦 创建发布包..." "Yellow"
    
    Push-Location $BUILD_DIR
    
    Get-ChildItem -Directory | ForEach-Object {
        $platformName = $_.Name
        Write-ColorOutput "📦 打包 $platformName..." "Blue"
        
        $archiveName = "$PROJECT_NAME-$VERSION-$platformName.zip"
        Compress-Archive -Path "$platformName\*" -DestinationPath $archiveName -Force
    }
    
    Pop-Location
    Write-ColorOutput "✅ 发布包创建完成" "Green"
}

# 显示结果
function Show-Results {
    Write-Host ""
    Write-ColorOutput "📁 编译结果:" "Blue"
    Write-ColorOutput "构建目录: $BUILD_DIR" "Blue"
    Write-Host ""
    
    if (Test-Path $BUILD_DIR) {
        Get-ChildItem $BUILD_DIR | Format-Table Name, Length, LastWriteTime
    }
    
    Write-Host ""
    Write-ColorOutput "🎉 编译完成！" "Green"
    Write-ColorOutput "版本: $VERSION" "Blue"
    Write-ColorOutput "时间: $(Get-Date)" "Blue"
}

# 主函数
function Main {
    try {
        Write-ColorOutput "开始编译流程..." "Blue"
        
        if (-not $SkipDeps) {
            Test-Dependencies
            Install-RustTargets
        }
        
        Clear-BuildDirectory
        
        if ($CleanOnly) {
            Write-ColorOutput "仅清理模式，编译流程结束" "Yellow"
            return
        }
        
        if (-not $SkipDeps) {
            Install-Dependencies
        }
        
        Build-All
        New-ReleasePackages
        Show-Results
        
    } catch {
        Write-ColorOutput "❌ 编译过程中发生错误: $($_.Exception.Message)" "Red"
        exit 1
    }
}

# 执行主函数
Main
