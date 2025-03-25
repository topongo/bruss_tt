#![feature(associated_type_defaults)]

mod area;
#[macro_use]
mod client;
mod error;
mod route;
mod stop;
mod trip;
mod api_error;
mod request;
#[cfg(feature = "stats")]
mod stats;

pub use area::{TTArea,AreaType,AreaTypeParseError};
pub use route::TTRoute;
pub use stop::TTStop; 
pub use trip::{TTTrip,TripQuery,StopTime};
pub use client::TTClient;
pub use error::{TTError,TTResult};
pub use request::RequestOptions;


use serde::de::DeserializeOwned;

pub trait TTType: DeserializeOwned {}

pub trait TTEndpoint: TTType {
    const ENDPOINT: &'static str;
}


