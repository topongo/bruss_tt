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

pub use {area::TTArea, area::AreaType, route::TTRoute, stop::TTStop, trip::{TTTrip,TripQuery}};
pub use client::TTClient;
pub use error::{TTError,TTResult};
pub use request::RequestOptions;


use serde::de::DeserializeOwned;

pub trait TTType: DeserializeOwned {}

pub trait TTEndpoint: TTType {
    const ENDPOINT: &'static str;
}


