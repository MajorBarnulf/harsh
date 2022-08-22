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
    pub fn new(sessions: Remote<SessionProc>, storage: Remote<StorageProc>) -> Self {
        Self { sessions, storage }
    }

    async fn handle_request(&mut self, address: Addr, request: ClientRequest) {
        match request {
            ClientRequest::Ping(client::Ping { content }) => {
                println!("received ping! '{content:?}'");
                let request = ServerRequest::Pong(server::Pong { content });
                self.sessions
                    .send(SessionCmd::new_send(address, request))
                    .unwrap();
            }
            ClientRequest::ChannelList(client::ChannelList {}) => {
                let (cmd, rec) = StorageCmd::new_channel_list();
                self.storage.send(cmd).unwrap();
                let channels = rec.await.unwrap().iter().map(|id| id.to_u64()).collect();
                let request = ServerRequest::new_channel_list(channels);
                self.sessions
                    .send(SessionCmd::new_send(address, request))
                    .unwrap();
            }
            ClientRequest::ChannelCreate(client::ChannelCreate { name }) => {
                let (cmd, rec) = StorageCmd::new_channel_create(name);
                let _id = rec.await.unwrap();
                self.storage.send(cmd).unwrap();
            }
            ClientRequest::ChannelDelete(client::ChannelDelete { channel_id }) => {
                self.storage
                    .send(StorageCmd::ChannelDelete(Id::from_u64(channel_id)))
                    .unwrap();
            }
            ClientRequest::ChannelGetName(client::ChannelGetName { channel_id }) => {
                let (cmd, rec) = StorageCmd::new_channel_get_name(Id::from_u64(channel_id));
                self.storage.send(cmd).unwrap();
                let name = rec.await.unwrap();
                let request = ServerRequest::new_channel_get_name(channel_id, name);
                self.sessions
                    .send(SessionCmd::new_send(address, request))
                    .unwrap();
            }
        }
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
