///! ###  General Commands
pub mod responses;
pub mod types;

use crate::NoResponse;
use atat::atat_derive::AtatCmd;
use responses::*;
use types::{Reboot as RebootVal, LogLevel as LogLevelVal};

/// 4.2.1 Manufacturer identification +CGMI
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGMI?", ManufacturerId)]
pub struct ManufacturerIdGet;

/// 4.2.2 Model identification +CGMM
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGMM?", ModelId)]
pub struct ModelIdGet;

/// 4.2.3 Software version identification +CGMR
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGMR?", SoftwareVersion)]
pub struct SoftwareVersionGet;

/// 4.2.4 Product serial number +CGSN
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGSN?", ProductSerial)]
pub struct ProductSerialNumberGet;

/// 4.2.5 Get Baud Rate
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGBR?", BaudRate)]
pub struct BaudRateGet;

/// 4.2.5 Set Baud Rate
#[derive(Clone, AtatCmd)]
#[at_cmd("+CGBR", NoResponse)]
pub struct BaudRateSet {
    #[at_arg(position = 0)]
    pub baud: u8,
}

/// 3.4.1 Reboot the device
#[derive(Clone, AtatCmd)]
#[at_cmd("+IREBOOT", NoResponse)]
pub struct Reboot {
    #[at_arg(position = 0)]
    pub when: RebootVal,
}

/// 3.4.2 Get the log level
#[derive(Clone, AtatCmd)]
#[at_cmd("+IREBOOT?", LogLevel)]
pub struct LogLevelGet;

/// 3.4.2 Set the log level
#[derive(Clone, AtatCmd)]
#[at_cmd("+ILOGLVL", NoResponse)]
pub struct LogLevelSet {
    #[at_arg(position = 0)]
    pub level: LogLevelVal,
}
