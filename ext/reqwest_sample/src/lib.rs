use hickory_proto::rr::rdata::svcb::{SvcParamKey, SvcParamValue};
use hickory_proto::rr::record_data::RData;
use hickory_proto::rr::{Name, RecordType};
use hickory_resolver::name_server::TokioConnectionProvider;
use hickory_resolver::Resolver;
use magnus::{function, method, prelude::*, Error, RHash, Ruby};
use std::collections::HashMap;
use std::error::Error as StdError;
use std::str::FromStr;

fn hello(subject: String) -> String {
    format!("Hello from Rust, {subject}!")
}

fn extract_domain(url_str: &str) -> Option<String> {
    url::Url::parse(url_str)
        .ok()?
        .host_str()
        .map(|s| s.to_string())
}

async fn lookup_https_record(domain: &str) -> Option<Vec<String>> {
    let resolver = match Resolver::builder(TokioConnectionProvider::default()) {
        Ok(builder) => builder.build(),
        Err(_) => {
            return None;
        }
    };

    let name = match Name::from_str(&format!("{}.", domain)) {
        Ok(n) => n,
        Err(_) => {
            return None;
        }
    };

    match resolver.lookup(name, RecordType::HTTPS).await {
        Ok(response) => {
            for record in response.iter() {
                if let RData::HTTPS(https_record) = record {
                    for (key, value) in https_record.svc_params() {
                        if let SvcParamKey::Alpn = key {
                            if let SvcParamValue::Alpn(alpn) = value {
                                return Some(alpn.0.clone());
                            }
                        }
                    }
                }
            }
            None
        }
        Err(_) => None,
    }
}

// Simple binding for reqwest::get
fn client_get(url: String) -> Result<String, Error> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

    rt.block_on(async {
        reqwest::get(&url)
            .await
            .map_err(|e| {
                let mut msg = e.to_string();
                let mut source: Option<&(dyn StdError + 'static)> = e.source();
                while let Some(s) = source {
                    msg.push_str(&format!("\n  caused by: {}", s));
                    source = s.source();
                }
                Error::new(magnus::exception::runtime_error(), msg)
            })?
            .text()
            .await
            .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))
    })
}

#[magnus::wrap(class = "ReqwestSample::Client")]
struct Client {
    inner: reqwest::Client,
    h3_client: reqwest::Client,
    runtime: tokio::runtime::Runtime,
}

impl Client {
    fn new() -> Result<Self, Error> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

        let (inner, h3_client) = runtime.block_on(async {
            let inner = reqwest::Client::builder()
                .build()
                .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

            let h3_client = reqwest::Client::builder()
                .use_rustls_tls()
                .http3_prior_knowledge()
                .build()
                .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?;

            Ok::<_, Error>((inner, h3_client))
        })?;

        Ok(Client { inner, h3_client, runtime })
    }

    fn get(&self, url: String) -> Result<Response, Error> {
        self.runtime.block_on(async {
            let supports_h3 = if let Some(domain) = extract_domain(&url) {
                let alpn = lookup_https_record(&domain).await;
                alpn.as_ref().map_or(false, |a| a.iter().any(|p| p == "h3"))
            } else {
                false
            };

            let resp = if supports_h3 {
                match self
                    .h3_client
                    .get(&url)
                    .version(reqwest::Version::HTTP_3)
                    .send()
                    .await
                {
                    Ok(r) => r,
                    Err(_) => {
                        self.inner
                            .get(&url)
                            .send()
                            .await
                            .map_err(|e| {
                                Error::new(magnus::exception::runtime_error(), e.to_string())
                            })?
                    }
                }
            } else {
                self.inner
                    .get(&url)
                    .send()
                    .await
                    .map_err(|e| Error::new(magnus::exception::runtime_error(), e.to_string()))?
            };

            Response::from_reqwest(resp).await
        })
    }
}

#[magnus::wrap(class = "ReqwestSample::Response")]
struct Response {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
    version: String,
}

impl Response {
    async fn from_reqwest(resp: reqwest::Response) -> Result<Self, Error> {
        let status = resp.status().as_u16();
        let version = format!("{:?}", resp.version());

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
            version,
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

    fn version(&self) -> String {
        self.version.clone()
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
    response.define_method("version", method!(Response::version, 0))?;

    let client = module.define_class("Client", ruby.class_object())?;
    client.define_singleton_method("get", function!(client_get, 1))?;
    client.define_singleton_method("new", function!(Client::new, 0))?;
    client.define_method("get", method!(Client::get, 1))?;
    Ok(())
}
