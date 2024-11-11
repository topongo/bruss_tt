use reqwest::Method;
use serde::Serialize;


#[derive(Default, Debug)]
pub struct RequestOptions<Q: Serialize> {
    pub method: Option<Method>,
    pub query: Option<Q>,
    pub endpoint: &'static str,
    pub id: Option<String>,
}

#[derive(Serialize)]
pub struct DummyQuery {}

impl RequestOptions<()> {
    pub fn none() -> Option<RequestOptions<DummyQuery>> {
        None
    }
}

impl<Q: Serialize> RequestOptions<Q> {
    pub fn new(endpoint: &'static str) -> Self {
        Self { method: None, query: None, endpoint, id: None }
    }

    pub fn query(self, query: Q) -> Self {
        Self { query: Some(query), ..self }
    }

    pub fn method(self, method: Method) -> Self {
        Self { method: Some(method), ..self }
    }

    pub fn id(self, id: String) -> Self {
        Self { id: Some(id), ..self }
    }
}

