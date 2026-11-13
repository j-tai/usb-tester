use core::ops::BitOr;

use embassy_stm32::gpio::{Flex, Pull, Speed};
use embassy_time::Timer;

pub struct Port<'d> {
    pub power: Flex<'d>,
    pub data_plus: Flex<'d>,
    pub data_minus: Flex<'d>,
}

impl<'d> Port<'d> {
    pub fn pins(&mut self) -> [&mut Flex<'d>; 3] {
        [&mut self.power, &mut self.data_plus, &mut self.data_minus]
    }
}

pub async fn probe(port1: &mut Port<'_>, port2: &mut Port<'_>) -> ProbeResult {
    async fn probe_bidirectional(pin1: &mut Flex<'_>, pin2: &mut Flex<'_>) -> bool {
        probe_unidirectional(pin1, pin2).await && probe_unidirectional(pin2, pin1).await
    }

    async fn probe_unidirectional(pin1: &mut Flex<'_>, pin2: &mut Flex<'_>) -> bool {
        pin1.set_as_output(Speed::Low);
        pin1.set_low();
        Timer::after_millis(1).await;
        let result = pin2.is_low();
        pin1.set_as_input(Pull::Up);
        result
    }

    let power = probe_bidirectional(&mut port1.power, &mut port2.power).await;
    let data = probe_bidirectional(&mut port1.data_plus, &mut port2.data_plus).await
        && probe_bidirectional(&mut port1.data_minus, &mut port2.data_minus).await;

    ProbeResult { power, data }
}

pub struct ProbeResult {
    pub power: bool,
    pub data: bool,
}

impl BitOr for ProbeResult {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ProbeResult {
            power: self.power || rhs.power,
            data: self.data || rhs.data,
        }
    }
}
