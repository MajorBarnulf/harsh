use std::{collections::HashMap, net::SocketAddr};

use harsh_common::ServerRequest;
use telecomande::{Processor, Remote};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    task::JoinHandle,
};

use crate::{gateway, Addr};
#[derive(Debug)]
pub enum SessionCmd {
    AddSession(TcpStream, SocketAddr, Remote<gateway::GatewayProc>),
    RemoveSession(Addr),
    Send(Addr, String),
}

impl SessionCmd {
    pub fn new_send(address: Addr, request: ServerRequest) -> Self {
        let content = request.serialize();
        Self::Send(address, content)
    }
}

#[derive(Debug, Default)]
pub struct SessionProc {
    clients: HashMap<Addr, (OwnedWriteHalf, JoinHandle<()>)>,
}

impl SessionProc {
    fn add_client(
        &mut self,
        stream: TcpStream,
        address: Addr,
        remote: Remote<gateway::GatewayProc>,
    ) {
        let (reader, writer) = stream.into_split();
        let handle = tokio::spawn(session(address.clone(), reader, remote));
        self.clients.insert(address, (writer, handle));
    }
}

#[telecomande::async_trait]
impl Processor for SessionProc {
    type Command = SessionCmd;

    type Error = ();

    async fn handle(&mut self, command: Self::Command) -> Result<(), Self::Error> {
        match command {
            SessionCmd::AddSession(stream, address, remote) => {
                let address = Addr::new(address);
                self.add_client(stream, address, remote)
            }
            SessionCmd::RemoveSession(address) => {
                self.clients.remove(&address);
            }
            SessionCmd::Send(address, content) => {
                if let Some((client, _)) = self.clients.get_mut(&address) {
                    client.write_all(content.as_bytes()).await.unwrap();
                    client.write_all(b"\n").await.unwrap();
                } else {
                    println!("failed to find session with address '{address:?}'")
                }
            }
        };
        Ok(())
    }
}

async fn session(address: Addr, reader: OwnedReadHalf, remote: Remote<gateway::GatewayProc>) {
    let mut reader = BufReader::new(reader);
    loop {
        let mut line = String::new();
        if let Err(error) = reader.read_line(&mut line).await {
            eprintln!("[session/error] {error}");
            break;
        }
        remote
            .send(gateway::GatewayCmd::Request(address.clone(), line.clone()))
            .unwrap();
    }
    remote
        .send(gateway::GatewayCmd::ClosedConnection(address))
        .unwrap();
}
