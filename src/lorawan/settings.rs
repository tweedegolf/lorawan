use crate::lorawan::RECEIVE_DELAY1;
use core::time::Duration;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Settings {
    rx_delay: Duration,
    rx1_dr_offset: usize,
    rx2_dr: usize,
}

impl Settings {
    pub fn new(rx_delay: u8, rx1_dr_offset: u8, rx2_dr: u8) -> Self {
        let rx_delay = Duration::from_secs(match rx_delay & 0x0F {
            0 => 1,
            n => n as u64,
        });

        Settings {
            rx_delay,
            rx1_dr_offset: rx1_dr_offset as usize,
            rx2_dr: rx2_dr as usize,
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

impl Default for Settings {
    fn default() -> Self {
        Settings {
            rx_delay: RECEIVE_DELAY1,
            rx1_dr_offset: 0,
            rx2_dr: 0,
        }
    }
}
