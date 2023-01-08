use atat::atat_derive::AtatCmd;
use heapless::String;

///! ### LoRaWAN commands and configuration
pub mod responses;
pub mod types;

use responses::*;
use types::{
    JoinMode as JoinModeVal,
    DownloadUploadSameOrDifferentFrequency as DownloadUploadSameOrDifferentFrequencyVal,
    LoRaWanClass as LoRaWanClassVal,
    LoRaWanOtaaJoinParameters as LoRaWanOtaaJoinParametersVal
};
use crate::NoResponse;

// Document version 0.1.0 13
// 4.1.2 LoRaWAN Network Related Parameter Setup Command Sets
// Command Description Option

/// 4.2.6 Get Join Mode (OTAA or ABP)
#[derive(Clone, AtatCmd)]
#[at_cmd("+CJOINMODE?", JoinMode)]
pub struct JoinModeGet;

/// 4.2.6 Set Join Mode (OTAA or ABP)
#[derive(Clone, AtatCmd)]
#[at_cmd("+CJOINMODE", NoResponse)]
pub struct JoinModeSet {
    #[at_arg(position = 0)]
    pub join_mode: JoinModeVal
}


/// 4.2.7 Get DevEUI - only applicable for OTAA
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDEVEUI?", DevEui)]
pub struct DevEuiGet;

/// 4.2.7 Set DevEUI - only applicable for OTAA
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDEVEUI", NoResponse)]
pub struct DevEuiSet {
    #[at_arg(position = 0)]
    pub dev_eui: String<32>,
}


/// 4.2.8 Get AppEUI - only applicable for OTAA
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPEUI?", AppEui)]
pub struct AppEuiGet;

/// 4.2.8 Set AppEUI - only applicable for OTAA
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPEUI", NoResponse)]
pub struct AppEuiSet {
    #[at_arg(position = 0)]
    pub app_eui: String<32>,
}


/// 4.2.9 Get AppKey - only applicable for OTAA
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPKEY?", AppEui)]
pub struct AppKeyGet;

/// 4.2.9 Set AppKey - only applicable for OTAA
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPKEY", NoResponse)]
pub struct AppKeySet {
    #[at_arg(position = 0)]
    pub app_key: String<64>,
}


/// 4.2.10 Get DevAddr - only applicable for ABP
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDEVADDR?", DevAddr)]
pub struct DevAddrGet;

/// 4.2.10 Set DevAddr - only applicable for ABP
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDEVADDR", NoResponse)]
pub struct DevAddrSet {
    #[at_arg(position = 0)]
    pub dev_addr: String<16>,
}


/// 4.2.11 Get App session key - only applicable for ABP
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPSKEY?", AppSessionKey)]
pub struct AppSessionKeyGet;

/// 4.2.11 Set App session key - only applicable for ABP
#[derive(Clone, AtatCmd)]
#[at_cmd("+CAPPSKEY", NoResponse)]
pub struct AppSessionKeySet {
    #[at_arg(position = 0)]
    pub app_session_key: String<64>,
}


/// 4.2.12 Get Network session key - only applicable for ABP
#[derive(Clone, AtatCmd)]
#[at_cmd("+CNWKSKEY?", NetworkSessionKey)]
pub struct NetworkSessionKeyGet;

/// 4.2.12 Set Network session key - only applicable for ABP
#[derive(Clone, AtatCmd)]
#[at_cmd("+CNWKSKEY", NoResponse)]
pub struct NetworkSessionKeySet {
    #[at_arg(position = 0)]
    pub network_session_key: String<64>,
}


/// 4.2.13 Get the frequency band mask, needs to be set before joining
#[derive(Clone, AtatCmd)]
#[at_cmd("+CFREQBANDMASK?", FrequencyBandMask)]
pub struct FrequencyBandMaskGet;

/// 4.2.13 Set the frequency band mask, needs to be set before joining
#[derive(Clone, AtatCmd)]
#[at_cmd("+CFREQBANDMASK", NoResponse)]
pub struct FrequencyBandMaskSet {
    #[at_arg(position = 0)]
    pub mask: String<8>,
}


