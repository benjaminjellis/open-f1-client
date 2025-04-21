use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Flag {
    #[serde(rename = "BLACK AND WHITE")]
    BlackAndWhite,
    #[serde(rename = "RED")]
    Red,
    #[serde(rename = "YELLOW")]
    Yellow,
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "CHEQUERED")]
    Chequered,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "CLEAR")]
    Clear,
    #[serde(rename = "DOUBLE YELLOW")]
    DoubleYellow,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RaceControlEvent {
    CarEvent,
    Drs,
    Flag,
    SafetyCar,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RaceControlEventScope {
    Track,
    Driver,
    Sector,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TyreCompound {
    Hard,
    Medium,
    Soft,
    Wet,
    TestUnknown,
    Unknown,
    Intermediate,
}

// impl Display for Flag {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Flag::BlackAndWhite => write!(f, "BLACK AND WHITE"),
//             Flag::Red => write!(f, "RED"),
//             Flag::Yellow => write!(f, "YELLOW"),
//             Flag::Blue => write!(f, "BLUE"),
//             Flag::Chequered => write!(f, "CHEQUERED"),
//         }
//     }
// }
//
// impl FromStr for Flag {
//     type Err = OpenF1ClientError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "BLACK AND WHITE" => Ok(Self::BlackAndWhite),
//             "RED" => Ok(Self::Red),
//             other => Err(OpenF1ClientError::UnknownFlagVariant {
//                 flag: other.to_string(),
//             }),
//         }
//     }
// }
