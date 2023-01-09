///! ### LoRaWAN MAC configuration
pub mod responses;
pub mod types;

use atat::atat_derive::AtatCmd;
use responses::*;
use crate::NoResponse;
use types::{
    ConfirmedUplink as ConfirmedUplinkVal,
    DataRate as DataRateVal,
    UploadReportMode as UploadReportModeVal,
    TransmitPower as TransmitPowerVal
};


/// 4.2.22 Get whether uplink messages are confirmed
#[derive(Clone, AtatCmd)]
#[at_cmd("+CCONFIRM", ConfirmedUplink)]
pub struct ConfirmedUplinkGet;

/// 4.2.22 Set whether uplink messages are confirmed
#[derive(Clone, AtatCmd)]
#[at_cmd("+CCONFIRM", NoResponse)]
pub struct ConfirmedUplinkSet {
    #[at_arg(position = 0)]
    pub confirmed: ConfirmedUplinkVal
}


/// 4.2.23 Get the application port
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPPORT", ApplicationPort)]
pub struct ApplicationPortGet;

/// 4.2.23 Set the application port. Default 10. Range 1..223
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPPORT", NoResponse)]
pub struct ApplicationPortSet {
    #[at_arg(position = 0)]
    pub port: u8
}


/// 4.2.24 Get the data rate
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDATARATE", DataRate)]
pub struct DataRateGet;

/// 4.2.24 Set the data rate. Default 3 - SF9 BW125
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDATARATE", NoResponse)]
pub struct DataRateSet {
    #[at_arg(position = 0)]
    pub rate: DataRateVal
}

// 4.2.25 Get RSSI - ignored because only supports CN470 (China 470MHz)

/// 4.2.26 Get max send times
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDATARATE", MaxSendTimes)]
pub struct MaxSendTimesGet;

/// 4.2.26 Set max send times
/// Must be set before sending data
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDATARATE", NoResponse)]
pub struct MaxSendTimesSet {
    #[at_arg(position = 0)]
    pub confirmed: ConfirmedUplinkVal,
    /// Value between 1 and 15
    #[at_arg(position = 1)]
    pub times: u8
}


/// 4.2.27 Get the upload reporting mode
/// Mainly used for testing purposes
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRM", UploadReportMode)]
pub struct UploadReportModeGet;

/// 4.2.27 Set the upload reporting mode
/// Mainly used for testing purposes
#[derive(Clone, AtatCmd)]
#[at_cmd("+CRM", NoResponse)]
pub struct UploadReportModeSet {
    #[at_arg(position = 0)]
    pub mode: UploadReportModeVal
}


/// 4.2.28 Get the transmit power
/// Default 17dBm. Needs to be set before sending data
#[derive(Clone, AtatCmd)]
#[at_cmd("+CTXP", TransmitPower)]
pub struct TransmitPowerGet;

/// 4.2.28 Set the upload reporting mode
/// Default 0 = 17dBm. Needs to be set before sending data
#[derive(Clone, AtatCmd)]
#[at_cmd("+CTXP", NoResponse)]
pub struct TransmitPowerSet {
    #[at_arg(position = 0)]
    pub power: TransmitPowerVal
}

// AT+CLINKCHECK Enable Link check Mandatory
// AT+CADR Enable/Disable ADR Function Mandatory
// AT+CRXP Set/Read Receive Window Parameter Mandatory
// AT+CRX1DELAY Set/Read TX and RX1 Delay Mandatory
// AT+CSAVE Save configuration Mandatory
// AT+CRESTORE Restore to Default Configuration Mandatory