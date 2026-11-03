<div align="center">

# outpost2

**《前哨2：分歧的命运》(Sierra On-Line / Dynamix, 1997) 逆向工程与现代 WebAssembly 移植 (Rust, HTML5 Canvas & Web Audio API)**

[![Rust](https://img.shields.io/badge/rust-1.80+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/wasm-webassembly-654ff0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![CI](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml/badge.svg)](https://github.com/pedryvo/outpost2/actions/workflows/ci.yml)
[![版本](https://img.shields.io/badge/版本-v1.0.0-blue)](docs/RELEASE_V1.0.md)
[![状态](https://img.shields.io/badge/状态-已完成-brightgreen)](#-开发进展)
[![测试](https://img.shields.io/badge/测试-112%20项全部通过-brightgreen)](docs/formats/integration_testing_and_qa.md)
[![阶段](https://img.shields.io/badge/阶段-v1.0.0_%E2%80%94_生产就绪-brightgreen)](docs/PLAN.md)
[![平台](https://img.shields.io/badge/平台-Web_%7C_PWA_%7C_Vercel_%7C_Mobile-lightgrey)](#-运行游戏)
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
| **战争迷雾与战术动态雷达** | ✅ | 三层视野状态与实时雷达态势感知（地下矿井、激光/轨道炮弹道光束、爆炸冲击波） · [`docs/formats/fog_of_war.md`](docs/formats/fog_of_war.md) |
| **殖民地模拟核心** | ✅ | 伊甸与普利茅斯物流：电力平衡、生命维持、采矿与地质灾害 · [`docs/formats/disasters.md`](docs/formats/disasters.md) |
| **建筑与承压运输管道** | ✅ | 模块化建筑组装与正交加压运输管道网络 · [`docs/formats/structures_and_tubes.md`](docs/formats/structures_and_tubes.md) |
| **交互式建造厂抽屉** | ✅ | 13 大建筑滑动蓝图抽屉、全息虚影预览与连接管道自动投射 |
| **地热发电厂 Geothermal**   | ✅ | 连续基载水热清洁能源（+450 kW），双冷却塔与动态升腾蒸汽羽流 |
| **岗哨防御炮塔 Guard Post** | ✅ | 自动化基地外围防御炮塔，配备双联防空反装甲速射炮与激光瞄准 |
| **矿石冶炼厂与排气烟羽** | ✅ | 熔炉金属熔池呼吸光效、发热循环动画与烟囱升腾排气烟羽 |
| **建筑拆解与资源回收** | ✅ | 拆除受损或废弃建筑，返还 50% 矿石成本并重平衡电网负荷 |
| **单位控制与八方向 A\*** | ✅ | 等轴测八方向 A* 路径规划、右键指令 · [`docs/ENGINE_ARCHITECTURE.md`](docs/ENGINE_ARCHITECTURE.md) |
| **框选多选与战术标线** | ✅ | 经典多单位拖拽框选、高科技直角瞄准标线与 Shift 增选 |
| **虎式重型坦克与电磁轨道炮** | ✅ | 双履带重装甲突击坦克，配备超音速电磁轨道炮与磁加速线圈特效 |
| **Robo-Miner 矿井钻塔建造** | ✅ | 在未开采矿标上就地开凿地下矿井，激活货运卡车全自动矿流物流 · [`docs/formats/mining_and_logistics.md`](docs/formats/mining_and_logistics.md) |
| **战车维修与战地保障** | ✅ | 移动维修车与维修工厂阵列，发射翡翠纳米修复光束治疗装甲 |
| **工程车工兵建造与推土清障** | ✅ | 强化陶瓷防御墙修筑、战地管道延伸、枯萎病净化与旧墙/管道推土拆除 |
| **ConVec 建造车套件部署** | ✅ | 战地建造车承载预制建筑套件，目标点展开并地基锁死就地落成 |
| **战术警戒与双向巡逻** | ✅ | 自动扇区警戒锚点、双向巡逻路径规划与自动化索敌还击 |
| **行星灾害与天体灾变** | ✅ | 蔓延性枯萎病微生物真菌、火山熔岩裂隙、大气尘暴旋风与动能陨石雨撞击 |
| **红色警报警报声与敌军突袭**| ✅ | 指挥中心双音频道紧急红色警报警报器，普利茅斯协同战术突击波次进攻 |
| **多学科科研矩阵** | ✅ | 11 大科技前置解锁树（火山学、电磁轨道炮、点防御）与被动增益 · [`docs/formats/tech_tree.md`](docs/formats/tech_tree.md) |
| **WebRTC P2P 联机与房间大厅**| ✅ | 无服务器直连对战（Base64 SDP 交换、定步长指令同步、战术聊天与大厅状态机） · [`docs/formats/save_games_and_lobby.md`](docs/formats/save_games_and_lobby.md) |
| **战术 AI 与遭遇战自治系统** | ✅ | 3 大 AI 战术流派（突击狂魔、防御堡垒、经济科技流）、基地规划与战术小队协同突袭 · [`docs/formats/ai_and_personalities.md`](docs/formats/ai_and_personalities.md) |
| **动态任务事件与脚本引擎**   | ✅ | 太阳日计时器、空间区域触发器、建筑伤亡阈值判定与行星灾变动态发生器 · [`docs/formats/dynamic_events_and_scripting.md`](docs/formats/dynamic_events_and_scripting.md) |
| **完整战役关卡 (伊甸 1-3 & 普利茅斯 1-3)**| ✅ | 包含伊甸第 3 关（*火山飞船逃亡*）与普利茅斯第 3 关（*黑曜石峭壁突击*）在内的原生地图 · [`docs/formats/campaign_progression_and_debrief.md`](docs/formats/campaign_progression_and_debrief.md) |
| **精灵渲染与多阶段爆炸动画** | ✅ | 八方向底盘朝向 ($45^\circ$)、炮塔独立瞄准、采矿车旋转钻头与 12 阶火球膨胀爆炸 · [`docs/formats/sprites_and_animation.md`](docs/formats/sprites_and_animation.md) |
| **原生 SoundBank 与战术语音包**| ✅ | 252 槽位经典声音映射表、Kathy Cullis 原版战场无线电播报与零延迟 WASM 预加载 · [`docs/formats/sound_and_voices.md`](docs/formats/sound_and_voices.md) |
| **现代 WebHUD 与雷达小地图** | ✅ | 战争迷雾雷达、行星灾害警报标点（岩浆、尘暴、陨石）、车间/科研悬浮窗与全快捷键响应 · [`docs/formats/ui_and_hud.md`](docs/formats/ui_and_hud.md) |
| **全状态持久化与 .op2save 导出** | ✅ | `LocalStorage` 多存档位（1-5 位、F5 快存、F9 快读、100 太阳日自动存档）与独立文件导出 · [`docs/formats/save_games_and_lobby.md`](docs/formats/save_games_and_lobby.md) |
| **WASM 打包与移动端触控优化**| ✅ | 高清晰度 Retina 缩放、手势轻触解锁 Web Audio、双指缩放/平移与极致发布编译包 · [`docs/formats/wasm_packaging_and_web_runtime.md`](docs/formats/wasm_packaging_and_web_runtime.md) |
| **交互式复古主菜单与战役简报系统** | ✅ | 纯 Canvas 原生渲染复古主菜单、战役剧场选择器、Kathy Cullis 原版战术简报语音与 CRT 扫描线显像管特效 · [`docs/formats/main_menu_and_briefing.md`](docs/formats/main_menu_and_briefing.md) |
| **程序化遭遇战地图生成器 (4大生物群落与对称性)** | ✅ | 竞技级程序化地图生成器：4大群落（火山盆地、峡谷峭壁、枯萎荒原、黑曜石山脊）、双向/四向/镜像对称性与 A* 通行度保障 · [`docs/formats/procedural_map_generator.md`](docs/formats/procedural_map_generator.md) |
| **PWA 离线运行 Service Worker 与音频多通道增益** | ✅ | Cache-First 离线缓存服务工作线程 (`sw.js`)、PWA 应用图标清单、Web Audio 主音量/音效/音乐三路独立增益节点与 0.5x–4.0x 仿真倍速控制 · [`docs/formats/pwa_and_offline_deployment.md`](docs/formats/pwa_and_offline_deployment.md) |
| **WebRTC 联机战局内网络遥测与阵营战术聊天** | ✅ | 战局顶部实时 RTT 往返延迟状态药丸、定步长指令缓冲区监控与阵营专属颜色战术聊天 (`[Eden]` 蓝绿色 / `[Plymouth]` 红色) · [`docs/formats/multiplayer_matchmaking_and_telemetry.md`](docs/formats/multiplayer_matchmaking_and_telemetry.md) |
| **游戏内六语言原生本地化引擎** | ✅ | 覆盖英语、葡萄牙语、西班牙语、德语、简体中文、俄语的强类型本地化系统，全面覆盖主菜单、HUD 遥测、战役简报与结算军衔 · [`docs/formats/i18n_and_localization.md`](docs/formats/i18n_and_localization.md) |
| **112 项单元与端到端集成测试 QA 矩阵** | ✅ | 108 个单元测试 + 4 大端到端集成测试，严格验证战役全生命周期、多 AI 遭遇战自治演化、WebRTC 定步长网络同步与存档往返一致性 · [`docs/formats/integration_testing_and_qa.md`](docs/formats/integration_testing_and_qa.md) |
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
