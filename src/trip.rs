use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use chrono::{NaiveDateTime, NaiveTime};

use crate::{AreaType,TTEndpoint,TTType};

#[derive(Debug, Deserialize)]
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

impl TTEndpoint for TTTrip {
    const ENDPOINT: &'static str = "/trips_new";
}

#[derive(Debug, Deserialize)]
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

fn deserialize_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error> where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    let sp: Vec<&str> = s.split(":").collect();
    let (h, m, s): (u32, u32, u32) = (sp[0].parse().map_err(Error::custom)?, sp[1].parse().map_err(Error::custom)?, sp[2].parse().map_err(Error::custom)?);

    match NaiveTime::from_hms_opt(h, m, s) {
        Some(n) => Ok(n),
        None => Err(Error::custom("could not parse time".to_owned()))
    }
}

#[derive(Debug, Serialize)]
pub struct TripQuery {
    #[serde(rename = "routeId")]
    pub route_id: u16,
    #[serde(rename = "type")]
    pub ty: AreaType,
    pub limit: u32,
    #[serde(rename = "refDateTime")]
    pub time: NaiveDateTime
}

