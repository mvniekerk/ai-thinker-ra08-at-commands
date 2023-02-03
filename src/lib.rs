#![no_std]

use atat::atat_derive::AtatResp;

pub mod general;
pub mod lorawan;
pub mod mac;
pub mod rxtx;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct NoResponse;
