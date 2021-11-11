use core::time::Duration;

pub const RECEIVE_DELAY1: Duration = Duration::from_secs(1);
pub const RECEIVE_DELAY2: Duration = Duration::from_secs(2);

pub const JOIN_ACCEPT_DELAY1: Duration = Duration::from_secs(5);
pub const JOIN_ACCEPT_DELAY2: Duration = Duration::from_secs(6);

pub const ADR_ACK_LIMIT: usize = 64;
pub const ADR_ACK_DELAY: usize = 32;
