//! CW32L052 最小点灯(通用模板):LED 引脚按板修改 `LED_PORT`/`LED_PIN`。
//!
//! cargo build --target thumbv6m-none-eabi --release --features rt --example blink

#![no_std]
#![no_main]

use embassy_executor::main;
use embassy_time::Timer;

use cw32l052_bsp::bsp_pin;


#[main]
async fn main(_spawner: embassy_executor::Spawner) {
    embassy_cw32::init(Default::default());
    let p = embassy_cw32::Peripherals::take().unwrap();
    // LED 引脚按板改。⚠ 本家族 PAC 的 Peripherals 仅收编 GPIOA——其它
    // 端口(如常见板卡的 PC13)等 PAC 补端口字段后,经 pins 表组合落地
    let mut led = Output::new(bsp_pin!(A, p, 0), Level::High);
    loop {
        Timer::after_millis(500).await;
        led.toggle();
    }
}

use embassy_cw32::gpio::{Level, Output};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
