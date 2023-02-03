use atat_derive::AtatEnum;

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum Reboot {
    #[at_arg(value = 0)]
    Immediately,
    #[at_arg(value = 1)]
    WhenDone,
}

#[derive(Clone, PartialEq, Debug, AtatEnum)]
pub enum LogLevel {
    #[at_arg(value = 0)]
    Disabled,
    #[at_arg(value = 1)]
    Level1,
    #[at_arg(value = 2)]
    Level2,
    #[at_arg(value = 3)]
    Level3,
    #[at_arg(value = 4)]
    Level4,
    #[at_arg(value = 4)]
    Level5,
}

