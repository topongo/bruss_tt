use crate::error::{TTError,TTResult};
use crate::api_error::ApiError;
use crate::request::RequestOptions;
use crate::TTEndpoint;
use reqwest::{Client, RequestBuilder, Method};
use serde::Serialize;
use serde_json::error::Category;
use log::info;

pub struct TTClient {
    base_url: String,
    secret: String,
    client: Client
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

    pub(super) fn build_request<Q: Serialize>(&self, url: &'static str, options: RequestOptions<Q>) -> RequestBuilder {
        self.auth_req(options.method.unwrap_or(Method::GET), url)
            .query(&options.query)
    }

    pub async fn request_opt<O: TTEndpoint, Q: Serialize>(&self, options: Option<RequestOptions<Q>>) -> TTResult<Vec<O>> {
        info!("creating request for endpoint \"{}\"", O::ENDPOINT);
        let r = self.build_request(O::ENDPOINT, options.unwrap_or(RequestOptions::new()));
        info!("generated request: {:?}", r);
        let body = r 
            .send()
            .await?
            .text()
            .await?;

        info!("received text from TT: {body}");
        serde_json::from_str::<Vec<O>>(&body).map_err(|e| {
            match e.classify() {
                Category::Data => match serde_json::from_str::<ApiError>(&body).map_err(TTError::from) {
                    Ok(e_body) => TTError::ApiError(e_body),
                    Err(_) => e.into()
                }
                _ => e.into()
            }
        })
    }
    
    pub async fn request<O: TTEndpoint>(&self) -> TTResult<Vec<O>> {
        self.request_opt(Option::<RequestOptions<()>>::None).await
    }
}
