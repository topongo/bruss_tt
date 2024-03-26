use crate::{TTEndpoint, TTType};
use serde::{Serialize,Deserialize,Deserializer};

#[derive(Serialize,Deserialize,Debug)]
pub struct TTRoute {
    #[serde(alias="routeId")]
    pub id: u16,
    #[serde(alias="routeType")]
    pub ty: u16,
    #[serde(alias="areaId")]
    pub area: u16,
    #[serde(deserialize_with="parse_color",alias="routeColor")]
    pub color: String,
    #[serde(alias="routeLongName")]
    pub name: String,
    #[serde(alias="routeShortName")]
    pub code: String,
}

impl TTType for TTRoute {}

impl TTEndpoint for TTRoute {
    const ENDPOINT: &'static str = "/routes";
}

fn parse_color<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("CCCCCC".to_string())
        })
}

