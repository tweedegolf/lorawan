use crate::lorawan::{AppEui, AppKey, AppSKey, DevAddr, DevEui, NwkSKey, Settings};

/// Credentials needed to join a device to a network. A device that has not joined a network will
/// use this as state.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Credentials {
    app_eui: AppEui,
    dev_eui: DevEui,
    app_key: AppKey,
}

impl Credentials {
    pub fn new(app_eui: AppEui, dev_eui: DevEui, app_key: AppKey) -> Self {
        Credentials {
            app_eui,
            dev_eui,
            app_key,
        }
    }

    pub fn app_eui(&self) -> &AppEui {
        &self.app_eui
    }

    pub fn dev_eui(&self) -> &DevEui {
        &self.dev_eui
    }

    pub fn app_key(&self) -> &AppKey {
        &self.app_key
    }
}

/// Represents the state of a device that has joined a network.
#[derive(Debug)]
pub struct DeviceState<R> {
    session: Session,
    settings: Settings<R>,
    tx_dr: usize,
    fcnt_up: u32,
    fcnt_down: u32,
    adr_ack_cnt: u32,
}

impl<R> DeviceState<R> {
    pub fn new(session: Session, settings: Settings<R>) -> Self {
        DeviceState {
            session,
            settings,
            tx_dr: 0,
            fcnt_up: 0,
            fcnt_down: 0,
            adr_ack_cnt: 0,
        }
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub fn settings(&self) -> &Settings<R> {
        &self.settings
    }

    pub fn tx_dr(&self) -> usize {
        self.tx_dr
    }

    pub fn fcnt_up(&self) -> u32 {
        self.fcnt_up
    }

    pub fn fcnt_down(&self) -> u32 {
        self.fcnt_down
    }

    pub fn increment_fcnt_up(&mut self) {
        self.fcnt_up += 1;
    }

    pub fn increment_fcnt_down(&mut self) {
        self.fcnt_down += 1;
    }
}

/// Session data for a device joined to a network.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Session {
    dev_addr: DevAddr,
    nwk_skey: NwkSKey,
    app_skey: AppSKey,
}

impl Session {
    /// Creates a session directly for ABP.
    pub fn new(dev_addr: DevAddr, nwk_skey: NwkSKey, app_skey: AppSKey) -> Self {
        Session {
            dev_addr,
            nwk_skey,
            app_skey,
        }
    }

    pub fn dev_addr(&self) -> &DevAddr {
        &self.dev_addr
    }

    pub fn nwk_skey(&self) -> &NwkSKey {
        &self.nwk_skey
    }

    pub fn app_skey(&self) -> &AppSKey {
        &self.app_skey
    }
}
