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
