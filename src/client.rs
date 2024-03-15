use crate::error::{TTError,TTResult};
use crate::api_error::ApiError;
use crate::request::RequestOptions;
use reqwest::{Client, RequestBuilder, Method};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::error::Category;

pub struct TTClient {
    base_url: String,
    secret: String,
    client: Client
}

pub(super) trait Endpoint<Output: DeserializeOwned> {
    type Query: Serialize = ();

    fn decode(body: String) -> TTResult<Output> {
        // eprintln!("==> Response body: {}", body);
        serde_json::from_str::<Output>(&body).map_err(|e| {
            match e.classify() {
                Category::Data => match serde_json::from_str::<ApiError>(&body).map_err(TTError::from) {
                    Ok(e_body) => TTError::ApiError(e_body),
                    Err(e) => e
                }
                _ => e.into()
            }
        })
    }

    async fn inner(request: RequestBuilder) -> TTResult<String> {
        request
            .send()
            .await?
            .text()
            .await
            .map_err(TTError::from)
    }
}

#[allow(private_bounds)]
pub trait VecEndpoint<Type: DeserializeOwned>: Endpoint<Vec<Type>> {
    const ENDPOINT: &'static str;

    #[allow(async_fn_in_trait)]
    async fn request<Q: Serialize>(&self, options: Option<RequestOptions<Q>>) -> TTResult<Vec<Type>>;
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

    
    pub(super) fn build_request<Q: Serialize>(&self, url: &'static str, options: RequestOptions<Q>) -> RequestBuilder {
        self.auth_req(options.method.unwrap_or(Method::GET), url)
            .query(&options.query)
    }
}

macro_rules! impl_vec_endpoint {
    ($output:ident, $endpoint:literal) => {
        impl VecEndpoint<$output> for TTClient {
            const ENDPOINT: &'static str = $endpoint;

            async fn request<Q: Serialize>(&self, options: Option<RequestOptions<Q>>) -> TTResult<Vec<$output>> {
                let options = options.unwrap_or(RequestOptions::new());

                <Self as Endpoint<Vec<$output>>>::decode(
                    <Self as Endpoint<Vec<$output>>>::inner(
                        self.build_request(<Self as VecEndpoint<$output>>::ENDPOINT, options)
                    ).await?
                )
            }
        }
    };
}

pub(crate) use impl_vec_endpoint;

