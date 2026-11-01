//! 引脚复用路由表(自动生成,勿手改)。
//!
//! 数据源:fan2nd/cw32-hal2 `cw32-data/af/cw32l010.yaml` —— 官方数据手册
//! 《?》AF 表的 PDF 定位提取,与厂商 SDK GPIO 宏(表内
//! `source_macro`)交叉核对。生成脚本:cw32-bsp/gen_pins.py。
//!
//! ⚠ 可落性:embassy 的 Peripherals 端口字段由 PAC 决定——本家族 PAC
//! 收编端口:PA, PB。标注 `[PAC]` 的组合其端口未被 PAC
//! 收编为 Peripherals 字段,当前无法构造(等 PAC 生成器补端口),
//! 其余组合可直接经 [`bsp_pin!`] 落地。

/// `I2C1` SCL 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C1_SCL: &[(&str, u8, u8)] = &[
    ("A", 1, 2), // PA01_AFx_I2C1SCL
    ("A", 6, 4), // PA06_AFx_I2C1SCL
    ("A", 8, 2), // PA08_AFx_I2C1SCL
    ("B", 1, 4), // PB01_AFx_I2C1SCL
    ("B", 6, 4), // PB06_AFx_I2C1SCL
];

/// `I2C1` SDA 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C1_SDA: &[(&str, u8, u8)] = &[
    ("A", 0, 2), // PA00_AFx_I2C1SDA
    ("A", 5, 4), // PA05_AFx_I2C1SDA
    ("A", 7, 2), // PA07_AFx_I2C1SDA
    ("B", 0, 4), // PB00_AFx_I2C1SDA
    ("B", 5, 4), // PB05_AFx_I2C1SDA
];

/// `UART1` RXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_RXD: &[(&str, u8, u8)] = &[
    ("A", 0, 1), // PA00_AFx_UART1RXD
    ("A", 5, 1), // PA05_AFx_UART1RXD
    ("A", 7, 1), // PA07_AFx_UART1RXD
    ("B", 0, 1), // PB00_AFx_UART1RXD
];

/// `UART1` TXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_TXD: &[(&str, u8, u8)] = &[
    ("A", 1, 1), // PA01_AFx_UART1TXD
    ("A", 6, 1), // PA06_AFx_UART1TXD
    ("A", 8, 1), // PA08_AFx_UART1TXD
    ("B", 1, 1), // PB01_AFx_UART1TXD
];
