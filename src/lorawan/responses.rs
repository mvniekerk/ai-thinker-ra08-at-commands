use crate::lorawan::types::{
    DeviceStatus as DeviceStatusVal,
    DownloadUploadSameOrDifferentFrequency as DownloadUploadSameOrDifferentFrequencyVal,
    JoinMode as JoinModeVal, LoRaWanClass as LoRaWanClassVal,
    LoRaWanOtaaJoinParameters as LoRaWanOtaaJoinParametersVal,
};
use atat::atat_derive::AtatResp;
use atat::serde_at::HexStr;
use heapless::String;

/// Join Mode response
#[derive(Clone, Debug, AtatResp)]
pub struct JoinMode {
    #[at_arg(position = 0)]
    pub join_mode: JoinModeVal,
}

/// Get DevEUI response
#[derive(Clone, Debug, AtatResp)]
pub struct DevEui {
    #[at_arg(position = 0)]
    pub dev_eui: HexStr<u64>,
}

/// Get AppEUI response
#[derive(Clone, Debug, AtatResp)]
pub struct AppEui {
    #[at_arg(position = 0)]
    pub app_eui: HexStr<u64>,
}

/// Get AppKey response
#[derive(Clone, Debug, AtatResp)]
pub struct AppKey {
    #[at_arg(position = 0)]
    pub app_eui: HexStr<u128>,
}

/// Get DevAddr response
#[derive(Clone, Debug, AtatResp)]
pub struct DevAddr {
    #[at_arg(position = 0)]
    pub app_eui: HexStr<u32>,
}

/// Get App session key response
#[derive(Clone, Debug, AtatResp)]
pub struct AppSessionKey {
    #[at_arg(position = 0)]
    pub app_s_key: HexStr<u128>,
}

/// Get Network session key response
#[derive(Clone, Debug, AtatResp)]
pub struct NetworkSessionKey {
    #[at_arg(position = 0)]
    pub network_session_key: HexStr<u128>,
}

/// Get frequency band mask response
#[derive(Clone, Debug, AtatResp)]
pub struct FrequencyBandMask {
    #[at_arg(position = 0)]
    pub mask: String<8>,
}

/// Get the mode if downloads and uploads should happen on a different frequency
#[derive(Clone, Debug, AtatResp)]
pub struct DownloadUploadSameOrDifferentFrequency {
    #[at_arg(position = 0)]
    pub mode: DownloadUploadSameOrDifferentFrequencyVal,
}

/// Get amount of multicast groups
#[derive(Clone, Debug, AtatResp)]
pub struct AmountOfMulticastGroups {
    #[at_arg(position = 0)]
    pub amount: u16,
}

/// Get the work mode. 2 is normal, else is error
#[derive(Clone, Debug, AtatResp)]
pub struct WorkMode {
    #[at_arg(position = 0)]
    pub mode: u8,
}

/// Get the device status
#[derive(Clone, Debug, AtatResp)]
pub struct DeviceStatus {
    #[at_arg(position = 0)]
    pub status: DeviceStatusVal,
}

/// Get the LoRaWAN class
#[derive(Clone, Debug, AtatResp)]
pub struct LoRaWanClass {
    #[at_arg(position = 0)]
    pub class: LoRaWanClassVal,
}

/// Device battery level
#[derive(Clone, Debug, AtatResp)]
pub struct BatteryLevel {
    #[at_arg(position = 0)]
    pub level: u16,
}

/// Get the LoRaWAN class
#[derive(Clone, Debug, AtatResp)]
pub struct LoRaWanOtaaJoinParameters {
    #[at_arg(position = 0)]
    pub parameters: LoRaWanOtaaJoinParametersVal,
}

/// Ping slot frequency
#[derive(Clone, Debug, AtatResp)]
pub struct PingSlotFrequency {
    #[at_arg(position = 0)]
    pub periodicity: u16,
}
