///! ### LoRaWAN MAC configuration
pub mod responses;
pub mod types;

use crate::mac::types::LinkCheckMode;
use crate::NoResponse;
use atat::atat_derive::AtatCmd;
use responses::*;
use types::{
    AdrEnabled as AdrEnabledVal, ConfirmedUplink as ConfirmedUplinkVal, DataRate as DataRateVal,
    TransmitPower as TransmitPowerVal, UploadReportMode as UploadReportModeVal,
};

/// 4.2.22 Get whether uplink messages are confirmed
#[derive(Clone, AtatCmd)]
#[at_cmd("+CCONFIRM?", ConfirmedUplink)]
pub struct ConfirmedUplinkGet;

/// 4.2.22 Set whether uplink messages are confirmed
#[derive(Clone, AtatCmd)]
#[at_cmd("+CCONFIRM", NoResponse)]
pub struct ConfirmedUplinkSet {
    #[at_arg(position = 0)]
    pub confirmed: ConfirmedUplinkVal,
}

/// 4.2.23 Get the application port
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPPORT?", ApplicationPort)]
pub struct ApplicationPortGet;

/// 4.2.23 Set the application port. Default 10. Range 1..223
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPPORT", NoResponse)]
pub struct ApplicationPortSet {
    #[at_arg(position = 0)]
    pub port: u8,
}

/// 4.2.24 Get the data rate
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDATARATE?", DataRate)]
pub struct DataRateGet;

/// 4.2.24 Set the data rate. Default 3 - SF9 BW125
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDATARATE", NoResponse)]
pub struct DataRateSet {
    #[at_arg(position = 0)]
    pub rate: DataRateVal,
}

// 4.2.25 Get RSSI - ignored because only supports CN470 (China 470MHz)

/// 4.2.26 Get max send times
#[derive(Clone, AtatCmd)]
#[at_cmd("+CNBTRIALS?", MaxSendTimes)]
pub struct MaxSendTimesGet;

/// 4.2.26 Set max send times
/// Must be set before sending data
#[derive(Clone, AtatCmd)]
#[at_cmd("+CNBTRIALS", NoResponse)]
pub struct MaxSendTimesSet {
    #[at_arg(position = 0)]
    pub confirmed: ConfirmedUplinkVal,
    /// Value between 1 and 15
    #[at_arg(position = 1)]
    pub times: u8,
}

/// 4.2.27 Get the upload reporting mode
/// Mainly used for testing purposes
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRM?", UploadReportMode)]
pub struct UploadReportModeGet;

/// 4.2.27 Set the upload reporting mode
/// Mainly used for testing purposes
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRM", NoResponse)]
pub struct UploadReportModeSet {
    #[at_arg(position = 0)]
    pub mode: UploadReportModeVal,
}

/// 4.2.28 Get the transmit power
/// Default 17dBm. Needs to be set before sending data
#[derive(Clone, AtatCmd)]
#[at_cmd("+CTXP?", TransmitPower)]
pub struct TransmitPowerGet;

/// 4.2.28 Set the upload reporting mode
/// Default 0 = 17dBm. Needs to be set before sending data
#[derive(Clone, AtatCmd)]
#[at_cmd("+CTXP", NoResponse)]
pub struct TransmitPowerSet {
    #[at_arg(position = 0)]
    pub power: TransmitPowerVal,
}

/// 4.2.29 Verify network connection
/// TODO Non-0 response denotes error response. Need to be better with the parsing to an enum
#[derive(Clone, AtatCmd)]
#[at_cmd("+CLINKCHECK", VerifyNetworkConnectionResponse)]
pub struct VerifyNetworkConnection {
    #[at_arg(position = 0)]
    pub mode: LinkCheckMode,
}

/// 4.2.30 Get ADR enable status
/// Needs to be set up before sending data. ADR is enabled by default.
#[derive(Clone, AtatCmd)]
#[at_cmd("+CADR?", VerifyNetworkConnectionResponse)]
pub struct AdrEnabledGet;

/// 4.2.30 Set ADR enable status
/// Needs to be set up before sending data. ADR is enabled by default.
#[derive(Clone, AtatCmd)]
#[at_cmd("+CADR", NoResponse)]
pub struct AdrEnabledSet {
    #[at_arg(position = 0)]
    pub enabled: AdrEnabledVal,
}

/// 4.2.31 Get receive window parameters
/// Needs to be set up before sending data. Doesn't affect default values
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRXP?", VerifyNetworkConnectionResponse)]
pub struct ReceiveWindowParametersGet;

/// 4.2.31 Set receive window parameters
/// Needs to be set up before sending data. Doesn't affect default values
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRXP", NoResponse)]
pub struct ReceiveWindowParametersSet {
    #[at_arg(position = 0)]
    pub rx1_datarate_offset: u8,
    #[at_arg(position = 1)]
    pub rx2_datarate: u8,
    #[at_arg(position = 1)]
    pub rx2_frequency: u32,
}

/// 4.2.32 Get Rx1 Delay setup. How long after sending to open the RX1 window, unit in seconds.
/// Needs to be set up before sending data.
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRX1DELAY?", Rx1Delay)]
pub struct Rx1DelayGet;

/// 4.2.32 Set Rx1 Delay setup. How long after sending to open the RX1 window, unit in seconds.
/// Needs to be set up before sending data.
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRX1DELAY", NoResponse)]
pub struct Rx1DelaySet {
    #[at_arg(position = 0)]
    pub delay_in_seconds: u16,
}

/// 4.2.33 Save MAC configuration parameters
/// Saves to EEPROM/Flash. After rebooting the module will use the saved values to initialize
/// the network. Needs to be saved before sending data
#[derive(Clone, AtatCmd)]
#[at_cmd("+CSAVE", NoResponse)]
pub struct SaveMacConfigurationParameters;

/// 4.2.34 Restore MAC configuration parameters to factory settings
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRESTORE", NoResponse)]
pub struct RestoreMacConfigurationParametersFromFactorySettings;
