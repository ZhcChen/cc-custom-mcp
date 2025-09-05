# MCP Manager 全平台编译脚本

这个目录包含了用于编译 MCP Manager 到所有支持平台的脚本。

## 🎯 支持的平台

- **macOS**: ARM64 (Apple Silicon) 和 x64 (Intel)
- **Windows**: ARM64 和 x64
- **Linux**: ARM64 和 x64

## 📋 前置要求

### 必需依赖
- **Rust**: 最新稳定版本
- **Node.js**: 18+ 版本
- **Tauri CLI**: 会自动安装

### 平台特定要求

#### macOS
- Xcode Command Line Tools
- 对于交叉编译到其他平台，需要额外的工具链

#### Windows
- Visual Studio Build Tools 或 Visual Studio
- Windows SDK

#### Linux
- GCC 工具链
- 对于交叉编译，需要额外的链接器

## 🚀 使用方法

### 方法 1: 使用 npm 脚本 (推荐)

```bash
# 编译所有平台
npm run build:all

# Windows 用户使用批处理脚本
npm run build:all:win

# 使用 PowerShell 脚本
npm run build:all:ps

# 仅清理构建目录
npm run build:clean

# 编译特定平台
npm run build:macos
npm run build:windows
npm run build:linux
```

### 方法 2: 直接运行脚本

#### Unix/Linux/macOS
```bash
# 给脚本执行权限
chmod +x scripts/build-all.sh

# 运行脚本
./scripts/build-all.sh
```

#### Windows (批处理)
```cmd
scripts\build-all.bat
```

#### Windows (PowerShell)
```powershell
# 设置执行策略 (如果需要)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# 运行脚本
.\scripts\build-all.ps1

# 带参数运行
.\scripts\build-all.ps1 -SkipDeps -Target windows
```

## 📦 输出结构

编译完成后，会在 `dist/` 目录下生成以下结构：

```
dist/
├── macos-arm64/           # macOS ARM64 版本
├── macos-x64/             # macOS x64 版本
├── windows-arm64/         # Windows ARM64 版本
├── windows-x64/           # Windows x64 版本
├── linux-arm64/           # Linux ARM64 版本
├── linux-x64/             # Linux x64 版本
├── cc-custom-mcp-0.1.0-macos-arm64.tar.gz
├── cc-custom-mcp-0.1.0-macos-x64.tar.gz
├── cc-custom-mcp-0.1.0-windows-arm64.zip
├── cc-custom-mcp-0.1.0-windows-x64.zip
├── cc-custom-mcp-0.1.0-linux-arm64.tar.gz
└── cc-custom-mcp-0.1.0-linux-x64.tar.gz
```

## 🔧 脚本参数

### PowerShell 脚本参数

```powershell
# 跳过依赖检查和安装
.\scripts\build-all.ps1 -SkipDeps

# 仅清理构建目录
.\scripts\build-all.ps1 -CleanOnly

# 编译特定目标 (暂未实现)
.\scripts\build-all.ps1 -Target "windows"
```

## 🐛 故障排除

### 常见问题

#### 1. Rust 目标平台未安装
```bash
# 手动安装所有目标平台
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
rustup target add aarch64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu
```

#### 2. 交叉编译失败
- **macOS → Windows/Linux**: 需要额外的链接器和库
- **Windows → macOS/Linux**: 需要特殊的工具链
- **Linux → macOS/Windows**: 需要交叉编译工具链

#### 3. 权限问题 (Windows)
```powershell
# 设置 PowerShell 执行策略
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

#### 4. 编译内存不足
- 关闭其他应用程序
- 增加虚拟内存
- 使用 `--release` 模式编译

### 日志和调试

脚本会输出详细的编译日志，包括：
- 依赖检查结果
- 目标平台安装状态
- 每个平台的编译结果
- 最终的统计信息

## 📝 自定义编译

### 修改目标平台

编辑脚本中的目标平台列表：

```bash
# 在 build-all.sh 中
TARGETS=(
    "aarch64-apple-darwin:macos:arm64"
    "x86_64-apple-darwin:macos:x64"
    # 添加或移除目标平台
)
```

### 添加编译选项

在 `cargo tauri build` 命令中添加额外参数：

```bash
cargo tauri build --target $target --release --verbose
```

## 🔄 CI/CD 集成

这些脚本可以轻松集成到 CI/CD 流水线中：

### GitHub Actions 示例
```yaml
- name: Build All Platforms
  run: npm run build:all
  
- name: Upload Artifacts
  uses: actions/upload-artifact@v3
  with:
    name: releases
    path: dist/*.{zip,tar.gz}
```

### 本地自动化
```bash
# 创建定时任务
crontab -e
# 添加: 0 2 * * * cd /path/to/project && npm run build:all
```

## 📄 许可证

这些脚本遵循与主项目相同的许可证。
