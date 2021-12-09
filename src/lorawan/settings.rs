use crate::lorawan::RECEIVE_DELAY;
use core::marker::PhantomData;
use core::time::Duration;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Settings<R> {
    rx_delay: Duration,
    rx1_dr_offset: usize,
    rx2_dr: usize,
    _region: PhantomData<R>,
}

impl<R> Settings<R> {
    pub fn new(rx_delay: u8, rx1_dr_offset: u8, rx2_dr: u8) -> Self {
        let rx_delay = Duration::from_secs(match rx_delay & 0x0F {
            0 => 1,
            n => n as u64,
        });

        Settings {
            rx_delay,
            rx1_dr_offset: rx1_dr_offset as usize,
            rx2_dr: rx2_dr as usize,
            _region: PhantomData,
        }
    }

    pub fn rx_delay(&self) -> Duration {
        self.rx_delay
    }

    pub fn rx1_dr_offset(&self) -> usize {
        self.rx1_dr_offset
    }

    pub fn rx2_dr(&self) -> usize {
        self.rx2_dr
    }
}

impl<R> Default for Settings<R> {
    fn default() -> Self {
        Settings {
            rx_delay: RECEIVE_DELAY,
            rx1_dr_offset: 0,
            rx2_dr: 0,
            _region: PhantomData,
        }
    }
}
