use atat::atat_derive::AtatResp;

use super::types::{
    AdrEnabled as AdrEnabledVal, ConfirmedUplink as ConfirmedUplinkVal, DataRate as DataRateVal,
    LinkCheckResult, TransmitPower as TransmitPowerVal, UploadReportMode as UploadReportModeVal,
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
    pub times: u8,
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

/// Verify network link
#[derive(Clone, Debug, AtatResp)]
pub struct VerifyNetworkConnectionResponse {
    #[at_arg(position = 0)]
    pub connection: LinkCheckResult,
}

/// Verify network link
#[derive(Clone, Debug, AtatResp)]
pub struct AdrEnabled {
    #[at_arg(position = 0)]
    pub enabled: AdrEnabledVal,
}

/// Receive window parameters
#[derive(Clone, Debug, AtatResp)]
pub struct ReceiveWindowParameters {
    #[at_arg(position = 0)]
    pub rx1_datarate_offset: u8,
    #[at_arg(position = 1)]
    pub rx2_datarate: u8,
    #[at_arg(position = 1)]
    pub rx2_frequency: u32,
}

/// How long after sending to open the RX1 window, unit in seconds
#[derive(Clone, Debug, AtatResp)]
pub struct Rx1Delay {
    #[at_arg(position = 0)]
    pub delay_in_s: u16,
}
