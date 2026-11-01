//! 引脚复用路由表(自动生成,勿手改)。
//!
//! 数据源:fan2nd/cw32-hal2 `cw32-data/af/cw32l012.yaml` —— 官方数据手册
//! 《?》AF 表的 PDF 定位提取,与厂商 SDK GPIO 宏(表内
//! `source_macro`)交叉核对。生成脚本:cw32-bsp/gen_pins.py。
//!
//! ⚠ 可落性:embassy 的 Peripherals 端口字段由 PAC 决定——本家族 PAC
//! 收编端口:PA, PB, PC, PF。标注 `[PAC]` 的组合其端口未被 PAC
//! 收编为 Peripherals 字段,当前无法构造(等 PAC 生成器补端口),
//! 其余组合可直接经 [`bsp_pin!`] 落地。

/// `I2C1` SCL 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C1_SCL: &[(&str, u8, u8)] = &[
    ("A", 4, 2), // PA04_AFx_I2C1SCL
    ("A", 9, 4), // PA09_AFx_I2C1SCL
    ("A", 14, 4), // PA14_AFx_I2C1SCL
    ("B", 6, 3), // PB06_AFx_I2C1SCL
    ("B", 8, 3), // PB08_AFx_I2C1SCL
    ("B", 10, 3), // PB10_AFx_I2C1SCL
    ("C", 15, 4), // PC15_AFx_I2C1SCL
    ("F", 1, 4), // PF01_AFx_I2C1SCL
    ("F", 6, 4), // PF06_AFx_I2C1SCL
];

/// `I2C1` SDA 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C1_SDA: &[(&str, u8, u8)] = &[
    ("A", 5, 2), // PA05_AFx_I2C1SDA
    ("A", 10, 4), // PA10_AFx_I2C1SDA
    ("A", 13, 4), // PA13_AFx_I2C1SDA
    ("B", 7, 3), // PB07_AFx_I2C1SDA
    ("B", 9, 3), // PB09_AFx_I2C1SDA
    ("B", 11, 3), // PB11_AFx_I2C1SDA
    ("C", 14, 4), // PC14_AFx_I2C1SDA
    ("F", 0, 4), // PF00_AFx_I2C1SDA
    ("F", 7, 4), // PF07_AFx_I2C1SDA
];

/// `SPI1` MISO 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI1_MISO: &[(&str, u8, u8)] = &[
    ("A", 6, 3), // PA06_AFx_SPI1MISO
    ("A", 11, 3), // PA11_AFx_SPI1MISO
    ("B", 4, 3), // PB04_AFx_SPI1MISO
    ("B", 14, 3), // PB14_AFx_SPI1MISO
];

/// `SPI1` MOSI 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI1_MOSI: &[(&str, u8, u8)] = &[
    ("A", 7, 3), // PA07_AFx_SPI1MOSI
    ("A", 12, 3), // PA12_AFx_SPI1MOSI
    ("B", 5, 3), // PB05_AFx_SPI1MOSI
    ("B", 15, 3), // PB15_AFx_SPI1MOSI
];

/// `SPI1` SCK 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI1_SCK: &[(&str, u8, u8)] = &[
    ("A", 5, 3), // PA05_AFx_SPI1SCK
    ("A", 10, 3), // PA10_AFx_SPI1SCK
    ("B", 3, 3), // PB03_AFx_SPI1SCK
    ("B", 13, 3), // PB13_AFx_SPI1SCK
];

/// `UART1` RXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_RXD: &[(&str, u8, u8)] = &[
    ("A", 3, 1), // PA03_AFx_UART1RXD
    ("A", 10, 1), // PA10_AFx_UART1RXD
    ("A", 13, 1), // PA13_AFx_UART1RXD
    ("A", 15, 1), // PA15_AFx_UART1RXD
    ("B", 0, 1), // PB00_AFx_UART1RXD
    ("B", 7, 1), // PB07_AFx_UART1RXD
    ("B", 11, 1), // PB11_AFx_UART1RXD
    ("C", 14, 1), // PC14_AFx_UART1RXD
    ("F", 0, 1), // PF00_AFx_UART1RXD
];

/// `UART1` TXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_TXD: &[(&str, u8, u8)] = &[
    ("A", 2, 1), // PA02_AFx_UART1TXD
    ("A", 9, 1), // PA09_AFx_UART1TXD
    ("A", 14, 1), // PA14_AFx_UART1TXD
    ("B", 1, 1), // PB01_AFx_UART1TXD
    ("B", 3, 1), // PB03_AFx_UART1TXD
    ("B", 6, 1), // PB06_AFx_UART1TXD
    ("B", 12, 1), // PB12_AFx_UART1TXD
    ("C", 15, 1), // PC15_AFx_UART1TXD
    ("F", 1, 1), // PF01_AFx_UART1TXD
];
