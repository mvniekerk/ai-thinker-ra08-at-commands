//! Responses for General Commands
use atat::atat_derive::AtatResp;
use heapless::String;
use super::types::LogLevel as LogLevelVal;

/// 4.2.1 Manufacturer identification
/// Manufacturer Identification
#[derive(Clone, Debug, AtatResp)]
pub struct ManufacturerId {
    #[at_arg(position = 0)]
    pub id: String<64>,
}

/// 4.2.2 Model identification
#[derive(Clone, Debug, AtatResp)]
pub struct ModelId {
    #[at_arg(position = 0)]
    pub id: String<64>,
}

/// Software version identification
#[derive(Clone, Debug, AtatResp)]
pub struct SoftwareVersion {
    #[at_arg(position = 0)]
    pub id: String<64>,
}

/// Product serial information
#[derive(Clone, Debug, AtatResp)]
pub struct ProductSerial {
    #[at_arg(position = 0)]
    pub id: String<64>,
}

/// Baud rate
/// Should not exceed 9600
#[derive(Clone, Debug, AtatResp)]
pub struct BaudRate {
    #[at_arg(position = 0)]
    pub baud: u8,
}

/// Log level
#[derive(Clone, Debug, AtatResp)]
pub struct LogLevel {
    #[at_arg(position = 0)]
    pub level: LogLevelVal,
}

