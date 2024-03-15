use serde::{Serialize,Deserialize};
use crate::TTType;
use crate::{client::Endpoint, TTClient, TTResult, VecEndpoint};
use crate::area::AreaType;
use crate::RequestOptions;

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
    pub street: String,
    pub town: String,
    #[serde(rename = "type")]
    pub ty: AreaType,
    #[serde(alias = "wheelchairBoarding")]
    pub wheelchair_boarding: bool
}

impl TTType for TTStop {}

impl Endpoint<Vec<TTStop>> for TTClient {}
impl_vec_endpoint!(TTStop, "/stops");

