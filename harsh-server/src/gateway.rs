use harsh_common::{Ping, Pong, ServerRequest};
use telecomande::{Processor, Remote};

use harsh_common::ClientRequest;

use crate::{sessions, Addr, SessionProc, StorageProc};

#[derive(Debug)]
pub enum GatewayCmd {
    Request(Addr, String),
    ClosedConnection(Addr),
}

pub struct GatewayProc {
    client_handler: Remote<SessionProc>,
    storage: Remote<StorageProc>,
}

impl GatewayProc {
    pub fn new(client_handler: Remote<SessionProc>, storage: Remote<StorageProc>) -> Self {
        Self {
            client_handler,
            storage,
        }
    }

    async fn handle_request(&mut self, address: Addr, request: ClientRequest) {
        match request {
            ClientRequest::Ping(Ping { content }) => {
                println!("received ping! '{content:?}'");
                let response = ServerRequest::Pong(Pong { content });
                let content = response.serialize();
                self.client_handler
                    .send(sessions::SessionCmd::Send(address, content))
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
                    self.handle_request(address, request).await;
                } else {
                    println!("failed to parse command");
                }
            }
            GatewayCmd::ClosedConnection(address) => self
                .client_handler
                .send(sessions::SessionCmd::RemoveSession(address))
                .unwrap(),
        }
        Ok(())
    }
}
