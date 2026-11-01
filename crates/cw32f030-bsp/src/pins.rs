//! 引脚复用路由表(自动生成,勿手改)。
//!
//! 数据源:fan2nd/cw32-hal2 `cw32-data/af/cw32f030.yaml` —— 官方数据手册
//! 《?》AF 表的 PDF 定位提取,与厂商 SDK GPIO 宏(表内
//! `source_macro`)交叉核对。生成脚本:cw32-bsp/gen_pins.py。
//!
//! ⚠ 可落性:embassy 的 Peripherals 端口字段由 PAC 决定——本家族 PAC
//! 收编端口:PA, PB, PC, PF。标注 `[PAC]` 的组合其端口未被 PAC
//! 收编为 Peripherals 字段,当前无法构造(等 PAC 生成器补端口),
//! 其余组合可直接经 [`bsp_pin!`] 落地。

/// `I2C1` SCL 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C1_SCL: &[(&str, u8, u8)] = &[
    ("A", 9, 3), // PA09_AFx_I2C1SCL
    ("A", 14, 2), // PA14_AFx_I2C1SCL
    ("B", 6, 3), // PB06_AFx_I2C1SCL
    ("B", 8, 1), // PB08_AFx_I2C1SCL
    ("B", 10, 3), // PB10_AFx_I2C1SCL
    ("F", 1, 3), // PF01_AFx_I2C1SCL
    ("F", 6, 2), // PF06_AFx_I2C1SCL
];

/// `I2C1` SDA 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C1_SDA: &[(&str, u8, u8)] = &[
    ("A", 10, 3), // PA10_AFx_I2C1SDA
    ("A", 13, 2), // PA13_AFx_I2C1SDA
    ("B", 7, 3), // PB07_AFx_I2C1SDA
    ("B", 9, 1), // PB09_AFx_I2C1SDA
    ("B", 11, 3), // PB11_AFx_I2C1SDA
    ("F", 0, 3), // PF00_AFx_I2C1SDA
    ("F", 7, 2), // PF07_AFx_I2C1SDA
];

/// `SPI1` MISO 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI1_MISO: &[(&str, u8, u8)] = &[
    ("A", 6, 5), // PA06_AFx_SPI1MISO
    ("A", 11, 5), // PA11_AFx_SPI1MISO
    ("B", 4, 5), // PB04_AFx_SPI1MISO
    ("B", 14, 5), // PB14_AFx_SPI1MISO
];

/// `SPI1` MOSI 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI1_MOSI: &[(&str, u8, u8)] = &[
    ("A", 7, 5), // PA07_AFx_SPI1MOSI
    ("A", 12, 5), // PA12_AFx_SPI1MOSI
    ("B", 5, 5), // PB05_AFx_SPI1MOSI
    ("B", 15, 5), // PB15_AFx_SPI1MOSI
];

/// `SPI1` SCK 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI1_SCK: &[(&str, u8, u8)] = &[
    ("A", 5, 5), // PA05_AFx_SPI1SCK
    ("A", 10, 5), // PA10_AFx_SPI1SCK
    ("B", 3, 5), // PB03_AFx_SPI1SCK
    ("B", 13, 5), // PB13_AFx_SPI1SCK
];

/// `UART1` RXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_RXD: &[(&str, u8, u8)] = &[
    ("A", 9, 2), // PA09_AFx_UART1RXD
    ("A", 13, 3), // PA13_AFx_UART1RXD
    ("A", 15, 3), // PA15_AFx_UART1RXD
    ("B", 9, 3), // PB09_AFx_UART1RXD
    ("B", 10, 2), // PB10_AFx_UART1RXD
];

/// `UART1` TXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_TXD: &[(&str, u8, u8)] = &[
    ("A", 8, 2), // PA08_AFx_UART1TXD
    ("A", 14, 3), // PA14_AFx_UART1TXD
    ("B", 2, 2), // PB02_AFx_UART1TXD
    ("B", 8, 3), // PB08_AFx_UART1TXD
];
