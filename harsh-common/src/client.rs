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
    pub id: u64,
}

#[derive(Debug)]
pub struct ChannelGetName {
    pub id: u64,
}

#[derive(Debug)]
pub struct ChannelSetName {
    pub id: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct MessageList {
    pub channel_id: u64,
}
#[derive(Debug)]
pub struct MessageCreate {
    pub channel_id: u64,
    pub content: String,
}
#[derive(Debug)]
pub struct MessageDelete {
    pub channel_id: u64,
    pub id: u64,
}
#[derive(Debug)]
pub struct MessageGetContent {
    pub channel_id: u64,
    pub id: u64,
}
#[derive(Debug)]
pub struct MessageSetContent {
    pub channel_id: u64,
    pub id: u64,
    pub content: String,
}

#[derive(Debug)]
pub struct UserList {}

#[derive(Debug)]
pub struct UserCreate {
    pub name: String,
    pub pass: String,
}

#[derive(Debug)]
pub struct UserDelete {
    pub id: u64,
}

#[derive(Debug)]
pub struct UserGetName {
    pub id: u64,
}

#[derive(Debug)]
pub struct UserSetName {
    pub id: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct UserGetPass {
    pub id: u64,
}

#[derive(Debug)]
pub struct UserSetPass {
    pub id: u64,
    pub pass: String,
}

#[derive(Debug)]
pub enum ClientRequest {
    Ping(Ping),
    ChannelList(ChannelList),
    ChannelCreate(ChannelCreate),
    ChannelDelete(ChannelDelete),
    ChannelGetName(ChannelGetName),
    ChannelSetName(ChannelSetName),

    MessageList(MessageList),
    MessageCreate(MessageCreate),
    MessageDelete(MessageDelete),
    MessageGetContent(MessageGetContent),
    MessageSetContent(MessageSetContent),

    UserList(UserList),
    UserCreate(UserCreate),
    UserDelete(UserDelete),
    UserGetName(UserGetName),
    UserSetName(UserSetName),
    UserSetPass(UserSetPass),
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
        Self::ChannelDelete(ChannelDelete { id: channel_id })
    }

    pub fn new_channel_get_name(channel_id: u64) -> Self {
        Self::ChannelGetName(ChannelGetName { id: channel_id })
    }

    pub fn new_channel_set_name(channel_id: u64, name: String) -> Self {
        Self::ChannelSetName(ChannelSetName {
            id: channel_id,
            name,
        })
    }

    pub fn new_message_list(channel_id: u64) -> Self {
        Self::MessageList(MessageList { channel_id })
    }
    pub fn new_message_create(channel_id: u64, content: String) -> Self {
        Self::MessageCreate(MessageCreate {
            channel_id,
            content,
        })
    }
    pub fn new_message_delete(channel_id: u64, id: u64) -> Self {
        Self::MessageDelete(MessageDelete { channel_id, id })
    }
    pub fn new_message_get_content(channel_id: u64, id: u64) -> Self {
        Self::MessageGetContent(MessageGetContent { channel_id, id })
    }
    pub fn new_message_set_content(channel_id: u64, id: u64, content: String) -> Self {
        Self::MessageSetContent(MessageSetContent {
            channel_id,
            id,
            content,
        })
    }

