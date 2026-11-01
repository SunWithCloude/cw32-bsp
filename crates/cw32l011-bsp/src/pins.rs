//! 引脚复用路由表(自动生成,勿手改)。
//!
//! 数据源:fan2nd/cw32-hal2 `cw32-data/af/cw32l011.yaml` —— 官方数据手册
//! 《?》AF 表的 PDF 定位提取,与厂商 SDK GPIO 宏(表内
//! `source_macro`)交叉核对。生成脚本:cw32-bsp/gen_pins.py。
//!
//! ⚠ 可落性:embassy 的 Peripherals 端口字段由 PAC 决定——本家族 PAC
//! 收编端口:PA, PB, PC。标注 `[PAC]` 的组合其端口未被 PAC
//! 收编为 Peripherals 字段,当前无法构造(等 PAC 生成器补端口),
//! 其余组合可直接经 [`bsp_pin!`] 落地。

/// `I2C` SCL 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C_SCL: &[(&str, u8, u8)] = &[
    ("A", 4, 2), // PA04_AFx_I2CSCL
    ("A", 9, 4), // PA09_AFx_I2CSCL
    ("A", 14, 4), // PA14_AFx_I2CSCL
    ("B", 6, 3), // PB06_AFx_I2CSCL
    ("C", 15, 4), // PC15_AFx_I2CSCL
];

/// `I2C` SDA 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C_SDA: &[(&str, u8, u8)] = &[
    ("A", 5, 2), // PA05_AFx_I2CSDA
    ("A", 10, 4), // PA10_AFx_I2CSDA
    ("A", 13, 4), // PA13_AFx_I2CSDA
    ("B", 7, 3), // PB07_AFx_I2CSDA
    ("C", 14, 4), // PC14_AFx_I2CSDA
];

/// `UART1` RXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_RXD: &[(&str, u8, u8)] = &[
    ("A", 3, 1), // PA03_AFx_UART1RXD
    ("A", 10, 1), // PA10_AFx_UART1RXD
    ("A", 13, 1), // PA13_AFx_UART1RXD
    ("A", 15, 1), // PA15_AFx_UART1RXD
    ("B", 0, 1), // PB00_AFx_UART1RXD
    ("B", 7, 1), // PB07_AFx_UART1RXD
    ("C", 14, 1), // PC14_AFx_UART1RXD
];

/// `UART1` TXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_TXD: &[(&str, u8, u8)] = &[
    ("A", 2, 1), // PA02_AFx_UART1TXD
    ("A", 9, 1), // PA09_AFx_UART1TXD
    ("A", 14, 1), // PA14_AFx_UART1TXD
    ("B", 1, 1), // PB01_AFx_UART1TXD
    ("B", 3, 1), // PB03_AFx_UART1TXD
    ("B", 6, 1), // PB06_AFx_UART1TXD
    ("C", 15, 1), // PC15_AFx_UART1TXD
];
