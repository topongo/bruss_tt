use reqwest::Method;
use serde::Serialize;


#[derive(Default)]
pub struct RequestOptions<Q: Serialize> {
    pub method: Option<Method>,
    pub query: Option<Q>,
}

#[derive(Serialize)]
pub struct DummyQuery {}

impl RequestOptions<()> {
    pub fn none() -> Option<RequestOptions<DummyQuery>> {
        None
    }
}

impl<Q: Serialize> RequestOptions<Q> {
    pub fn new() -> Self {
        Self { method: None, query: None }
    }

    pub fn query(self, query: Q) -> Self {
        Self { query: Some(query), ..self }
    }

    pub fn method(self, method: Method) -> Self {
        Self { method: Some(method), ..self }
    }

    
}

