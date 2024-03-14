use std::time::Instant;

use reqwest::Method;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use chrono::{Local, NaiveDateTime, NaiveTime};

use crate::client::Endpoint;
use crate::{AreaType, TTClient, TTError, TTResult, TTRoute, TTType, VecEndpoint};

#[derive(Deserialize)]
pub struct TTTrip {
    #[serde(alias = "tripId")]
    pub id: String,
    pub delay: Option<f32>,
    #[serde(alias = "directionId")]
    pub direction: u16,
    #[serde(alias = "stopNext")]
    pub next_stop: u16,
    #[serde(alias = "stopLast")]
    pub last_stop: u16,
    #[serde(alias = "matricolaBus")]
    pub bus_id: Option<u16>,
    #[serde(alias = "routeId")]
    pub route: i32,
    #[serde(alias = "stopTimes")]
    pub stop_times: Vec<StopTime>
}

impl TTType for TTTrip {}

#[derive(Deserialize)]
pub struct StopTime {
    #[serde(alias = "arrivalTime", deserialize_with = "deserialize_time")]
    pub arrival: NaiveTime,
    #[serde(alias = "departureTime", deserialize_with = "deserialize_time")]
    pub departure: NaiveTime,
    #[serde(alias = "stopSequence")]
    pub sequence: u16,
    #[serde(alias = "stopId")]
    pub stop: u16,
}

impl TTType for StopTime {}

fn deserialize_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error> where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    let sp: Vec<&str> = s.split(":").collect();
    let (h, m, s): (u32, u32, u32) = (sp[0].parse().map_err(Error::custom)?, sp[1].parse().map_err(Error::custom)?, sp[2].parse().map_err(Error::custom)?);

    match NaiveTime::from_hms_opt(h, m, s) {
        Some(n) => Ok(n),
        None => Err(Error::custom("could not parse time".to_owned()))
    }
}

#[derive(Serialize)]
struct TripQuery {
    #[serde(rename = "routeId")]
    route_id: u16,
    #[serde(rename = "type")]
    ty: AreaType,
    limit: u32,
    #[serde(rename = "refDateTime")]
    time: NaiveDateTime
}

impl Endpoint<Vec<TTTrip>> for TTClient {
    async fn inner(request: reqwest::RequestBuilder, query: Option<Self::Query>) -> TTResult<Vec<TTTrip>> {
        request
            .query(&TripQuery { route_id: 396, ty: AreaType::U, limit: 1, time: Local::now().naive_local() })
            .send()
            .await?
            .json()
            .await
            .map_err(TTError::from)
    }
}

impl VecEndpoint<TTTrip> for TTClient {
    const ENDPOINT: &'static str = "/trip_new";

    async fn request(&self) -> TTResult<Vec<TTTrip>> {
        <Self as Endpoint<Vec<TTTrip>>>::inner(self.auth_req(Method::GET, <Self as VecEndpoint<TTTrip>>::ENDPOINT), Option::<()>::None).await
    }
}


