#![no_std]
#![no_main]

mod probe;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Flex, Level, Output, Pull, Speed};
use embassy_time::Timer;
use panic_probe as _;

use crate::probe::{Port, ProbeResult, probe};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peri = embassy_stm32::init(Default::default());
    // Do not use these pins! They are reserved for debugging!
    let _swdio = peri.PA13;
    let _swclk = peri.PA14;

    info!("Hello, world!");

    let mut power_indicator = Output::new(peri.PB2, Level::Low, Speed::Low);
    let mut data_indicator = Output::new(peri.PB1, Level::Low, Speed::Low);

    let mut common = Port {
        power: Flex::new(peri.PA3),
        data_plus: Flex::new(peri.PA5),
        data_minus: Flex::new(peri.PA4),
    };

    let mut port_a = Port {
        power: Flex::new(peri.PA8),
        data_plus: Flex::new(peri.PA10),
        data_minus: Flex::new(peri.PA9),
    };

    let mut port_b = Port {
        power: Flex::new(peri.PB13),
        data_plus: Flex::new(peri.PB15),
        data_minus: Flex::new(peri.PB14),
    };

    let mut port_c = Port {
        power: Flex::new(peri.PA6),
        data_plus: Flex::new(peri.PB0),
        data_minus: Flex::new(peri.PA7),
    };

    for port in [&mut common, &mut port_a, &mut port_b, &mut port_c] {
        for pin in port.pins() {
            pin.set_as_input(Pull::Up);
        }
    }

    loop {
        let ProbeResult { power, data } = probe(&mut common, &mut port_a).await
            | probe(&mut common, &mut port_b).await
            | probe(&mut common, &mut port_c).await
            | probe(&mut port_a, &mut port_b).await
            | probe(&mut port_a, &mut port_c).await
            | probe(&mut port_b, &mut port_c).await;

        power_indicator.set_level(power.into());
        data_indicator.set_level(data.into());
        Timer::after_millis(200).await;
    }
}
