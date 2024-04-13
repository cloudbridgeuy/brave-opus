use serde::{Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub api_key: String,
    pub version: Option<String>,
}

impl Clone for Auth {
    fn clone(&self) -> Self {
        Self { api_key: self.api_key.clone(), version: self.version.clone() }
    }
}

impl Auth {
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        Self { api_key: api_key.to_string(), version: None }
    }

    /// # Errors
    ///
    /// Will return `Err` if the environment variable `ANTHROPIC_API_KEY` is not defined.
    pub fn from_env() -> Result<Self, String> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| "Missing ANTHROPIC_API_KEY".to_string())?;
        let version = std::env::var("ANTHROPIC_API_VERSION").ok();
        Ok(Self { api_key, version })
    }
}

#[derive(Debug)]
pub struct Anthropic {
    pub auth: Auth,
    pub api_url: String,
    pub(crate) agent: Agent,
}

impl Clone for Anthropic {
    fn clone(&self) -> Self {
        Self { auth: self.auth.clone(), api_url: self.api_url.clone(), agent: self.agent.clone() }
    }
}

impl Anthropic {
    #[must_use]
    pub fn new(auth: Auth, api_url: &str) -> Self {
        Self { auth, api_url: api_url.to_string(), agent: AgentBuilder::new().build() }
    }
}
