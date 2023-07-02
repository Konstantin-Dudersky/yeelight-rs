use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AllProperties {
    ActiveMode,
    BgBright,
    BgCt,
    BgFlowing,
    BgFlowParams,
    BgHue,
    BgLmode,
    BgPower,
    BgRgb,
    BgSat,
    Bright,
    ColorMode,
    Ct,
    Delayoff,
    Flowing,
    FlowParams,
    Hue,
    MusicOn,
    Name,
    NlBr,
    Power,
    Rgb,
    Sat,
}
