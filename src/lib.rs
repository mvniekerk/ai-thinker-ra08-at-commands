#![no_std]

use atat::atat_derive::AtatResp;

pub mod general;
pub mod lorawan;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct NoResponse;