    pub fn try_parse(line: &str) -> Option<Self> {
        use repr::Command::*;
        let command: repr::Command = serde_json::from_str(line).ok()?;
        let mapped = match command {
            ping { content } => Self::Ping(Ping { content }),
            channel_list {} => Self::ChannelList(ChannelList {}),
            channel_create { name } => Self::ChannelCreate(ChannelCreate { name }),
            channel_delete { id: channel_id } => {
                Self::ChannelDelete(ChannelDelete { id: channel_id })
            }
            channel_get_name { id: channel_id } => {
                Self::ChannelGetName(ChannelGetName { id: channel_id })
            }
            channel_set_name {
                id: channel_id,
                name,
            } => Self::ChannelSetName(ChannelSetName {
                id: channel_id,
                name,
            }),
            message_list { channel_id } => Self::MessageList(MessageList { channel_id }),
            message_create {
                channel_id,
                content,
            } => Self::MessageCreate(MessageCreate {
                channel_id,
                content,
            }),
            message_delete { id, channel_id } => {
                Self::MessageDelete(MessageDelete { id, channel_id })
            }
            message_get_content { id, channel_id } => {
                Self::MessageGetContent(MessageGetContent { id, channel_id })
            }
            message_set_content {
                id,
                channel_id,
                content,
            } => Self::MessageSetContent(MessageSetContent {
                content,
                id,
                channel_id,
            }),
            user_list {} => Self::UserList(UserList {}),
            user_create { name, pass } => Self::UserCreate(UserCreate { name, pass }),
            user_delete { id } => Self::UserDelete(UserDelete { id }),
            user_get_name { id } => Self::UserGetName(UserGetName { id }),
            user_set_name { id, name } => Self::UserSetName(UserSetName { id, name }),
            user_set_pass { id, pass } => Self::UserSetPass(UserSetPass { id, pass }),
        };
        Some(mapped)
    }

    pub fn serialize(self) -> String {
        use repr::Command::*;
        let mapped = match self {
            Self::Ping(Ping { content }) => ping { content },
            Self::ChannelList(ChannelList {}) => repr::Command::channel_list {},
            Self::ChannelCreate(ChannelCreate { name }) => channel_create { name },
            Self::ChannelDelete(ChannelDelete { id: channel_id }) => {
                channel_delete { id: channel_id }
            }
            Self::ChannelGetName(ChannelGetName { id: channel_id }) => {
                channel_get_name { id: channel_id }
            }
            Self::ChannelSetName(ChannelSetName {
                id: channel_id,
                name,
            }) => channel_set_name {
                id: channel_id,
                name,
            },
            Self::MessageList(MessageList { channel_id }) => message_list { channel_id },
            Self::MessageCreate(MessageCreate {
                channel_id,
                content,
            }) => message_create {
                channel_id,
                content,
            },
            Self::MessageDelete(MessageDelete { id, channel_id }) => {
                message_delete { id, channel_id }
            }
            Self::MessageGetContent(MessageGetContent { id, channel_id }) => {
                message_get_content { id, channel_id }
            }
            Self::MessageSetContent(MessageSetContent {
                content,
                id,
                channel_id,
            }) => message_set_content {
                id,
                channel_id,
                content,
            },
            Self::UserList(UserList {}) => user_list {},
            Self::UserCreate(UserCreate { name, pass }) => user_create { name, pass },
            Self::UserDelete(UserDelete { id }) => user_delete { id },
            Self::UserGetName(UserGetName { id }) => user_get_name { id },
            Self::UserSetName(UserSetName { id, name }) => user_set_name { id, name },
            Self::UserSetPass(UserSetPass { id, pass }) => user_set_pass { id, pass },
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
        ping {
            content: String,
        },
        channel_list {},
        channel_create {
            name: String,
        },
        channel_delete {
            id: u64,
        },
        channel_get_name {
            id: u64,
        },
        channel_set_name {
            id: u64,
            name: String,
        },
        message_list {
            channel_id: u64,
        },
        message_create {
            channel_id: u64,
            content: String,
        },
        message_delete {
            channel_id: u64,
            id: u64,
        },
        message_get_content {
            channel_id: u64,
            id: u64,
        },
        message_set_content {
            channel_id: u64,
            id: u64,
            content: String,
        },
        user_list {},
        user_create {
            name: String,
            pass: String,
        },
        user_delete {
            id: u64,
        },
        user_get_name {
            id: u64,
        },
        user_set_name {
            id: u64,
            name: String,
        },
        user_set_pass {
            id: u64,
            pass: String,
        },
    }
}
