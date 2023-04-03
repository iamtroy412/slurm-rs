//! A rust library for interacting with the Slurm REST API.
//!
//! For more information, the Slurm REST API is documented at
//! <https://slurm.schedmd.com/rest_api.html>
use anyhow::Result;
use reqwest::{header, Client, Method, Request, Url};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

const SLURM_USER: &str = "X-SLURM-USER-NAME";
const SLURM_TOKEN: &str = "X-SLURM-USER-TOKEN";
const SLURM_ENDPOINT: &str = "X-SLURM-ENDPOINT";
const SLURM_API_VERSION: &str = "v0.0.38";

/// Entrypoint for interacting with the API.
/// To authenticate with the API, we need a user and a token.
pub struct Slurm {
    user: String,
    token: String,
    endpoint: Url,
    client: Arc<Client>,
}

impl Slurm {
    /// Create a new Slurm client struct. It takes any type that can convert
    /// into a &str and any type that can convert into a URL for the endpoint.
    /// Since this lib is useless withouth a client to connect with, this
    /// will panic if creating a client fails.
    pub fn new<U, T, L>(user: U, token: T, url: L) -> Self
    where
        U: ToString,
        T: ToString,
        L: ToString,
    {
        let client = Client::builder().build();
        match client {
            Ok(c) => Slurm {
                user: user.to_string(),
                token: token.to_string(),
                endpoint: Url::parse(&url.to_string()).expect("Unable to parse endpoint into URL!"),
                client: Arc::new(c),
            },
            Err(e) => panic!("Unable to create client: {e:?}"),
        }
    }

    /// Create a new Slurm client struct from environment variables.
    /// It takes any type that can convert into a &str.
    /// Since this lib is useless withouth a client to connect with, this
    /// will panic if creating a client fails.
    pub fn new_from_env() -> Self {
        let endpoint = env::var(SLURM_ENDPOINT).expect("{SLURM_ENDPOINT} should be set!");
        let user = env::var(SLURM_USER).expect("{SLURM_USER} should be set!");
        let token = env::var(SLURM_TOKEN).expect("{SLURM_TOKEN} should be set!");

        Slurm::new(user, token, endpoint)
    }

    // This will be our internal request builder.
    fn request<B>(
        &self,
        method: Method,
        path: &str,
        body: B,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<Request>
    where
        B: Serialize,
    {
        // https://slurm-endpoint/{slurm,slurmdb}/v0.0.38/{nodes, diag, etc..}
        let url = self.endpoint.join("slurm")?;
        let url = url.join(SLURM_API_VERSION)?;
        let url = url.join(path)?;

        // Build auth headers
        let user_header_name =
            header::HeaderName::from_bytes(SLURM_USER.to_lowercase().as_bytes())?;
        let user_header_val = header::HeaderValue::from_str(&self.user)?;
        let token_header_name =
            header::HeaderName::from_bytes(SLURM_TOKEN.to_lowercase().as_bytes())?;
        let token_header_val = header::HeaderValue::from_str(&self.token)?;

        // Set default headers
        let mut headers = header::HeaderMap::new();
        headers.append(user_header_name, user_header_val);
        headers.append(token_header_name, token_header_val);
        headers.append(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        // Start building up our request
        let mut request_builder = self.client.request(method.clone(), url).headers(headers);

        // if we have query variable, add it to our Url
        match query {
            None => (),
            Some(q) => {
                request_builder = request_builder.query(&q);
            }
        }

        // Add the body if our request method is something other than
        // GET or DELETE
        if method != Method::GET && method != Method::DELETE {
            request_builder = request_builder.json(&body);
        }

        // Build it!
        Ok(request_builder.build()?)
    }
}

/// Entrypoint for interacting with the API.
/// To authenticate with the API, we need a user and a token.
pub struct SlurmDB {
    user: String,
    token: String,
    endpoint: Url,
    client: Arc<Client>,
}

impl SlurmDB {
    /// Create a new Slurm client struct. It takes any type that can convert
    /// into a &str and any type that can convert into a URL for the endpoint.
    /// Since this lib is useless withouth a client to connect with, this
    /// will panic if creating a client fails.
    pub fn new<U, T, L>(user: U, token: T, url: L) -> Self
    where
        U: ToString,
        T: ToString,
        L: ToString,
    {
        let client = Client::builder().build();
        match client {
            Ok(c) => SlurmDB {
                user: user.to_string(),
                token: token.to_string(),
                endpoint: Url::parse(&url.to_string()).expect("Unable to parse endpoint into URL!"),
                client: Arc::new(c),
            },
            Err(e) => panic!("Unable to create client: {e:?}"),
        }
    }

    /// Create a new Slurm client struct from environment variables.
    /// It takes any type that can convert into a &str.
    /// Since this lib is useless withouth a client to connect with, this
    /// will panic if creating a client fails.
    pub fn new_from_env() -> Self {
        let endpoint = env::var(SLURM_ENDPOINT).expect("{SLURM_ENDPOINT} should be set!");
        let user = env::var(SLURM_USER).expect("{SLURM_USER} should be set!");
        let token = env::var(SLURM_TOKEN).expect("{SLURM_TOKEN} should be set!");

        SlurmDB::new(user, token, endpoint)
    }

    // This will be our internal request builder.
    fn request<B>(
        &self,
        method: Method,
        path: &str,
        body: B,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<Request>
    where
        B: Serialize,
    {
        // https://slurm-endpoint/{slurm,slurmdb}/v0.0.38/{nodes, diag, etc..}
        let url = self.endpoint.join("slurmdb")?;
        let url = url.join(SLURM_API_VERSION)?;
        let url = url.join(path)?;

        // Build auth headers
        let user_header_name =
            header::HeaderName::from_bytes(SLURM_USER.to_lowercase().as_bytes())?;
        let user_header_val = header::HeaderValue::from_str(&self.user)?;
        let token_header_name =
            header::HeaderName::from_bytes(SLURM_TOKEN.to_lowercase().as_bytes())?;
        let token_header_val = header::HeaderValue::from_str(&self.token)?;

        // Set default headers
        let mut headers = header::HeaderMap::new();
        headers.append(user_header_name, user_header_val);
        headers.append(token_header_name, token_header_val);
        headers.append(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        // Start building up our request
        let mut request_builder = self.client.request(method.clone(), url).headers(headers);

        // if we have query variable, add it to our Url
        match query {
            None => (),
            Some(q) => {
                request_builder = request_builder.query(&q);
            }
        }

        // Add the body if our request method is something other than
        // GET or DELETE
        if method != Method::GET && method != Method::DELETE {
            request_builder = request_builder.json(&body);
        }

        // Build it!
        Ok(request_builder.build()?)
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Serialize)]
pub struct Ping {
    #[serde(default)]
    pub hostname: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ping: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub status: i32,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Serialize)]
pub struct Meta {
    #[serde(default)]
    pub plugin: MetaPlugin,
    #[serde(default)]
    pub slurm: MetaSlurm,
}

#[derive(Debug, Default, Clone, Deserialize, JsonSchema, Serialize)]
pub struct MetaPlugin {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    // it's `type` in the slurm api..might need to adjust this
    pub atype: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

#[derive(Debug, Default, Clone, Deserialize, JsonSchema, Serialize)]
pub struct MetaSlurm {
    #[serde(default)]
    pub version: MetaSlurmVersion,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

#[derive(Debug, Default, Clone, Deserialize, JsonSchema, Serialize)]
pub struct MetaSlurmVersion {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub major: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub micro: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub minor: String,
}
