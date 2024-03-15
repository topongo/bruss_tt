use reqwest::StatusCode;
use serde::{Deserialize, Deserializer};


// {"timestamp":"2024-03-15T11:50:59.310+0000","status":404,"error":"Not Found","message":"No message available","path":"/gtlservice/trip_new"}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ApiError {
    #[serde(deserialize_with = "deserialize_status")]
    status: StatusCode,
    error: String,
    message: String,
    path: String
}

fn deserialize_status<'de, D>(deserializer: D) -> Result<StatusCode, D::Error> where D: Deserializer<'de> {
    let s = u16::deserialize(deserializer)?;
    StatusCode::from_u16(s).map_err(|e| serde::de::Error::custom(e.to_string()))
}

