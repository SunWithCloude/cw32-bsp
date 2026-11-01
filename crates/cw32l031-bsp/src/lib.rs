//! `cw32l031-bsp` —— 武汉芯源 **CW32L031**(Cortex-M0+)的通用最小系统 BSP。
//!
//! 基于 [embassy-cw32](https://github.com/SunWithCloude/Cw32l012_embassy)
//! (feature `cw32l031`)。家族事实:复位默认 SysClk **8MHz**(数据手册
//! 原文),外设引脚复用全表见 [`pins`](pins)(含厂商 SDK 宏出处)。
//!
//! 引脚落地用 [`bsp_pin!`]:本家族 PAC 的 Peripherals 收编端口为
//! PA——其它端口的组合见 pins 表中的 `[PAC]` 标注
//! (等 PAC 生成器补端口字段)。
//!
//! ```ignore
//! use cw32l031_bsp::bsp_pin;
//! let tx = bsp_pin!(A, p, 8); // PA8
//! ```
#![no_std]

pub mod pins;

/// 按端口字母把 `(PORT, 引脚号)` 落成 embassy GPIO `Pin`
/// (展开为对应 Peripherals 端口字段)。
#[macro_export]
macro_rules! bsp_pin {
    (A, $p:ident, $n:expr) => { embassy_cw32::gpio::Pin::new($p.gpioa, $n) };
    ($other:expr) => {
        compile_error!(concat!("端口未被本家族 PAC 收编:", stringify!($other)))
    };
}
