use serde::{Deserialize, Serialize};

pub mod messages;

// Messages API
const MESSAGES_CREATE: &str = "messages";

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    /// Determines the content shape.
    pub r#type: String,
    /// Response content
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self { role: self.role.clone(), content: self.content.clone() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Assistant,
    User,
}

impl Clone for Role {
    fn clone(&self) -> Self {
        match self {
            Self::Assistant => Self::Assistant,
            Self::User => Self::User,
        }
    }
}
