#![feature(associated_type_defaults)]

mod area;
mod client;
mod error;
mod route;
mod stop;
mod trip;

pub use {area::TTArea, area::AreaType, route::TTRoute, stop::TTStop, trip::{TTTrip}};
pub use client::{TTClient,VecEndpoint};
pub use error::{TTError,TTResult};
use serde::de::DeserializeOwned;

pub trait TTType: DeserializeOwned {}

