use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use chrono::{DateTime, NaiveDateTime, TimeDelta, Utc};

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
    pub route: u16,
    #[serde(alias = "stopTimes")]
    pub stop_times: Vec<StopTime>,
    #[serde(alias = "type")]
    pub ty: AreaType,
    #[serde(alias = "tripHeadsign")]
    pub headsign: String,
    #[serde(alias = "lastEventRecivedAt")]
    pub last_event: Option<DateTime<Utc>>,
}

impl TTType for TTTrip {}

impl TTEndpoint for TTTrip {
    const ENDPOINT: &'static str = "/trips";
}

#[derive(Debug, Deserialize)]
pub struct StopTime {
    #[serde(alias = "arrivalTime", deserialize_with = "deserialize_time")]
    pub arrival: TimeDelta,
    #[serde(alias = "departureTime", deserialize_with = "deserialize_time")]
    pub departure: TimeDelta,
    #[serde(alias = "stopSequence")]
    pub sequence: u16,
    #[serde(alias = "stopId")]
    pub stop: u16,
}

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<TimeDelta, D::Error> where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(TimeDelta::zero())
    } else {
        let sp: Vec<&str> = s.split(":").collect();
        let (h, m, s) = (sp[0].parse().map_err(Error::custom)?, sp[1].parse().map_err(Error::custom)?, sp[2].parse().map_err(Error::custom)?);

        // why tho? tt being weird...
        // h %= 24;

        Ok(TimeDelta::hours(h) + TimeDelta::minutes(m) + TimeDelta::seconds(s))
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

