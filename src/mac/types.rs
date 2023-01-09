use atat::atat_derive::AtatEnum;

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum ConfirmedUplink {
    #[at_arg(value = 0)]
    Unconfirmed,
    #[at_arg(value = 1)]
    Confirmed,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum DataRate {
    #[at_arg(value = 0)]
    Sf12Bw125,
    #[at_arg(value = 1)]
    Sf11Bw125,
    #[at_arg(value = 2)]
    Sf10Bw125,
    #[at_arg(value = 3)]
    Sf9Bw125,
    #[at_arg(value = 4)]
    Sf8Bw125,
    #[at_arg(value = 5)]
    Sf7Bw125,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum UploadReportMode {
    #[at_arg(value = 0)]
    Aperiodic,
    # [at_arg(value = 1)]
    Periodic(u16)
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum TransmitPower {
    #[at_arg(value = 0)]
    _17dBm,
    #[at_arg(value = 1)]
    _15dBm,
    #[at_arg(value = 2)]
    _13dBm,
    #[at_arg(value = 3)]
    _11dBm,
    #[at_arg(value = 4)]
    _9dBm,
    #[at_arg(value = 5)]
    _7dBm,
    #[at_arg(value = 6)]
    _5dBm,
    #[at_arg(value = 7)]
    _3dBm,
}

