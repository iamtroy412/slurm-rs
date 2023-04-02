//! A rust library for interacting with the Slurm REST API.
//!
//! For more information, the Slurm REST API is documented at
//! <https://slurm.schedmd.com/rest_api.html>
use reqwest::Client;
use std::{env, sync::Arc};

/// Entrypoint for interacting with the API.
/// To authenticate with the API, we need a user and a token.
pub struct Slurm {
    user: String,
    token: String,
    client: Arc<Client>,
}

impl Slurm {
    /// Create a new Slurm client struct. It takes any type that can convert
    /// into a &str.
    /// Since this lib is useless withouth a client to connect with, this
    /// will panic if creating a client fails.
    pub fn new<U, T>(user: U, token: T) -> Self
    where
        U: ToString,
        T: ToString,
    {
        let client = Client::builder().build();
        match client {
            Ok(c) => Slurm {
                user: user.to_string(),
                token: token.to_string(),
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
        let user = env::var("X-SLURM-USER-NAME").expect("X-SLURM-USER-NAME should be set!");
        let token = env::var("X-SLURM-USER-TOKEN").expect("X-SLURM-USER-TOKEN should be set!");

        Slurm::new(user, token)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
