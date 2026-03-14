<div align="center">

# PicoForge

<img src="static/appIcons/in.suyogtandel.picoforge.svg" width="512" height="512" alt="PicoForge Logo">

**一个开源的 Pico FIDO 安全密钥配置工具**

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![GitHub issues](https://img.shields.io/github/issues/librekeys/picoforge)](https://github.com/librekeys/picoforge/issues)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/librekeys/picoforge/release.yml)
[![Copr build status](https://copr.fedorainfracloud.org/coprs/lockedmutex/picoforge/package/picoforge/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/lockedmutex/picoforge/package/picoforge/)
[![GitHub stars](https://img.shields.io/github/stars/librekeys/picoforge)](https://github.com/librekeys/picoforge/stargazers)

</div>

> [!IMPORTANT]
> PicoForge 是一个独立的、由社区开发的工具，与官方的 [pico-fido](https://github.com/polhenarejos/pico-fido) 项目没有任何关联或背书关系。
> 本软件与官方闭源的 pico-fido 应用程序没有任何代码共享。
>
> 请查看应用程序的 [安装 Wiki](https://github.com/librekeys/picoforge/wiki/Installation) 获取 PicoForge 应用程序在您的系统上的安装指南。
>
> PicoForge 仅支持 PICO FIDO 系列固件的 v7.2 版本。对 v7.4 及以上版本的支持正在进行中。

## 关于

PicoForge 是一个现代化的桌面应用程序，用于配置和管理 Pico FIDO 安全密钥。它使用 Rust 和 GPUI 构建，提供直观的界面用于：

- 读取设备信息和固件详情
- 配置 USB VID/PID 和产品名称
- 调整 LED 设置（GPIO、亮度、驱动器）
- 管理安全功能（安全启动、固件锁定）（进行中）
- 实时系统日志和诊断
- 支持多种硬件变体和供应商

> **Beta 状态**：此应用程序目前处于积极开发阶段和 beta 阶段。用户应该会遇到一些 bug，我们鼓励您报告这些问题。该应用程序已在 Linux 和 Windows 10 上测试过，支持官方的 Raspberry Pi Pico2 和 ESP32-S3，目前仅支持 Pico FIDO 固件版本 7.2。

## 截图

<div align="center">

### 主界面
![PicoForge 主界面](data/screenshots/screenshot-1.webp)

### 通行密钥管理
![配置选项](data/screenshots/screenshot-2.webp)

### 配置界面
![设备管理](data/screenshots/screenshot-3.webp)

</div>

## 功能

- **设备配置** - 自定义 USB 标识符、LED 行为和硬件设置
- **安全管理** - 启用安全启动和固件验证（实验性，进行中）
- **实时监控** - 查看闪存使用情况、连接状态和系统日志
- **现代 UI** - 使用 Rust 和 GPUI 构建的干净、响应式界面
- **多供应商支持** - 兼容多种硬件变体
- **跨平台** - 支持 Windows、macOS 和 Linux

## 安装

请查看官方的 [PicoForge Wiki](https://github.com/librekeys/picoforge/wiki/Installation) 获取应用程序的安装信息。

## 使用方法

1. 连接您的智能卡读卡器
2. 插入您的 Pico FIDO 设备
3. 启动 PicoForge
4. 点击右上角的 **刷新** 按钮检测您的密钥
5. 通过侧边栏导航配置设置：
   - **主页** - 设备概览和快速操作
   - **配置** - USB 设置、LED 选项
   - **安全** - 安全启动管理（实验性）
   - **日志** - 实时事件监控
   - **关于** - 应用程序信息

## 要求

### 开发要求

要为 PicoForge 做出贡献，您需要：

- **[Rust](https://www.rust-lang.org/)** - 系统编程语言 (1.80+)
- **PC/SC 中间件**：
  - Linux: `pcscd`（通常预装）
  - macOS: 内置
  - Windows: 内置

## 从源码构建

### 1. 克隆仓库

```bash
git clone https://github.com/librekeys/picoforge.git
cd picoforge
```

### 2. 构建和运行

以开发模式运行应用程序：

```bash
cargo run
```

为生产环境构建：

```bash
cargo build --release
```

编译后的二进制文件将位于 `target/release/picoforge`（Linux/macOS）或 `target/release/picoforge.exe`（Windows）。

## 使用 Nix 构建和开发

[Nix](https://nixos.org/) 为开发者提供完整和一致的开发环境。

您可以使用 Nix 轻松构建和开发 picoforge。

### 1. 安装 Nix

按照 [安装指南](https://nixos.org/download/#download-nix) 和 [NixOS Wiki](https://wiki.nixos.org/wiki/Flakes#Setup) 安装 Nix 并启用 Flakes。

### 2. 构建和运行

#### a. 使用 Flakes

您可以使用单个命令构建和运行 PicoForge：

```bash
nix run github:librekeys/picoforge
```

或者简单地构建并链接到当前目录：

```bash
nix build github:librekeys/picoforge
```

> [!TIP]
> 您可以使用我们的二进制缓存来节省构建时间，方法是允许 Nix 设置 extra-substitutes。

#### b. 不使用 Flakes

下载包定义：

```bash
curl -LO https://raw.githubusercontent.com/librekeys/picoforge/main/package.nix
```

在包含 `package.nix` 的目录中运行以下命令：

```bash
nix-build -E 'with import <nixpkgs> {}; callPackage ./package.nix { }'
```

编译后的二进制文件将位于：`result/bin/picoforge`

### 3. 开发

您可以进入一个包含所有必需依赖项的开发环境。

#### a. 使用 Flakes

```bash
nix develop github:librekeys/picoforge
```

#### b. 不使用 Flakes

您可以使用位于仓库根目录的 `shell.nix` 文件：

```bash
nix-shell
```

然后您可以使用以下命令从源码构建和运行应用程序：

```bash
cargo run
```

## 项目结构

```
picoforge/
├── Cargo.toml                              # Rust 依赖项和项目元数据
├── Cargo.lock                              # Rust 依赖项锁定文件
├── Packager.toml                           # cargo-packager 配置
├── src/                                    # 源代码
│   ├── main.rs                             # 应用程序入口点
│   ├── logging.rs                          # 日志基础设施
│   ├── error.rs                            # 全局应用程序错误类型
│   ├── device/                             # 设备通信逻辑
│   │   ├── fido/                           # FIDO 实现
│   │   ├── rescue/                         # 救援模式处理
│   │   ├── io.rs                           # IO 工具
│   │   ├── mod.rs                          # 设备模块声明
│   │   └── types.rs                        # 设备数据类型
│   └── ui/                                 # GPUI 前端
│       ├── components/                     # 可重用的 UI 组件
│       ├── views/                          # 视图定义
│       ├── assets.rs                       # 资源加载器
│       ├── colors.rs                       # 颜色定义
│       ├── rootview.rs                     # 根视图容器
│       ├── types.rs                        # UI 相关类型
│       └── mod.rs                          # UI 模块声明
├── data/                                   # 应用程序数据
│   ├── in.suyogtandel.picoforge.desktop    # Linux 桌面入口文件
│   └── screenshots/                        # 截图用于文档
├── docs/                                   # 项目文档/wiki 文件
│   ├── Building.md                         # 从源码构建说明
│   ├── Home.md                             # Wiki 首页
│   ├── Installation.md                     # 安装指南
│   └── Troubleshooting.md                  # 故障排除常见问题
├── maintainers/                            # 包维护者的脚本和资源
│   └── scripts/                            # 自动化维护任务的实用脚本
│       ├── update.nix                      # Nix 更新脚本配置
│       └── update.py                       # 更新脚本实现
├── static/                                 # 静态应用程序资源
│   ├── appIcons/                           # 各种尺寸和格式的应用图标
│   └── icons/                              # GPUI 前端使用的内部 SVG 图标
├── themes/                                 # 应用程序主题
│   └── picoforge-zinc.json                 # Zinc 主题配置文件
├── flake.nix                               # Nix flake 配置
├── flake.lock                              # Nix flake 锁定文件
├── default.nix                             # Nix 包定义/shell
├── shell.nix                               # Nix 开发 shell
├── picoforge.spec                          # RPM Spec 文件
├── package.nix                             # Nix 包定义
├── ci.nix                                  # cachix 的 CI 配置
├── rustfmt.toml                            # Rust 格式化配置
├── CREDITS.md                              # 致谢
└── LICENSE                                 # 许可证
```

## 贡献

欢迎贡献（真的需要，请帮助我们）！

请查看 [CONTRIBUTING.md](.github/CONTRIBUTING.md) 文件获取完整的贡献流程和开发指南。

## 许可证

本项目采用 **GNU Affero 通用公共许可证 v3.0 (AGPL-3.0-only)** 许可。

查看 [LICENSE](LICENSE) 文件获取完整详情。

## 仓库维护者

- **Suyog Tandel** ([@lockedmutex](https://github.com/lockedmutex))
- **Fabrice Bellamy** ([@Lab-8916100448256](https://github.com/Lab-8916100448256)) - 联合维护者

## 包维护者

- **JetCookies** ([@jetcookies](https://github.com/jetcookies)): [Nix](https://nixos.org/) 包维护者
- **Suyog Tandel** ([@lockedmutex](https://github.com/lockedmutex)): [RPM](https://rpm.org/) 包和 Fedora Copr 仓库维护者

## 支持

- **Matrix**: [加入我们的 Matrix 房间](https://matrix.to/#/%23librekeys:matrix.org)
- **Discord**: [加入我们的 Discord 服务器](https://discord.gg/6wYBpSHJY2)
- **Issues**: [GitHub Issues](https://github.com/librekeys/picoforge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/librekeys/picoforge/discussions)

## 免责声明

> [!WARNING]
> PicoForge 是实验性软件，仍处于 Beta 阶段！
> 应用程序确实包含 bug，并且没有任何手段保证安全性。
>
> 它不支持 `pico-fido` 固件和 `pico-hsm` 暴露的所有功能。

> [!CAUTION]
> **USB VID/PID 注意**：本软件中提供的供应商预设包含 USB 供应商 ID (VID) 和产品 ID (PID)，这些是其各自所有者的知识产权。这些标识符仅用于测试和教育目的。您**不得**分发或商业销售使用您不拥有或许可的 VID/PID 组合的设备。商业分发需要从 USB 实施者论坛 ([usb.org](https://www.usb.org/getting-vendor-id)) 获取您自己的 VID，并遵守所有适用的商标和认证要求。未经授权使用可能违反 USB-IF 政策和知识产权法。PicoForge 开发者不对 USB 标识符的 misuse 承担任何责任。

---

<div align="center">

**由 LibreKeys 社区用 ❤️ 制作**

版权所有 © 2026 Suyog Tandel

</div>
