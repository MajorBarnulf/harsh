#[derive(Debug)]
pub struct Pong {
    pub content: String,
}

#[derive(Debug)]
pub struct ChannelList {
    pub channels: Vec<u64>,
}

#[derive(Debug)]
pub struct ChannelGetName {
    pub id: u64,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct ChannelCreate {
    pub id: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct ChannelDelete {
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
    pub messages: Vec<u64>,
}
#[derive(Debug)]
pub struct MessageCreate {
    pub channel_id: u64,
    pub id: u64,
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
    pub content: Option<String>,
}
#[derive(Debug)]
pub struct MessageSetContent {
    pub channel_id: u64,
    pub id: u64,
    pub content: String,
}

#[derive(Debug)]
pub struct UserList {
    pub users: Vec<u64>,
}

#[derive(Debug)]
pub struct UserCreate {
    pub id: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct UserDelete {
    pub id: u64,
}

#[derive(Debug)]
pub struct UserGetName {
    pub id: u64,
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct UserSetName {
    pub id: u64,
    pub name: String,
}

#[derive(Debug)]
pub struct UserSetPass {
    pub id: u64,
}

#[derive(Debug)]
pub enum ServerRequest {
    Pong(Pong),

    ChannelCreate(ChannelCreate),
    ChannelDelete(ChannelDelete),
    ChannelList(ChannelList),
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

    pub fn new_channel_create(id: u64, name: String) -> Self {
        Self::ChannelCreate(ChannelCreate { id, name })
    }

    pub fn new_channel_delete(id: u64) -> Self {
        Self::ChannelDelete(ChannelDelete { id })
    }

    pub fn new_channel_set_name(id: u64, name: String) -> Self {
        Self::ChannelSetName(ChannelSetName { id, name })
    }

    pub fn new_message_list(channel_id: u64, messages: Vec<u64>) -> Self {
        Self::MessageList(MessageList {
            channel_id,
            messages,
        })
    }

    pub fn new_message_create(channel_id: u64, id: u64, content: String) -> Self {
        Self::MessageCreate(MessageCreate {
            channel_id,
            content,
            id,
        })
    }
    pub fn new_message_delete(channel_id: u64, id: u64) -> Self {
        Self::MessageDelete(MessageDelete { channel_id, id })
    }

    pub fn new_message_get_content(channel_id: u64, id: u64, content: Option<String>) -> Self {
        Self::MessageGetContent(MessageGetContent {
            channel_id,
            content,
            id,
        })
    }

    pub fn new_message_set_content(channel_id: u64, id: u64, content: String) -> Self {
        Self::MessageSetContent(MessageSetContent {
            channel_id,
            content,
            id,
        })
    }

    pub fn new_user_list(users: Vec<u64>) -> Self {
        Self::UserList(UserList { users })
    }

    pub fn new_user_create(id: u64, name: String) -> Self {
        Self::UserCreate(UserCreate { id, name })
    }

    pub fn new_user_delete(id: u64) -> Self {
        Self::UserDelete(UserDelete { id })
    }

    pub fn new_user_get_name(id: u64, name: Option<String>) -> Self {
        Self::UserGetName(UserGetName { id, name })
    }

    pub fn new_user_set_name(id: u64, name: String) -> Self {
        Self::UserSetName(UserSetName { id, name })
    }

    pub fn new_user_set_pass(id: u64) -> Self {
        Self::UserSetPass(UserSetPass { id })
    }

    pub fn try_parse(line: &str) -> Option<Self> {
        use repr::Command::*;
        let command: repr::Command = serde_json::from_str(line).ok()?;
        let mapped = match command {
            pong { content } => Self::Pong(Pong { content }),
            channel_list { channels } => Self::ChannelList(ChannelList { channels }),
            channel_get_name { id, name } => Self::ChannelGetName(ChannelGetName { id, name }),
            channel_create { id, name } => Self::ChannelCreate(ChannelCreate { id, name }),
            channel_set_name { id, name } => Self::ChannelSetName(ChannelSetName { id, name }),
            channel_delete { id } => Self::ChannelDelete(ChannelDelete { id }),
            message_list {
                channel_id,
                messages,
            } => Self::MessageList(MessageList {
                channel_id,
                messages,
            }),
            message_create {
                channel_id,
                id,
                content,
            } => Self::MessageCreate(MessageCreate {
                channel_id,
                content,
                id,
            }),
            message_delete { channel_id, id } => {
                Self::MessageDelete(MessageDelete { channel_id, id })
            }
            message_get_content {
                channel_id,
                id,
                content,
            } => Self::MessageGetContent(MessageGetContent {
                channel_id,
                content,
                id,
            }),
            message_set_content {
                channel_id,
                id,
                content,
            } => Self::MessageSetContent(MessageSetContent {
                channel_id,
                content,
                id,
            }),
            user_list { users } => Self::UserList(UserList { users }),
            user_create { id, name } => Self::UserCreate(UserCreate { id, name }),
            user_delete { id } => Self::UserDelete(UserDelete { id }),
            user_get_name { id, name } => Self::UserGetName(UserGetName { id, name }),
            user_set_name { id, name } => Self::UserSetName(UserSetName { id, name }),
            user_set_pass { id } => Self::UserSetPass(UserSetPass { id }),
        };
        Some(mapped)
    }

    pub fn serialize(self) -> String {
        use repr::Command::*;
        let mapped = match self {
            Self::Pong(Pong { content }) => pong { content },
            Self::ChannelList(ChannelList { channels }) => channel_list { channels },
            Self::ChannelGetName(ChannelGetName { id, name }) => channel_get_name { id, name },
            Self::ChannelCreate(ChannelCreate { id, name }) => channel_create { id, name },
            Self::ChannelSetName(ChannelSetName { id, name }) => channel_set_name { id, name },
            Self::ChannelDelete(ChannelDelete { id }) => channel_delete { id },

            Self::MessageList(MessageList {
                channel_id,
                messages,
            }) => message_list {
                channel_id,
                messages,
            },
            Self::MessageCreate(MessageCreate {
                channel_id,
                content,
                id,
            }) => message_create {
                channel_id,
                id,
                content,
            },
            Self::MessageDelete(MessageDelete { channel_id, id }) => {
                message_delete { channel_id, id }
            }
            Self::MessageGetContent(MessageGetContent {
                channel_id,
                content,
                id,
            }) => message_get_content {
                channel_id,
                id,
                content,
            },
            Self::MessageSetContent(MessageSetContent {
                channel_id,
                content,
                id,
            }) => message_set_content {
                channel_id,
                id,
                content,
            },
            Self::UserList(UserList { users }) => user_list { users },
            Self::UserCreate(UserCreate { id, name }) => user_create { id, name },
            Self::UserDelete(UserDelete { id }) => user_delete { id },
            Self::UserGetName(UserGetName { id, name }) => user_get_name { id, name },
            Self::UserSetName(UserSetName { id, name }) => user_set_name { id, name },
            Self::UserSetPass(UserSetPass { id }) => user_set_pass { id },
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
        pong {
            content: String,
        },
        channel_list {
            channels: Vec<u64>,
        },
        channel_get_name {
            id: u64,
            name: Option<String>,
        },
        channel_create {
            id: u64,
            name: String,
        },
        channel_delete {
            id: u64,
        },
        channel_set_name {
            id: u64,
            name: String,
        },
        message_list {
            channel_id: u64,
            messages: Vec<u64>,
        },
        message_create {
            channel_id: u64,
            id: u64,
            content: String,
        },
        message_delete {
            channel_id: u64,
            id: u64,
        },
        message_get_content {
            channel_id: u64,
            id: u64,
            content: Option<String>,
        },
        message_set_content {
            channel_id: u64,
            id: u64,
            content: String,
        },
        user_list {
            users: Vec<u64>,
        },
        user_create {
            id: u64,
            name: String,
        },
        user_delete {
            id: u64,
        },
        user_get_name {
            id: u64,
            name: Option<String>,
        },
        user_set_name {
            id: u64,
            name: String,
        },
        user_set_pass {
            id: u64,
        },
    }
}
