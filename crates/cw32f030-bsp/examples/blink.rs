//! CW32F030 最小点灯(通用模板):LED 引脚按板修改 `LED_PORT`/`LED_PIN`。
//!
//! cargo build --target thumbv6m-none-eabi --release --features rt --example blink

#![no_std]
#![no_main]

use embassy_executor::main;
use embassy_time::Timer;

use cw32f030_bsp::bsp_pin;


#[main]
async fn main(_spawner: embassy_executor::Spawner) {
    embassy_cw32::init(Default::default());
    let p = embassy_cw32::Peripherals::take().unwrap();
    // LED 引脚按板改(立创天猛星 F030 板 = PC13;宏参数是编译期字面端口字母)
    let mut led = Output::new(bsp_pin!(C, p, 13), Level::High);
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
