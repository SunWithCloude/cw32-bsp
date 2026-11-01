//! CW32F030 UART1 回显(115200-8N1):pins 表中首组不同脚的 PA 组合。
//!
//! cargo build --target thumbv6m-none-eabi --release --features rt --example uart_echo

#![no_std]
#![no_main]

use embassy_executor::main;

use cw32f030_bsp::bsp_pin};

embassy_cw32::bind_interrupts!(struct Irqs {
    UART1 => embassy_cw32::uart::Uart1Irq;
});

#[main]
async fn main(_spawner: embassy_executor::Spawner) {
    embassy_cw32::init(Default::default());
    let p = embassy_cw32::Peripherals::take().unwrap();

    // UART1 全部引脚组合见 pins::UART1_TXD / UART1_RXD(含 SDK 宏出处)
    let tx = bsp_pin!(A, p, 8);
    let rx = bsp_pin!(A, p, 9);
    let mut u = embassy_cw32::uart::Uart::new_uart1(
        p.uart1, tx, rx, 2, Irqs, Default::default());

    loop {
        if let Ok(b) = u.read_byte().await {
            u.write_byte(b).await;
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
