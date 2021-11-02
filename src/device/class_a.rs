use std::fmt::Debug;
use std::time::Duration;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Channel, State};
use radio::blocking::{BlockingError, BlockingOptions, BlockingReceive, BlockingTransmit};

use crate::constants::{RX1_DELAY, RX2_DELAY};
use crate::lorawan::{LoRaWANChannel, LoRaWANInfo, LoRaWANState, Packet};

const INTERVAL: Duration = Duration::from_millis(100);
const TIMEOUT: Duration = Duration::from_millis(200);

const BLOCKING_OPTIONS: BlockingOptions = BlockingOptions {
    poll_interval: INTERVAL,
    timeout: TIMEOUT,
};

pub struct ClassA<R> {
    radio: R,
}

impl<R, E> ClassA<R>
    where R: BlockingTransmit<E> + BlockingReceive<LoRaWANInfo, E> + State<State=LoRaWANState, Error=E> + Channel<Channel=LoRaWANChannel, Error=E> + DelayUs<u32>,
          E: Debug
{
    /// Transmits `packet` and waits for an optional response
    pub fn uplink(&mut self, mut packet: Packet) -> Result<Option<Packet>, BlockingError<E>> {
        self.radio.do_transmit(packet.content(), BLOCKING_OPTIONS)?;
        self.radio.set_channel(&LoRaWANChannel::RX1)?;
        self.radio.delay_us(RX1_DELAY.as_micros() as u32);
        match self.radio.do_receive(packet.buf(), BLOCKING_OPTIONS) {
            Ok((n, info)) => {
                packet.set_length(n);
                Ok(Some(packet))
            }
            Err(error) => {
                match error {
                    BlockingError::Timeout => {
                        self.radio.set_channel(&LoRaWANChannel::RX2)?;
                        self.radio.delay_us((RX2_DELAY - RX1_DELAY - TIMEOUT).as_micros() as u32);
                        match self.radio.do_receive(packet.buf(), BLOCKING_OPTIONS) {
                            Ok((n, info)) => {
                                packet.set_length(n);
                                Ok(Some(packet))
                            }
                            Err(error) => {
                                match error {
                                    BlockingError::Timeout => Ok(None),
                                    error => Err(error)
                                }
                            }
                        }
                    }
                    error => Err(error),
                }
            }
        }
    }
}
