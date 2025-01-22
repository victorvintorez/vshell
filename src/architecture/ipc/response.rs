use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case")]
pub enum Response {
    Ok { message: Option<String> },
    Error { message: Option<String> },
}

impl Response {
    pub fn error(message: &str) -> Self {
        Self::Error {
            message: Some(message.to_string()),
        }
    }
}
