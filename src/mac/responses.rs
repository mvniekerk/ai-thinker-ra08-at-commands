use atat::atat_derive::AtatResp;

use super::types::{
    ConfirmedUplink as ConfirmedUplinkVal,
    DataRate as DataRateVal,
    UploadReportMode as UploadReportModeVal,
    TransmitPower as TransmitPowerVal
};

/// Get whether uplink messages should be confirmed
#[derive(Clone, Debug, AtatResp)]
pub struct ConfirmedUplink {
    #[at_arg(position = 0)]
    pub confirmed: ConfirmedUplinkVal,
}

/// Application port
#[derive(Clone, Debug, AtatResp)]
pub struct ApplicationPort {
    #[at_arg(position = 0)]
    pub port: u8,
}

/// Max send times
#[derive(Clone, Debug, AtatResp)]
pub struct MaxSendTimes {
    #[at_arg(position = 0)]
    pub confirmed: ConfirmedUplinkVal,
    /// Value between 1 and 15
    #[at_arg(position = 1)]
    pub times: u8
}

/// Data rate
#[derive(Clone, Debug, AtatResp)]
pub struct DataRate {
    #[at_arg(position = 0)]
    pub rate: DataRateVal,
}

/// Upload report mode
#[derive(Clone, Debug, AtatResp)]
pub struct UploadReportMode {
    #[at_arg(position = 0)]
    pub mode: UploadReportModeVal,
}

/// Transmit power
#[derive(Clone, Debug, AtatResp)]
pub struct TransmitPower {
    #[at_arg(position = 0)]
    pub power: TransmitPowerVal,
}
