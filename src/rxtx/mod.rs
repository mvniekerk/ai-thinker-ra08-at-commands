pub mod urc;

use atat_derive::AtatCmd;

// /// 4.2.20 Send dataframe
// /// TODO Non-0 response denotes error response. Need to be better with the parsing to an enum
// #[derive(Clone, AtatCmd)]
// #[at_cmd("+CLINKCHECK", VerifyNetworkConnectionResponse)]
// pub struct VerifyNetworkConnection {
//     #[at_arg(position = 0)]
//     pub mode: LinkCheckMode,
// }