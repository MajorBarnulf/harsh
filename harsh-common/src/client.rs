#[derive(Debug)]
pub enum ClientRequest {
    Ping(Ping),
}

#[derive(Debug)]
pub struct Ping {
    pub content: String,
}

impl ClientRequest {
    pub fn new_ping(content: String) -> Self {
        Self::Ping(Ping { content })
    }

    pub fn try_parse(line: &str) -> Option<Self> {
        let command: repr::Command = serde_json::from_str(line).ok()?;
        let mapped = match command {
            repr::Command::ping { content } => Self::Ping(Ping { content }),
        };
        Some(mapped)
    }

    pub fn serialize(self) -> String {
        let mapped = match self {
            Self::Ping(Ping { content }) => repr::Command::ping { content },
        };
        serde_json::to_string(&mapped).unwrap()
    }
}

mod repr {
    #![allow(non_camel_case_types)]

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "type")]
    pub enum Command {
        ping { content: String },
    }
}
