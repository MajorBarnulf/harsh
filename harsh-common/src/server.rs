pub enum ServerRequest {
    Pong(Pong),
}

pub struct Pong {
    pub content: String,
}

impl ServerRequest {
    pub fn new_pong(content: String) -> Self {
        Self::Pong(Pong { content })
    }

    pub fn try_parse(line: &str) -> Option<Self> {
        let command: repr::Command = serde_json::from_str(line).ok()?;
        let mapped = match command {
            repr::Command::pong { content } => Self::Pong(Pong { content }),
        };
        Some(mapped)
    }

    pub fn serialize(self) -> String {
        let mapped = match self {
            Self::Pong(Pong { content }) => repr::Command::pong { content },
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
        pong { content: String },
    }
}
