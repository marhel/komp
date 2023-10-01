#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::UART1;
use embassy_rp::uart::{BufferedInterruptHandler, BufferedUart, Config};
use embassy_time::{Duration, Timer};
use embedded_io_async::{Write};
use static_cell::make_static;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART1_IRQ => BufferedInterruptHandler<UART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let (tx_pin, rx_pin, uart) = (p.PIN_4, p.PIN_5, p.UART1);

    let tx_buf = &mut make_static!([0u8; 16])[..];
    let rx_buf = &mut make_static!([0u8; 16])[..];
    let uart = BufferedUart::new(uart, Irqs, tx_pin, rx_pin, tx_buf, rx_buf, Config::default());
    let (_rx, mut tx) = uart.split();

    info!("Driver sending...");
    let mut turn = 0;
    const BUF_SIZE: usize = 31;
    loop {
        let mut data = [0; BUF_SIZE];
        for i in 0..BUF_SIZE {
            data[i] = (i + turn * BUF_SIZE) as u8;
        }
        info!("TX {:?}", data);
        tx.write_all(&data).await.unwrap();
        Timer::after(Duration::from_secs(1)).await;
        turn+=1;
    }
}
