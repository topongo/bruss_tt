use crate::route::TTRoute;
use crate::stop::TTStop;
use crate::area::TTArea;
use crate::error::{TTError,TTResult};
use reqwest::{Client, Request, RequestBuilder, Method};
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};

pub struct TTClient {
    base_url: String,
    secret: String,
    client: Client
}

pub(super) trait Endpoint<Output: DeserializeOwned> {
    type Query: Serialize = ();

    async fn inner(request: RequestBuilder, query: Option<Self::Query>) -> TTResult<Output> {
        request
            .send()
            .await?
            .json::<Output>()
            .await
            .map_err(TTError::from)
    }
}

pub trait VecEndpoint<Type: DeserializeOwned>: Endpoint<Vec<Type>> {
    const ENDPOINT: &'static str;

    async fn request(&self) -> TTResult<Vec<Type>>;
    // async fn request_opt<Q>(&self, query: Option<Q>) -> TTResult<Vec<Type>> where Q: Serialize;
}


impl TTClient {
    pub fn new(base_url: String, secret: String) -> Self {
        Self { base_url, secret, client: Client::new() }
    }

    pub(super) fn auth_req(&self, method: Method, url: &str) -> RequestBuilder {
        self.client
            .request(method, format!("{}{}", self.base_url, url))
            .header("authorization", format!("Basic {}", self.secret))
    }
}


