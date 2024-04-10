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

impl Into<u8> for AreaType {
    fn into(self) -> u8 {
        match self {
            Self::E => b'e',
            Self::U => b'u'
        }
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

