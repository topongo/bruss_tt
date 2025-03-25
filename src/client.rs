use std::fmt::Debug;

use crate::error::{TTError,TTResult};
use crate::api_error::ApiError;
use crate::request::RequestOptions;
use crate::TTEndpoint;
use reqwest::{Client, RequestBuilder, Method};
use serde::Serialize;
use serde_json::error::Category;
use log::{debug, info, trace};
use tokio::sync::Mutex;

pub struct TTClient {
    base_url: String,
    secret: String,
    client: Client,
    telemetry: Mutex<Telemetry>,
}

#[derive(Debug)]
struct Telemetry {
    pub requests: usize,
    // pub bytes_sent: u64,
    pub bytes_received: usize,
    // pub errors: u64,
    pub instant: std::time::Instant,
}

impl Default for Telemetry {
    fn default() -> Self {
        Self { requests: 0, bytes_received: 0, instant: std::time::Instant::now() }
    }
}

impl TTClient {
    pub fn new(base_url: String, secret: String) -> Self {
        Self { base_url, secret, client: Client::new(), telemetry: Mutex::new(Telemetry::default()) }
    }

    pub(super) fn auth_req(&self, method: Method, url: &str, extra: Option<String>) -> RequestBuilder {
        self.client
            .request(method, format!("{}{}/{}", self.base_url, url, extra.unwrap_or("".to_owned())))
            .header("authorization", format!("Basic {}", self.secret))
    }

    pub(super) fn build_request<Q: Serialize>(&self, options: RequestOptions<Q>, extra: Option<String>) -> RequestBuilder {
        self.auth_req(options.method.unwrap_or(Method::GET), options.endpoint, extra)
            .query(&options.query)
    }

    pub async fn request_opt<O: TTEndpoint, Q: Serialize + Debug>(&self, options: Option<RequestOptions<Q>>) -> TTResult<Vec<O>> {
        debug!("creating request for endpoint \"{}\"", O::ENDPOINT);
        debug!("using options: {:?}", options);
        let r = self.build_request(options.unwrap_or(RequestOptions::new(O::ENDPOINT)), None);
        info!("generated request: {:?}", r);
        info!("requesting endpoint {}", O::ENDPOINT);
        let response = r 
            .send()
            .await?;

        debug!("response: {:?}", response);
        self.telemetry.lock().await.requests += 1;
        let body = response
            .text()
            .await?;

        self.telemetry.lock().await.bytes_received += body.as_bytes().len();

        trace!("received text from TT: {body}");
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

    pub async fn request_one_opt<O: TTEndpoint, Q: Serialize + Debug>(&self, id: String, options: Option<RequestOptions<Q>>) -> TTResult<O> {
        debug!("creating request for endpoint \"{}\"", O::ENDPOINT);
        debug!("using options: {:?}", options);
        let r = self.build_request(options.unwrap_or(RequestOptions::new(O::ENDPOINT)), Some(id.clone()));
        info!("generated request: {:?}", r);
        info!("requesting endpoint {} and id {}", O::ENDPOINT, id);
        let response = r 
            .send()
            .await?;

        debug!("response: {:?}", response);
        self.telemetry.lock().await.requests += 1;
        let body = response
            .text()
            .await?;
        
        self.telemetry.lock().await.bytes_received += body.as_bytes().len();

        trace!("received text from TT: {body}");
        serde_json::from_str::<O>(&body).map_err(|e| {
            match e.classify() {
                Category::Data => match serde_json::from_str::<ApiError>(&body).map_err(TTError::from) {
                    Ok(e_body) => TTError::ApiError(e_body),
                    Err(_) => e.into()
                }
                _ => e.into()
            }
        })
    }

    pub async fn request_one<O: TTEndpoint>(&self, id: String) -> TTResult<O> {
        self.request_one_opt::<O, ()>(id, Option::<RequestOptions<()>>::None).await
    }

    pub async fn print_telemetry(&self) -> String {
        let t = self.telemetry.lock().await;
        format!(
            "num requests: {}\nbytes received: {}\nelapsed time: {:.2}s\nrequests/s: {:.2}\nbytes/s: {}", 
            t.requests,
            t.bytes_received,
            t.instant.elapsed().as_secs(),
            t.requests as f64 / t.instant.elapsed().as_secs_f64(),
            t.bytes_received as f64 / t.instant.elapsed().as_secs_f64()
        )
    }
}
