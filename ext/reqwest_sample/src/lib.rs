use magnus::{function, method, prelude::*, Error, RHash, Ruby};
use std::collections::HashMap;

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

// Simple binding for reqwest::get
fn client_get(url: String) -> Result<String, Error> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

    rt.block_on(async {
        reqwest::get(&url)
            .await
            .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?
            .text()
            .await
            .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))
    })
}

#[magnus::wrap(class = "ReqwestSample::Client")]
struct Client {
    inner: reqwest::Client,
    runtime: tokio::runtime::Runtime,
}

impl Client {
    fn new() -> Result<Self, Error> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

        let inner = reqwest::Client::new();

        Ok(Client { inner, runtime })
    }

    fn get(&self, url: String) -> Result<Response, Error> {
        self.runtime.block_on(async {
            let resp = self
                .inner
                .get(&url)
                .send()
                .await
                .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

            Response::from_reqwest(resp).await
        })
    }
}

#[magnus::wrap(class = "ReqwestSample::Response")]
struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    async fn from_reqwest(resp: reqwest::Response) -> Result<Self, Error> {
        let status = resp.status().as_u16();

        let mut headers = HashMap::new();
        for (name, value) in resp.headers() {
            if let Ok(v) = value.to_str() {
                headers.insert(name.to_string(), v.to_string());
            }
        }

        let body = resp
            .text()
            .await
            .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

        Ok(Response {
            status,
            headers,
            body,
        })
    }

    fn status(&self) -> u16 {
        self.status
    }

    fn headers(&self) -> Result<RHash, Error> {
        let ruby = Ruby::get().unwrap();
        let hash = ruby.hash_new();
        for (key, value) in &self.headers {
            hash.aset(key.clone(), value.clone())?;
        }
        Ok(hash)
    }

    fn body(&self) -> String {
        self.body.clone()
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("ReqwestSample")?;
    module.define_singleton_method("hello", function!(hello, 1))?;

    let response = module.define_class("Response", ruby.class_object())?;
    response.define_method("status", method!(Response::status, 0))?;
    response.define_method("headers", method!(Response::headers, 0))?;
    response.define_method("body", method!(Response::body, 0))?;

    let client = module.define_class("Client", ruby.class_object())?;
    client.define_singleton_method("get", function!(client_get, 1))?;
    client.define_singleton_method("new", function!(Client::new, 0))?;
    client.define_method("get", method!(Client::get, 1))?;
    Ok(())
}
