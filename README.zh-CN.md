<div align="center">

# outpost2

**《前哨2：分歧的命运》(Sierra On-Line / Dynamix, 1997) 逆向工程与现代 WebAssembly 移植 (Rust, HTML5 Canvas & Web Audio API)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![状态](https://img.shields.io/badge/状态-已完成-brightgreen)](#-开发进展)
[![阶段](https://img.shields.io/badge/阶段-7_%E2%80%94_战役与逃逸飞船-blue)](docs/PLAN.md)
[![平台](https://img.shields.io/badge/平台-Web_%7C_Vercel_%7C_Mobile-lightgrey)](#-运行游戏)
[![开源协议: GPL-3.0](https://img.shields.io/badge/协议-GPL--3.0--or--later-blue)](LICENSE)

_Sierra 和 Dynamix 经典硬科幻即时战略游戏，<br>专为现代浏览器、桌面与移动触控端原生重制。_

[English](README.md) · [Português](README.pt-BR.md) · [Español](README.es.md) · [Deutsch](README.de.md) · **中文** · [Русский](README.ru.md)

</div>

---

## 🚀 项目简介

**outpost2** 借助 **REA** (Reverse Engineer Anything) 与 Ghidra 工具对 1997 年发行的原版游戏《前哨2：分歧的命运》(`Outpost2.exe`) 及其数据包 (`art.vol`, `maps.vol`, `sound.vol`, `tech.vol`) 进行了深入逆向工程与类继承体系还原，采用现代化的 **Rust 与 WebAssembly** 技术栈完全重新实现。

该项目旨在无需任何模拟器或 Windows 兼容层的前提下，以原生性能在浏览器中体验原汁原味的科幻基地建设、地质灾害规避、全向寻路、自动化采矿运输与星际逃逸飞船制造。

---

## ✨ 核心特性

- 🦀 **纯 Rust & WebAssembly 架构**：零模拟器开销，通过 LLVM LTO 极致压缩，独立 WASM 体积仅约 370 KB。
- 🗺️ **高分辨率等轴测网格引擎**：支持平滑缩放、亚像素相机平移、昼夜交替日照光效与 2.5D 悬崖地形立体阴影。
- ⚙️ **科幻指挥 HUD**：实时显示殖民地遥测数据、电力收支、生命支持系统、矿石储量与交互式战术雷达小地图。
- 🚀 **逃逸飞船组装系统**：抢在熔岩与纳米枯萎物吞噬星球之前，四阶段建造逃逸飞船（指挥舰桥、低温冷冻舱、聚变推进引擎与辐射防御护盾）。
- ⚔️ **多战役战场选择器**：内置伊甸园战役、普利茅斯战役与学院训练模拟地图 (`eden01.map`, `ply01.map`, `tutorial.map`) 及敌对 AI 指挥系统。
- 🏭 **工厂生产队列与等离子焊接特效**：战车车间生产装甲车辆时展示动态等离子弧光焊接与 HUD 实时进度条。
- 📱 **移动与触摸优化**：全面支持手机与平板触控拖拽移动、双指捏合缩放（Pinch-to-zoom）。
- 🔊 **Web Audio 与战术电台**：程序化环境音效、背景音乐与 Web Speech API 战术语音警报。
- 🌍 **实时六语言切换 (i18n)**：支持无刷新即时切换 (**English**, **Português do Brasil**, **Español**, **Deutsch**, **简体中文**, **Русский**)。

---

## 📊 开发进展

| 领域 | 状态 | 详细说明 |
|---|:---:|---|
| **规划与架构设计** | ✅ | 详尽的项目计划与路线图 · [`docs/PLAN.md`](docs/PLAN.md) |
| **REA 逆向工程** | ✅ | 分析 `Outpost2.exe`，提取 509 个导出 C++ 引擎符号 · [`re/engine_symbols.json`](re/engine_symbols.json) |
| **`.vol` 数据包解析器** | ✅ | 纯 Rust 实现的高性能流式容器解包器 · [`docs/formats/vol_archive.md`](docs/formats/vol_archive.md) |
| **调色板与精灵图解码** | ✅ | 256 色主调色板与 5,390 个精灵定义 · [`docs/formats/prt_format.md`](docs/formats/prt_format.md) |
| **等轴测渲染器** | ✅ | 动态网格、昼夜交替日照与 2.5D 悬崖光影 · [`docs/formats/palette_and_tiles.md`](docs/formats/palette_and_tiles.md) |
| **战争迷雾与战术雷达** | ✅ | 三层视野状态（未探索、已探索、完全可见） · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **殖民地模拟核心** | ✅ | 伊甸与普利茅斯物流：电力平衡、生命维持、采矿与地质灾害 · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **建筑与承压运输管道** | ✅ | 模块化建筑组装与正交加压运输管道网络 · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **交互式建造厂抽屉** | ✅ | 11 大建筑滑动蓝图抽屉、全息虚影预览与连接管道自动投射 |
| **建筑拆解与资源回收** | ✅ | 拆除受损或废弃建筑，返还 50% 矿石成本并重平衡电网负荷 |
| **单位控制与八方向 A\*** | ✅ | 等轴测八方向 A* 路径规划、右键指令 · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **框选多选与战术标线** | ✅ | 经典多单位拖拽框选、高科技直角瞄准标线与 Shift 增选 |
| **战车维修与战地保障** | ✅ | 移动维修车与维修工厂阵列，发射翡翠纳米修复光束治疗装甲 |
| **工程车工兵建造** | ✅ | 强化陶瓷防御墙修筑、战地管道延伸与枯萎病危害净化清扫 |
| **ConVec 建造车套件部署** | ✅ | 战地建造车承载预制建筑套件，目标点展开并地基锁死就地落成 |
| **战术警戒与双向巡逻** | ✅ | 自动扇区警戒锚点、双向巡逻路径规划与自动化索敌还击 |
| **行星灾害与天体灾变** | ✅ | 蔓延性枯萎病微生物真菌、火山熔岩裂隙、大气尘暴旋风与动能陨石雨撞击 |
| **多学科科研矩阵** | ✅ | 分类科技树交互弹窗、被动殖民地加成（强化装甲、脉冲激光、矿石精炼） · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **多战役战场选择器** | ✅ | 真实战役战场（伊甸 1、普利茅斯 1、学院教学） · [`docs/formats/campaigns_and_scenarios.md`](docs/formats/campaigns_and_scenarios.md) |
| **工厂生产队列与焊接** | ✅ | 战车制造排队机制、等离子火花与 HUD 进度条 |
| **星际逃逸飞船建造** | ✅ | 四阶段飞船组装与生命支持人口动态机制 · [`docs/formats/starship_assembly.md`](docs/formats/starship_assembly.md) |
| **状态持久化与触控** | ✅ | 浏览器本地极速存取档与移动端手势控制 |
| **音频引擎与战斗音效** | ✅ | Web Audio 纯程序化音效合成（激光、爆炸、建造蜂鸣）与经典原声音频 |
| **多语言本地化 (i18n)** | ✅ | 完美支持 EN, PT-BR, ES, DE, ZH-CN, RU |
| **Vercel & WASM 构建** | ✅ | 基于 Trunk 的自动化发布优化构建 (`vercel.json`) |

---

## 🎮 运行游戏

```bash
# 通过 mise 自动配置开发环境
mise install

# 启动本地热重载开发服务器
mise exec -- trunk serve

# 构建生产级优化发布版本
mise exec -- trunk build --release
```

---

## 📜 授权协议

**outpost2** 源代码基于 **GNU General Public License v3.0 或更高版本** 开源 ([LICENSE](LICENSE))。
