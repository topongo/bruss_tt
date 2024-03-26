use serde::{Deserialize, Deserializer, Serialize};
use crate::{TTEndpoint, TTType};
use crate::area::AreaType;

#[derive(Serialize,Deserialize,Debug)]
pub struct TTStop {
    #[serde(alias = "stopId")]
    pub id: u16,
    #[serde(alias = "stopCode")]
    pub code: String,
    #[serde(alias = "stopDesc")]
    pub description: String,
    #[serde(alias = "stopLat")]
    pub lat: f32,
    #[serde(alias = "stopLon")]
    pub lng: f32,
    #[serde(alias = "stopLevel")]
    pub altitude: i32,
    #[serde(alias = "stopName")]
    pub name: String,
    pub street: Option<String>,
    pub town: Option<String>,
    #[serde(rename = "type")]
    pub ty: AreaType,
    #[serde(alias = "wheelchairBoarding", deserialize_with = "deserialize_wheelchair_boarding")]
    pub wheelchair_boarding: bool
}

impl TTType for TTStop {}

impl TTEndpoint for TTStop {
    const ENDPOINT: &'static str = "/stops";
}

fn deserialize_wheelchair_boarding<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: Deserializer<'de> {
    let d = u8::deserialize(deserializer)?;

    Ok(match d {
        1 => true,
        _ => true
    })
} 

