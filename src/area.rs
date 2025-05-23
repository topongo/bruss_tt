use std::{error::Error as StdError, fmt::Display, str::FromStr};
use serde::{Deserialize,Serialize};

use crate::{TTEndpoint, TTType};

#[derive(Deserialize, Debug)]
pub struct TTArea {
    #[serde(rename = "areaId")]
    pub id: u16,
    #[serde(rename = "areaDesc")]
    pub label: String,
    #[serde(rename = "type")]
    pub ty: AreaType,
}

impl TTType for TTArea {}

impl TTEndpoint for TTArea {
    const ENDPOINT: &'static str = "/areas";
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Hash, Eq, Copy)]
pub enum AreaType {
    #[serde(rename(serialize = "e"), alias = "e")]
    E,
    #[serde(rename(serialize = "u"), alias = "u")]
    U
}

impl TTType for AreaType {}

impl FromStr for AreaType {
    type Err = AreaTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Self::E),
            "u" => Ok(Self::U),
            _ => Err(AreaTypeParseError)
        }
    }
}

impl Display for AreaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AreaType::E => write!(f, "e"),
            AreaType::U => write!(f, "u"),
        }
    }
}

impl From<AreaType> for u8 {
    fn from(value: AreaType) -> u8 {
        match value {
            AreaType::E => b'e',
            AreaType::U => b'u'
        }
    }
}

impl From<AreaType> for &'static str {
    fn from(value: AreaType) -> &'static str {
        match value {
            AreaType::E => "e",
            AreaType::U => "u"
        }
    }
}

#[derive(Debug)]
pub struct AreaTypeParseError;

impl StdError for AreaTypeParseError {}

impl Display for AreaTypeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn test_se() {
    let raw = r#"
    {
        "areaId": 23,
        "areaDesc": "Urbano Trento",
        "type": "U"
    }        
    "#;
    
    let data: TTArea = serde_json::from_str(raw).unwrap();
    assert_eq!(data.ty, AreaType::U);
    assert_eq!(data.label, "Urbano Trento");
    assert_eq!(data.id, 23);
}

