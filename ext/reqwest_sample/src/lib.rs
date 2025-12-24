use magnus::{function, prelude::*, Error, Ruby};

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

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("ReqwestSample")?;
    module.define_singleton_method("hello", function!(hello, 1))?;

    let client = module.define_class("Client", ruby.class_object())?;
    client.define_singleton_method("get", function!(client_get, 1))?;
    Ok(())
}
