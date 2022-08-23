use std::{collections::HashMap, net::SocketAddr};

use harsh_common::ServerEvent;
use telecomande::{Processor, Remote};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::oneshot::{self, Receiver, Sender},
    task::JoinHandle,
};

use crate::{gateway, Addr, Id};
#[derive(Debug)]
pub enum SessionCmd {
    AddSession(TcpStream, SocketAddr, Remote<gateway::GatewayProc>),
    RemoveSession(Addr),
    Send(Addr, String),
    Broadcast(String),
    GetUser(Addr, Sender<Option<Id>>),
    SetUser(Addr, Option<Id>),
}

impl SessionCmd {
    pub fn new_add_session(
        stream: TcpStream,
        address: SocketAddr,
        gateway: Remote<gateway::GatewayProc>,
    ) -> Self {
        Self::AddSession(stream, address, gateway)
    }

    pub fn new_remove_session(address: Addr) -> Self {
        Self::RemoveSession(address)
    }

    pub fn new_send(address: Addr, request: ServerEvent) -> Self {
        let content = request.serialize();
        Self::Send(address, content)
    }

    pub fn new_broadcast(request: ServerEvent) -> Self {
        let content = request.serialize();
        Self::Broadcast(content)
    }

    pub fn new_get_user(address: Addr) -> (Self, Receiver<Option<Id>>) {
        let (sender, receiver) = oneshot::channel();
        let command = Self::GetUser(address, sender);
        (command, receiver)
    }

    pub fn new_set_user(address: Addr, user: Option<Id>) -> Self {
        Self::SetUser(address, user)
    }
}

#[derive(Debug, Default)]
pub struct SessionProc {
    clients: HashMap<Addr, Client>,
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
        self.clients.insert(address, Client::new(writer, handle));
    }
}

#[telecomande::async_trait]
impl Processor for SessionProc {
    type Command = SessionCmd;

    type Error = ();

    async fn handle(&mut self, command: Self::Command) -> Result<(), Self::Error> {
        match command {
            SessionCmd::AddSession(stream, address, remote) => {
                println!("[sessions/info] new connection from '{address:?}'");
                let address = Addr::new(address);
                self.add_client(stream, address, remote)
            }
            SessionCmd::RemoveSession(address) => {
                println!("[sessions/info] closed connection from '{address:?}'");
                if let Some(client) = self.clients.remove(&address) {
                    client.unwrap().await.unwrap();
                }
            }
            SessionCmd::Send(address, content) => {
                if let Some(client) = self.clients.get_mut(&address) {
                    println!("[session/info] sending '{content}' to '{address:?}'");
                    client.send(&content).await;
                }
            }
            SessionCmd::Broadcast(content) => {
                println!("[session/info] broadcasting '{content}'");
                for client in self.clients.values_mut() {
                    client.send(&content).await;
                }
            }
            SessionCmd::GetUser(address, sender) => {
                let user = self.clients.get_mut(&address).and_then(|c| c.get_user());
                sender.send(user).unwrap();
            }
            SessionCmd::SetUser(address, user) => {
                if let Some(client) = self.clients.get_mut(&address) {
                    client.set_user(user);
                }
            }
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct Client {
    writer: OwnedWriteHalf,
    handle: JoinHandle<()>,
    user: Option<Id>,
}

impl Client {
    pub fn new(writer: OwnedWriteHalf, handle: JoinHandle<()>) -> Self {
        let user = None;
        Self {
            handle,
            user,
            writer,
        }
    }

    pub fn unwrap(self) -> JoinHandle<()> {
        let Self {
            writer,
            handle,
            user,
        } = self;
        drop((writer, user));
        handle
    }

    pub async fn send(&mut self, message: &str) {
        self.writer.write_all(message.as_bytes()).await.unwrap();
        self.writer.write_all(b"\n").await.unwrap();
    }
    pub fn set_user(&mut self, id: Option<Id>) {
        self.user = id;
    }
    pub fn get_user(&self) -> Option<Id> {
        self.user
    }
}

async fn session(address: Addr, reader: OwnedReadHalf, remote: Remote<gateway::GatewayProc>) {
    let mut reader = BufReader::new(reader);
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line).await {
            Err(error) => eprintln!("[session/error] {error}"),
            Ok(0) => break,
            _ => (),
        }
        remote
            .send(gateway::GatewayCmd::Request(address.clone(), line.clone()))
            .unwrap();
    }
    remote
        .send(gateway::GatewayCmd::ClosedConnection(address))
        .unwrap();
}

#[telecomande::async_trait]
pub trait SessionExt {
    fn send(&self, cmd: SessionCmd);

    async fn is_logged(&self, address: Addr) -> bool {
        self.get_user(address).await.is_some()
    }

    async fn get_user(&self, address: Addr) -> Option<Id> {
        let (cmd, rec) = SessionCmd::new_get_user(address);
        self.send(cmd);
        rec.await.unwrap()
    }
}

impl SessionExt for Remote<SessionProc> {
    fn send(&self, cmd: SessionCmd) {
        self.send(cmd).unwrap();
    }
}
