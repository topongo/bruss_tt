use serde::{Deserialize,Serialize};

use crate::client::{impl_vec_endpoint, Endpoint, TTClient, VecEndpoint};
use crate::error::TTResult;
use crate::TTType;
use crate::RequestOptions;

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

impl Endpoint<Vec<TTArea>> for TTClient {}
impl_vec_endpoint!(TTArea, "/areas");

// impl VecEndpoint<TTArea> for TTClient {
//     const ENDPOINT: &'static str = "/areas";
//
//     async fn request(&self) -> TTResult<Vec<TTArea>> {
//         <Self as Endpoint<Vec<TTArea>>>::decode(<Self as Endpoint<Vec<TTArea>>>::inner(self.auth_req(Method::GET, <Self as VecEndpoint<TTArea>>::ENDPOINT), Option::<()>::None).await?)
//     }
// }

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum AreaType {
    #[serde(rename(serialize = "e"), alias = "e")]
    E,
    #[serde(rename(serialize = "u"), alias = "u")]
    U
}

impl TTType for AreaType {}