/// 4.2.14 Get the frequency band mask, needs to be set before joining
#[derive(Clone, AtatCmd)]
#[at_cmd("+CULDLMODE?", DownloadUploadSameOrDifferentFrequency)]
pub struct DownloadUploadSameOrDifferentFrequencyGet;

/// 4.2.14 Set the frequency band mask, needs to be set before joining
#[derive(Clone, AtatCmd)]
#[at_cmd("+CULDLMODE", NoResponse)]
pub struct DownloadUploadSameOrDifferentFrequencySet {
    #[at_arg(position = 0)]
    pub mode: DownloadUploadSameOrDifferentFrequencyVal,
}


/// 4.2.36 Add one multicast address
#[derive(Clone, AtatCmd)]
#[at_cmd("+CADDMUTICAST", NoResponse)]
pub struct AddMulticastAddress {
    #[at_arg(position = 0)]
    pub device_addr: String<16>,
    #[at_arg(position = 1)]
    pub app_session_key: String<64>,
    #[at_arg(position = 2)]
    pub network_session_key: String<64>,
    #[at_arg(position = 3)]
    pub periodicity: u8,
    #[at_arg(position = 4)]
    pub data_rate: u8,
}

/// 4.2.37 Delete one multicast address
#[derive(Clone, AtatCmd)]
#[at_cmd("+CDELMUTICAST", NoResponse)]
pub struct DeleteMulticastAddress {
    #[at_arg(position = 0)]
    pub device_addr: String<16>,
}

/// 4.2.38 Query the amount of multicast groups
#[derive(Clone, AtatCmd)]
#[at_cmd("+CNUMMUTICAST", AmountOfMulticastGroups)]
pub struct AmountOfMulticastGroupsGet {
    #[at_arg(position = 0)]
    pub amount: u16,
}


/// 4.2.15 Get the work mode
#[derive(Clone, AtatCmd)]
#[at_cmd("+CWORKMODE?", WorkMode)]
pub struct WorkModeGet;

/// 4.2.15 Set the work mode. 2 is normal
#[derive(Clone, AtatCmd)]
#[at_cmd("+CWORKMODE", NoResponse)]
pub struct WorkModeSet {
    #[at_arg(position = 0)]
    pub mode: u8,
}


/// 4.2.16 Get the LoRaWAN class type
#[derive(Clone, AtatCmd)]
#[at_cmd("+CCLASS?", LoRaWanClass)]
pub struct LoRaWanClassGet;

/// 4.2.16 Set the LoRaWAN class type
#[derive(Clone, AtatCmd)]
#[at_cmd("+CCLASS", NoResponse)]
pub struct LoRaWanClassSet {
    #[at_arg(position = 0)]
    pub class: LoRaWanClassVal,
}


/// 4.2.17 Get the battery level
#[derive(Clone, AtatCmd)]
#[at_cmd("+CBL?", BatteryLevel)]
pub struct BatteryLevelGet;

/// 4.2.18 Get the device current status
#[derive(Clone, AtatCmd)]
#[at_cmd("+CSTATUS?", DeviceStatus)]
pub struct DeviceStatusGet;


/// 4.2.19 Get the LoRaWAN OTAA join parameters
#[derive(Clone, AtatCmd)]
#[at_cmd("+CJOIN?", LoRaWanOtaaJoinParameters)]
pub struct LoRaWanJoinOtaaParametersGet;

/// 4.2.19 Set the LoRaWAN OTAA join parameters
#[derive(Clone, AtatCmd)]
#[at_cmd("+CJOIN", NoResponse)]
pub struct LoRaWanOtaaJoinParametersSet {
    #[at_arg(position = 0)]
    pub class: LoRaWanOtaaJoinParametersVal,
}

/// 4.2.35 Get the ping slot frequency / periodicity (Class B)
#[derive(Clone, AtatCmd)]
#[at_cmd("+CPINGSLOTINFOREQ?", PingSlotFrequency)]
pub struct PingSlotFrequencyGet;

/// 4.2.35 Set the LoRaWAN OTAA join parameters
#[derive(Clone, AtatCmd)]
#[at_cmd("+CPINGSLOTINFOREQ", NoResponse)]
pub struct PingSlotFrequencySet {
    #[at_arg(position = 0)]
    pub periodicity: u16,
}