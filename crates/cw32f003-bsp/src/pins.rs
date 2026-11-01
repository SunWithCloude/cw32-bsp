//! 引脚复用路由表(自动生成,勿手改)。
//!
//! 数据源:fan2nd/cw32-hal2 `cw32-data/af/cw32f003.yaml` —— 官方数据手册
//! 《?》AF 表的 PDF 定位提取,与厂商 SDK GPIO 宏(表内
//! `source_macro`)交叉核对。生成脚本:cw32-bsp/gen_pins.py。
//!
//! ⚠ 可落性:embassy 的 Peripherals 端口字段由 PAC 决定——本家族 PAC
//! 收编端口:PA, PC。标注 `[PAC]` 的组合其端口未被 PAC
//! 收编为 Peripherals 字段,当前无法构造(等 PAC 生成器补端口),
//! 其余组合可直接经 [`bsp_pin!`] 落地。

/// `I2C` SCL 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C_SCL: &[(&str, u8, u8)] = &[
    ("A", 5, 3), // PA05_AFx_I2CSCL
    ("B", 1, 3), // PB01_AFx_I2CSCL [PAC]
    ("B", 4, 2), // PB04_AFx_I2CSCL [PAC]
    ("B", 6, 2), // PB06_AFx_I2CSCL [PAC]
];

/// `I2C` SDA 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const I2C_SDA: &[(&str, u8, u8)] = &[
    ("A", 2, 3), // PA02_AFx_I2CSDA
    ("A", 6, 3), // PA06_AFx_I2CSDA
    ("B", 0, 2), // PB00_AFx_I2CSDA [PAC]
    ("B", 3, 2), // PB03_AFx_I2CSDA [PAC]
    ("B", 5, 2), // PB05_AFx_I2CSDA [PAC]
];

/// `SPI` MISO 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI_MISO: &[(&str, u8, u8)] = &[
    ("A", 4, 3), // PA04_AFx_SPIMISO
    ("C", 1, 3), // PC01_AFx_SPIMISO
    ("C", 3, 3), // PC03_AFx_SPIMISO
];

/// `SPI` MOSI 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI_MOSI: &[(&str, u8, u8)] = &[
    ("A", 1, 3), // PA01_AFx_SPIMOSI
    ("C", 2, 3), // PC02_AFx_SPIMOSI
    ("C", 4, 3), // PC04_AFx_SPIMOSI
];

/// `SPI` SCK 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const SPI_SCK: &[(&str, u8, u8)] = &[
    ("A", 0, 3), // PA00_AFx_SPISCK
    ("B", 7, 3), // PB07_AFx_SPISCK [PAC]
    ("C", 0, 3), // PC00_AFx_SPISCK
];

/// `UART1` RXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_RXD: &[(&str, u8, u8)] = &[
    ("A", 0, 1), // PA00_AFx_UART1RXD
    ("A", 2, 1), // PA02_AFx_UART1RXD
    ("A", 3, 2), // PA03_AFx_UART1RXD
    ("A", 4, 1), // PA04_AFx_UART1RXD
    ("B", 0, 1), // PB00_AFx_UART1RXD [PAC]
    ("B", 5, 1), // PB05_AFx_UART1RXD [PAC]
    ("C", 4, 1), // PC04_AFx_UART1RXD
];

/// `UART1` TXD 的全部可用引脚组合:`(端口字母, 引脚号, AF 编号)`。
pub const UART1_TXD: &[(&str, u8, u8)] = &[
    ("A", 5, 1), // PA05_AFx_UART1TXD
    ("B", 1, 1), // PB01_AFx_UART1TXD [PAC]
    ("B", 2, 1), // PB02_AFx_UART1TXD [PAC]
    ("B", 6, 1), // PB06_AFx_UART1TXD [PAC]
    ("B", 7, 2), // PB07_AFx_UART1TXD [PAC]
    ("C", 0, 2), // PC00_AFx_UART1TXD
    ("C", 3, 1), // PC03_AFx_UART1TXD
];
