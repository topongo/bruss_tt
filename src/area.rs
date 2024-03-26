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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AreaType {
    #[serde(rename(serialize = "e"), alias = "e")]
    E,
    #[serde(rename(serialize = "u"), alias = "u")]
    U
}

impl TTType for AreaType {}


