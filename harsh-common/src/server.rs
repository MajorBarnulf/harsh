pub struct Pong {
    pub content: String,
}

pub struct ChannelList {
    pub channels: Vec<u64>,
}

pub struct ChannelGetName {
    pub id: u64,
    pub name: Option<String>,
}
pub enum ServerRequest {
    Pong(Pong),
    ChannelList(ChannelList),
    ChannelGetName(ChannelGetName),
}

impl ServerRequest {
    pub fn new_pong(content: String) -> Self {
        Self::Pong(Pong { content })
    }

    pub fn new_channel_list(channels: Vec<u64>) -> Self {
        Self::ChannelList(ChannelList { channels })
    }

    pub fn new_channel_get_name(id: u64, name: Option<String>) -> Self {
        Self::ChannelGetName(ChannelGetName { name, id })
    }

    pub fn try_parse(line: &str) -> Option<Self> {
        use repr::Command::*;
        let command: repr::Command = serde_json::from_str(line).ok()?;
        let mapped = match command {
            pong { content } => Self::Pong(Pong { content }),
            channel_list { channels } => Self::ChannelList(ChannelList { channels }),
            channel_get_name { id, name } => Self::ChannelGetName(ChannelGetName { id, name }),
        };
        Some(mapped)
    }

    pub fn serialize(self) -> String {
        use repr::Command::*;
        let mapped = match self {
            Self::Pong(Pong { content }) => pong { content },
            Self::ChannelList(ChannelList { channels }) => channel_list { channels },
            Self::ChannelGetName(ChannelGetName { id, name }) => channel_get_name { id, name },
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
        channel_list { channels: Vec<u64> },
        channel_get_name { id: u64, name: Option<String> },
    }
}
