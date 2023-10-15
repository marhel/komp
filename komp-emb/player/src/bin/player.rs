#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::UART1;

use embassy_rp::uart::{Config, InterruptHandler, UartRx};

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART1_IRQ => InterruptHandler<UART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut rx = UartRx::new(p.UART1, p.PIN_5, Irqs, p.DMA_CH1, Config::default());

    info!("Player listening...");
    loop {
        let mut buf = [0; 16];
        rx.read(&mut buf).await.unwrap();
        info!("RX {:?}", buf);
    }
}
