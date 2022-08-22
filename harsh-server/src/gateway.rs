use harsh_common::{client, server, ClientRequest, ServerRequest};
use telecomande::{Processor, Remote};

use crate::{Addr, Id, SessionCmd, SessionProc, StorageCmd, StorageProc};

#[derive(Debug)]
pub enum GatewayCmd {
    Request(Addr, String),
    ClosedConnection(Addr),
}

pub struct GatewayProc {
    sessions: Remote<SessionProc>,
    storage: Remote<StorageProc>,
}

impl GatewayProc {
    async fn handle_request(&mut self, address: Addr, request: ClientRequest) {
        use client as c;
        use ClientRequest::*;
        match request {
            Ping(c::Ping { content }) => self.on_ping(content, address),

            ChannelCreate(c::ChannelCreate { name }) => self.on_channel_create(name).await,
            ChannelDelete(c::ChannelDelete { id }) => self.on_channel_delete(id),
            ChannelList(c::ChannelList {}) => self.on_channel_list(address).await,
            ChannelGetName(c::ChannelGetName { id }) => self.on_channel_get_name(id, address).await,
            ChannelSetName(c::ChannelSetName { id, name }) => self.on_channel_set_name(id, name),

            MessageList(c::MessageList { channel_id }) => {
                self.on_message_list(channel_id, address).await
            }
            MessageCreate(c::MessageCreate {
                channel_id,
                content,
            }) => self.on_message_create(channel_id, content).await,
            MessageDelete(c::MessageDelete { channel_id, id }) => {
                self.on_message_delete(channel_id, id)
            }
            MessageGetContent(c::MessageGetContent { channel_id, id }) => {
                self.on_message_get_content(channel_id, id, address).await
            }

            MessageSetContent(c::MessageSetContent {
                channel_id,
                id,
                content,
            }) => {
                self.on_message_set_content(channel_id, id, content);
            }

            // TODO: user
            UserList(c::UserList {}) => {
                todo!()
            }
            UserCreate(c::UserCreate { name, pass }) => {
                todo!()
            }
            UserDelete(c::UserDelete { id }) => {
                todo!()
            }
            UserGetName(c::UserGetName { id }) => {
                todo!()
            }
            UserSetName(c::UserSetName { id, name }) => {
                todo!()
            }
            UserSetPass(c::UserSetPass { id, pass }) => {
                todo!()
            }
        }
    }

    pub fn new(sessions: Remote<SessionProc>, storage: Remote<StorageProc>) -> Self {
        Self { sessions, storage }
    }

    fn on_ping(&mut self, content: String, address: Addr) {
        println!("[gateway/PING] '{content:?}'");
        let request = ServerRequest::Pong(server::Pong { content });
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    async fn on_channel_create(&mut self, name: String) {
        let (cmd, rec) = StorageCmd::new_channel_create(name.clone());
        self.storage.send(cmd).unwrap();
        let id = rec.await.unwrap().to_u64();
        let request = ServerRequest::new_channel_create(id, name);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    fn on_channel_delete(&mut self, id: u64) {
        let command = StorageCmd::new_channel_delete(id.into());
        self.storage.send(command).unwrap();
        let request = ServerRequest::new_channel_delete(id);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_channel_list(&mut self, address: Addr) {
        let (cmd, rec) = StorageCmd::new_channel_list();
        self.storage.send(cmd).unwrap();
        let channels = rec.await.unwrap().iter().map(|id| id.to_u64()).collect();
        let request = ServerRequest::new_channel_list(channels);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    async fn on_channel_get_name(&mut self, id: u64, address: Addr) {
        let (cmd, rec) = StorageCmd::new_channel_get_name(id.into());
        self.storage.send(cmd).unwrap();
        let name = rec.await.unwrap();
        let request = ServerRequest::new_channel_get_name(id, name);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    fn on_channel_set_name(&mut self, id: u64, name: String) {
        let command = StorageCmd::new_channel_set_name(id.into(), name.clone());
        self.storage.send(command).unwrap();
        let request = ServerRequest::new_channel_set_name(id, name);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_message_list(&mut self, channel_id: u64, address: Addr) {
        let (cmd, rec) = StorageCmd::new_message_list(channel_id.into());
        self.storage.send(cmd).unwrap();
        let messages = rec.await.unwrap().iter().map(Id::to_u64).collect();
        let request = ServerRequest::new_message_list(channel_id, messages);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    async fn on_message_create(&mut self, channel_id: u64, content: String) {
        let (cmd, rec) = StorageCmd::new_message_create(channel_id.into(), content.clone());
        self.storage.send(cmd).unwrap();
        let id = rec.await.unwrap();
        let request = ServerRequest::new_message_create(channel_id, id.to_u64(), content);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    fn on_message_delete(&mut self, channel_id: u64, id: u64) {
        let command = StorageCmd::new_message_delete(channel_id.into(), id.into());
        self.storage.send(command).unwrap();
        let request = ServerRequest::new_message_delete(channel_id, id);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_message_get_content(&mut self, channel_id: u64, id: u64, address: Addr) {
        let (cmd, rec) = StorageCmd::new_message_get_content(channel_id.into(), id.into());
        self.storage.send(cmd).unwrap();
        let request = ServerRequest::new_message_get_content(channel_id, id, rec.await.unwrap());
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    fn on_message_set_content(&mut self, channel_id: u64, id: u64, content: String) {
        let command =
            StorageCmd::new_message_set_content(channel_id.into(), id.into(), content.clone());
        self.storage.send(command).unwrap();
        let request = ServerRequest::new_message_set_content(channel_id, id, content);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }
}

#[telecomande::async_trait]
impl Processor for GatewayProc {
    type Command = GatewayCmd;
    type Error = ();
    async fn handle(&mut self, command: Self::Command) -> Result<(), Self::Error> {
        match command {
            GatewayCmd::Request(address, request) => {
                if let Some(request) = ClientRequest::try_parse(&request) {
                    println!("[session/info] received command '{request:?}'");
                    self.handle_request(address, request).await;
                } else {
                    println!("[session/warn] failed to parse command");
                }
            }
            GatewayCmd::ClosedConnection(address) => self
                .sessions
                .send(SessionCmd::RemoveSession(address))
                .unwrap(),
        }
        Ok(())
    }
}
