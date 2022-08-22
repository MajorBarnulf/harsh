#[derive(Debug)]
pub struct Ping {
    pub content: String,
}

#[derive(Debug)]
pub struct ChannelList {}

#[derive(Debug)]
pub struct ChannelCreate {
    pub name: String,
}

#[derive(Debug)]
pub struct ChannelDelete {
    pub channel_id: u64,
}

#[derive(Debug)]
pub struct ChannelGetName {
    pub channel_id: u64,
}

#[derive(Debug)]
pub enum ClientRequest {
    Ping(Ping),
    ChannelList(ChannelList),
    ChannelCreate(ChannelCreate),
    ChannelDelete(ChannelDelete),
    ChannelGetName(ChannelGetName),
}

impl ClientRequest {
    pub fn new_ping(content: String) -> Self {
        Self::Ping(Ping { content })
    }

    pub fn new_channel_list() -> Self {
        Self::ChannelList(ChannelList {})
    }

    pub fn new_channel_create(name: String) -> Self {
        Self::ChannelCreate(ChannelCreate { name })
    }

    pub fn new_channel_delete(channel_id: u64) -> Self {
        Self::ChannelDelete(ChannelDelete { channel_id })
    }

    pub fn new_channel_get_name(channel_id: u64) -> Self {
        Self::ChannelGetName(ChannelGetName { channel_id })
    }

    pub fn try_parse(line: &str) -> Option<Self> {
        use repr::Command::*;
        let command: repr::Command = serde_json::from_str(line).ok()?;
        let mapped = match command {
            ping { content } => Self::Ping(Ping { content }),
            channel_list {} => Self::ChannelList(ChannelList {}),
            channel_create { name } => Self::ChannelCreate(ChannelCreate { name }),
            channel_delete { channel_id } => Self::ChannelDelete(ChannelDelete { channel_id }),
            channel_get_name { channel_id } => Self::ChannelGetName(ChannelGetName { channel_id }),
        };
        Some(mapped)
    }

    pub fn serialize(self) -> String {
        use repr::Command::*;
        let mapped = match self {
            Self::Ping(Ping { content }) => ping { content },
            Self::ChannelList(ChannelList {}) => repr::Command::channel_list {},
            Self::ChannelCreate(ChannelCreate { name }) => channel_create { name },
            Self::ChannelDelete(ChannelDelete { channel_id }) => channel_delete { channel_id },
            Self::ChannelGetName(ChannelGetName { channel_id }) => channel_get_name { channel_id },
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
        channel_list {},
        channel_create { name: String },
        channel_delete { channel_id: u64 },
        channel_get_name { channel_id: u64 },
    }
}
