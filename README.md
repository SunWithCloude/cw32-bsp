# cw32-bsp

武汉芯源（CMSemicon）**CW32 系列** Cortex-M0+ MCU 的**通用最小系统 BSP** 合集 —— 全部 11 个家族一个 crate，基于 [embassy-cw32](https://github.com/SunWithCloude/Cw32l012_embassy)。

每个 BSP 提供：

- **引脚复用全表**（`pins` 模块）：UART1/SPI1/I2C1 的全部可用引脚组合与 AF 编号，逐条带厂商 SDK 宏出处；数据源为官方数据手册 AF 表的定位提取（[fan2nd/cw32-hal2 `cw32-data/af/`](https://github.com/fan2nd/cw32-hal2)），生成脚本 `gen_pins.py`。
- **`bsp_pin!` 宏**：按端口字母把 `(PORT, 引脚号)` 落成 embassy GPIO `Pin`（展开到家族 PAC 的 Peripherals 端口字段；未被 PAC 收编的端口组合以 `[PAC]` 标注）。
- **blink / uart_echo 示例**：引脚注入式模板，改两个常量即可适配你的板子。
- 家族事实（复位默认时钟，数据手册核实）与调试串口组装入口。

## 家族矩阵

| 家族 | crate | 复位时钟 | 实板支持 |
|---|---|---|---|
| CW32F003 | `cw32f003-bsp` | 8MHz | — |
| CW32F020 | `cw32f020-bsp` | 8MHz | — |
| CW32F030 | `cw32f030-bsp` | 8MHz | **立创天猛星 CW32F030C8T7**（LED PC13、UART1 PA08/PA09 AF2——实机验证） |
| CW32L010 | `cw32l010-bsp` | 4MHz | — |
| CW32L011 | `cw32l011-bsp` | 4MHz | — |
| CW32L012 | `cw32l012-bsp` | 4MHz | 用户板（LED PC13、UART1 PA02/PA03 AF1——实机验证） |
| CW32L031 | `cw32l031-bsp` | 8MHz | — |
| CW32L052 | `cw32l052-bsp` | 8MHz | — |
| CW32L083 | `cw32l083-bsp` | 8MHz | — |
| CW32R031 | `cw32r031-bsp` | 8MHz | — |
| CW32W031 | `cw32w031-bsp` | 8MHz | — |

"实板支持"列的引脚组合已在真机验证；其余家族为 generic 模板（引脚数据来自数据手册，语义级验证待相应板卡）。

## 用法

```toml
[dependencies]
cw32f030-bsp = { git = "https://github.com/SunWithCloude/cw32-bsp" }
```

```rust
use cw32f030_bsp::{bsp_pin, pins};

let tx = bsp_pin!(A, p, 8);           // PA8
let rx = bsp_pin!(A, p, 9);           // PA9
// UART1 其它组合见 pins::UART1_TXD / pins::UART1_RXD
```

## 生态位置

```
cw32-pacs（PAC，11 家族） → embassy-cw32（异步 HAL，11 家族） → cw32-bsp（本仓：板级模板）
                                                    ↘ cw32l012-hal（阻塞 HAL，L012）
cw32-tools（VS Code 插件：构建/烧录/调试/Agent API）
probe-rs（#4549 F030 烧录 + #4598 全系列自动识别，均已上游合并）
```

## 许可

MIT OR Apache-2.0。引脚数据来自官方数据手册（出处逐条内联）。
