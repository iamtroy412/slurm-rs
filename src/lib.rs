//! A rust library for interacting with the Slurm REST API.
//!
//! For more information, the Slurm REST API is documented at
//! <https://slurm.schedmd.com/rest_api.html>
use anyhow::{bail, Result};
use reqwest::{header, Client, Method, Request, StatusCode, Url};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

const SLURM_USER: &str = "X-SLURM-USER-NAME";
const SLURM_TOKEN: &str = "X-SLURM-USER-TOKEN";
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
        let endpoint =
            env::var("X_SLURM_ENDPOINT").expect("env variable 'X_SLURM_ENDPOINT' should be set");
        let user =
            env::var("X_SLURM_USER_NAME").expect("env variable 'X_SLURM_USER_NAME' should be set");
        let token = env::var("X_SLURM_USER_TOKEN")
            .expect("env variable 'X_SLURM_USER_TOKEN' should be set");

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
        let url_path = format!("slurm/{}/{}", SLURM_API_VERSION, path);
        let url = self.endpoint.join(&url_path)?;

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

    /// Ping test!
    /// SEE: <https://slurm.schedmd.com/rest_api.html#slurmV0038Ping>
    pub async fn ping(&self) -> Result<Pings> {
        let request = self.request(Method::GET, "ping", (), None)?;

        let response = self.client.execute(request).await?;
        match response.status() {
            StatusCode::OK => (),
            status => {
                bail!("status code: {}, body: {}", status, response.text().await?);
            }
        };

        let r: Pings = response.json().await?;
        Ok(r)
    }

    /// Get all parition information
    /// SEE: <https://slurm.schedmd.com/rest_api.html#slurmV0038GetPartitions>
    pub async fn get_partitions(&self) -> Result<PartitionsResponse> {
        let request = self.request(Method::GET, "partitions", (), None)?;

        let response = self.client.execute(request).await?;
        match response.status() {
            StatusCode::OK => (),
            status => {
                bail!("status code: {}, body: {}", status, response.text().await?);
            }
        };

        let r: PartitionsResponse = response.json().await?;
        Ok(r)
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

    /// Create a new SlurmDB client struct from environment variables.
    /// It takes any type that can convert into a &str.
    /// Since this lib is useless withouth a client to connect with, this
    /// will panic if creating a client fails.
    pub fn new_from_env() -> Self {
        let endpoint =
            env::var("X_SLURM_ENDPOINT").expect("env variable 'X_SLURM_ENDPOINT' should be set");
        let user =
            env::var("X_SLURM_USER_NAME").expect("env variable 'X_SLURM_USER_NAME' should be set");
        let token = env::var("X_SLURM_USER_TOKEN")
            .expect("env variable 'X_SLURM_USER_TOKEN' should be set");

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
        let url_path = format!("slurm/{}/{}", SLURM_API_VERSION, path);
        let url = self.endpoint.join(&url_path)?;

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
pub struct PartitionsResponse {
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub errors: Vec<Error>,
    #[serde(default)]
    pub partitions: Vec<Partition>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Serialize)]
pub struct Partition {
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub preemption_mode: Vec<String>,
    #[serde(default)]
    pub allowed_allocation_nodes: String,
    #[serde(default)]
    pub allowed_accounts: String,
    #[serde(default)]
    pub allowed_groups: String,
    #[serde(default)]
    pub allowed_qos: String,
    #[serde(default)]
    pub alternative: String,
    #[serde(default)]
    pub billing_weights: String,
    #[serde(default)]
    pub default_memory_per_cpu: Option<i64>,
    #[serde(default)]
    pub default_time_limit: Option<i64>,
    #[serde(default)]
    pub denied_accounts: String,
    #[serde(default)]
    pub denied_qos: String,
    #[serde(default)]
    pub preemption_grace_time: Option<i64>,
    #[serde(default)]
    pub maximum_cpus_per_node: u64,
    #[serde(default)]
    pub maximum_memory_per_node: Option<i64>,
    #[serde(default)]
    pub maximum_nodes_per_job: u64,
    #[serde(default)]
    pub max_time_limit: Option<i64>,
    #[serde(default)]
    pub min_nodes_per_job: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nodes: String,
    #[serde(default)]
    pub over_time_limit: u64,
    #[serde(default)]
    pub priority_job_factor: u64,
    #[serde(default)]
    pub priority_tier: u64,
    #[serde(default)]
    pub qos: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub total_cpus: u64,
    #[serde(default)]
    pub total_nodes: u64,
    #[serde(default)]
    pub tres: String,
    #[serde(default)]
    pub maximum_memory_per_cpu: Option<i64>,
    #[serde(default)]
    pub default_memory_per_node: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Serialize)]
pub struct Pings {
    #[serde(default)]
    pub meta: Meta,
    #[serde(default)]
    pub errors: Vec<Error>,
    #[serde(default)]
    pub pings: Vec<Ping>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Serialize)]
pub struct Ping {
    #[serde(default)]
    pub hostname: String,
    #[serde(default)]
    pub ping: String,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub status: i32,
}

#[derive(Debug, Default, Clone, Deserialize, JsonSchema, Serialize)]
pub struct Meta {
    #[serde(default)]
    pub plugin: MetaPlugin,
    #[serde(default, rename = "Slurm")]
    pub slurm: MetaSlurm,
}

#[derive(Debug, Default, Clone, Deserialize, JsonSchema, Serialize)]
pub struct MetaPlugin {
    #[serde(default, rename = "type")]
    pub plugin_type: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Default, Clone, Deserialize, JsonSchema, Serialize)]
pub struct MetaSlurm {
    #[serde(default)]
    pub version: MetaSlurmVersion,
    #[serde(default)]
    pub release: String,
}

#[derive(Debug, Default, Clone, Deserialize, JsonSchema, Serialize)]
pub struct MetaSlurmVersion {
    #[serde(default)]
    pub major: i32,
    #[serde(default)]
    pub micro: i32,
    #[serde(default)]
    pub minor: i32,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, Serialize)]
pub struct Error {
    #[serde(default)]
    pub error: String,
    #[serde(default)]
    pub error_number: i32,
}
