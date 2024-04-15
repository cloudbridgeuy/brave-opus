use serde::{Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub subscription_token: String,
}

impl Clone for Auth {
    fn clone(&self) -> Self {
        Self { subscription_token: self.subscription_token.clone() }
    }
}

impl Auth {
    #[must_use]
    pub fn new(subscription_token: &str) -> Self {
        Self { subscription_token: subscription_token.to_string() }
    }

    /// # Errors
    ///
    /// Will return `Err` if the environment variable `BRAVE_SUBSCRIPTION_TOKEN` is not defined.
    pub fn from_env() -> Result<Self, String> {
        let subscription_token = std::env::var("BRAVE_SUBSCRIPTION_TOKEN")
            .map_err(|_| "Missing BRAVE_SUBSCRIPTION_TOKEN".to_string())?;
        Ok(Self { subscription_token })
    }
}

#[derive(Debug)]
pub struct Brave {
    pub auth: Auth,
    pub api_url: String,
    pub(crate) agent: Agent,
}

impl Clone for Brave {
    fn clone(&self) -> Self {
        Self { auth: self.auth.clone(), api_url: self.api_url.clone(), agent: self.agent.clone() }
    }
}

impl Brave {
    #[must_use]
    pub fn new(auth: Auth, api_url: &str) -> Self {
        Self { auth, api_url: api_url.to_string(), agent: AgentBuilder::new().build() }
    }
}
