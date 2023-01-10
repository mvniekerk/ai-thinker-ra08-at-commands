use atat::atat_derive::{AtatEnum, AtatLen};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum JoinMode {
    #[at_arg(value = 0)]
    OTAA,
    #[at_arg(value = 1)]
    ABP,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum DownloadUploadSameOrDifferentFrequency {
    #[at_arg(value = 1)]
    SameFrequency,
    #[at_arg(value = 2)]
    DifferentFrequency,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum ClassBBranch {
    /// Periodicity as parameter
    #[at_arg(value = 0)]
    Branch0(u16),
    #[at_arg(value = 1)]
    Branch1(ClassBBranch1Parameters),
}

/// Get frequency band mask response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, AtatLen)]
pub struct ClassBBranch1Parameters {
    #[at_arg(position = 0)]
    pub frequency_hz: f32,
    #[at_arg(position = 1)]
    pub data_rate: u32,
    #[at_arg(position = 2)]
    pub ping_frequency_hz: f32,
    #[at_arg(position = 2)]
    pub data_rate_ping_slot: u16,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum LoRaWanClass {
    #[at_arg(value = 0)]
    ClassA,
    #[at_arg(value = 1)]
    ClassB(ClassBBranch),
    #[at_arg(value = 1)]
    ClassC,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum DeviceStatus {
    /// Currently no data operation
    #[at_arg(value = 00)]
    NoDataOperation,
    /// There's data in sending
    #[at_arg(value = 01)]
    DataInSending,
    /// There's data sent but failed
    #[at_arg(value = 02)]
    DataSentButFailed,
    /// There's data sent with success
    #[at_arg(value = 03)]
    DataSentAndSuccess,
    /// JOIN success (only appears in first join procedure)
    #[at_arg(value = 04)]
    JoinSuccess,
    /// JOIN failed (only appears in first join procedure)
    #[at_arg(value = 05)]
    JoinFailure,
    /// Network may be abnormal - link check failure
    #[at_arg(value = 06)]
    NetworkMayBeAbnormalLinkCheckFailure,
    /// Data sent success, but no download
    #[at_arg(value = 07)]
    DataSentSuccessNoDownload,
    /// Data sent success, download available
    #[at_arg(value = 08)]
    DataSentSuccessDownloadAvailable,
}

/// LoRaWAN join command or status
#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum LoRaWanJoin {
    #[at_arg(value = 0)]
    StopJoin,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum LoRaWanAutoJoin {
    #[at_arg(value = 0)]
    Disabled,
    #[at_arg(value = 1)]
    Enabled,
}

/// LoRaWAN join parameters
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, AtatLen)]
pub struct LoRaWanOtaaJoinParameters {
    #[at_arg(position = 0)]
    pub auto_join: LoRaWanAutoJoin,
    /// Join period in seconds, default 8. Range 7 .. 255
    #[at_arg(position = 1)]
    pub join_period_in_s: u8,
    /// Retry times, range 1..256
    #[at_arg(position = 2)]
    pub retries: u16,
}
