use harsh_common::{client, server, ClientRequest, ServerEvent};
use telecomande::{Processor, Remote};

use crate::{
    sessions::SessionExt, storage::Perm, Addr, Id, SecurityCmd, SecurityProc, SessionCmd,
    SessionProc, StorageCmd, StorageProc,
};

#[derive(Debug)]
pub enum GatewayCmd {
    Request(Addr, String),
    ClosedConnection(Addr),
}

pub struct GatewayProc {
    sessions: Remote<SessionProc>,
    storage: Remote<StorageProc>,
    security: Remote<SecurityProc>,
}

use client::*;

impl GatewayProc {
    async fn handle_request(&mut self, addr: Addr, request: ClientRequest) -> Result<(), String> {
        use client::*;
        use ClientRequest as CR;

        // auth-free API
        let request = match request {
            CR::Ping(ping) => return self.on_ping(ping, addr),
            CR::Authenticate(authenticate) => {
                return self.on_authenticate(authenticate, addr).await
            }
            _ => request,
        };

        // let user = self.sessions.get_user(addr.clone()).await.ok_or("owno")?;

        // auth API
        match request {
            CR::Ping(_) | CR::Authenticate(_) => unreachable!(),

            CR::ChannelCreate(req) => self.on_channel_create(req).await,
            CR::ChannelDelete(req) => todo!(), //self.on_channel_delete(req, user).await,
            CR::ChannelList(req) => self.on_channel_list(req, addr).await,
            CR::ChannelGetName(req) => self.on_channel_get_name(req, addr).await,
            CR::ChannelSetName(req) => self.on_channel_set_name(req),

            CR::MessageList(req) => self.on_message_list(req, addr).await,
            CR::MessageCreate(req) => self.on_message_create(req).await,
            CR::MessageDelete(req) => self.on_message_delete(req),
            CR::MessageGetContent(req) => self.on_message_get_content(req, addr).await,
            CR::MessageSetContent(req) => self.on_message_set_content(req),

            CR::UserList(req) => self.on_user_list(req, addr).await,
            CR::UserCreate(req) => self.on_user_create(req).await,
            CR::UserDelete(req) => self.on_user_delete(req),
            CR::UserGetName(req) => self.on_user_get_name(req, addr).await,
            CR::UserSetName(req) => self.on_user_set_name(req),
            CR::UserSetPass(req) => self.on_user_set_pass(req, addr),
        };
        Ok(())
    }

    async fn on_authenticate(
        &mut self,
        Authenticate { id, pass }: Authenticate,
        address: Addr,
    ) -> Result<(), String> {
        let (cmd, rec) = SecurityCmd::new_authenticate(id.into(), pass);
        self.security.send(cmd).unwrap();
        if rec.await.unwrap() {
            let command = SessionCmd::new_set_user(address, Some(id.into()));
            self.sessions.send(command).unwrap();
        } else {
            Err("Invalid password")?;
        };
        Ok(())
    }

    pub fn new(
        sessions: Remote<SessionProc>,
        storage: Remote<StorageProc>,
        security: Remote<SecurityProc>,
    ) -> Self {
        Self {
            sessions,
            storage,
            security,
        }
    }

    fn on_ping(&mut self, Ping { content }: Ping, address: Addr) -> Result<(), String> {
        println!("[gateway/PING] '{content:?}'");
        let request = ServerEvent::Pong(server::Pong { content });
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
        Ok(())
    }

    async fn on_channel_create(&mut self, ChannelCreate { name }: ChannelCreate) {
        let (cmd, rec) = StorageCmd::new_channel_create(name.clone());
        self.storage.send(cmd).unwrap();
        let id = rec.await.unwrap().to_u64();
        let request = ServerEvent::new_channel_create(id, name);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_channel_delete(&mut self, ChannelDelete { id }: ChannelDelete, user: Id) {
        // TODO: verify is OP
        let (cmd, req) = SecurityCmd::new_verify(user, Perm::OpChannel(id.into()));
        self.security.send(cmd).unwrap();
        req.await.unwrap();
        let command = StorageCmd::new_channel_delete(id.into());
        self.storage.send(command).unwrap();
        let request = ServerEvent::new_channel_delete(id);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_channel_list(&mut self, _: ChannelList, address: Addr) {
        let (cmd, rec) = StorageCmd::new_channel_list();
        self.storage.send(cmd).unwrap();
        let channels = rec.await.unwrap().iter().map(|id| id.to_u64()).collect();
        let request = ServerEvent::new_channel_list(channels);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    async fn on_channel_get_name(&mut self, ChannelGetName { id }: ChannelGetName, address: Addr) {
        let (cmd, rec) = StorageCmd::new_channel_get_name(id.into());
        self.storage.send(cmd).unwrap();
        let name = rec.await.unwrap();
        let request = ServerEvent::new_channel_get_name(id, name);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    fn on_channel_set_name(&mut self, ChannelSetName { id, name }: ChannelSetName) {
        let command = StorageCmd::new_channel_set_name(id.into(), name.clone());
        self.storage.send(command).unwrap();
        let request = ServerEvent::new_channel_set_name(id, name);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_message_list(&mut self, MessageList { channel_id }: MessageList, address: Addr) {
        let (cmd, rec) = StorageCmd::new_message_list(channel_id.into());
        self.storage.send(cmd).unwrap();
        let messages = rec.await.unwrap().iter().map(Id::to_u64).collect();
        let request = ServerEvent::new_message_list(channel_id, messages);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    async fn on_message_create(
        &mut self,
        MessageCreate {
            channel_id,
            content,
        }: MessageCreate,
    ) {
        let (cmd, rec) = StorageCmd::new_message_create(channel_id.into(), content.clone());
        self.storage.send(cmd).unwrap();
        let id = rec.await.unwrap();
        let request = ServerEvent::new_message_create(channel_id, id.to_u64(), content);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    fn on_message_delete(&mut self, MessageDelete { channel_id, id }: MessageDelete) {
        let command = StorageCmd::new_message_delete(channel_id.into(), id.into());
        self.storage.send(command).unwrap();
        let request = ServerEvent::new_message_delete(channel_id, id);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_message_get_content(
        &mut self,
        MessageGetContent { channel_id, id }: MessageGetContent,
        address: Addr,
    ) {
        let (cmd, rec) = StorageCmd::new_message_get_content(channel_id.into(), id.into());
        self.storage.send(cmd).unwrap();
        let request = ServerEvent::new_message_get_content(channel_id, id, rec.await.unwrap());
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    fn on_message_set_content(
        &mut self,
        MessageSetContent {
            channel_id,
            id,
            content,
        }: MessageSetContent,
    ) {
        let command =
            StorageCmd::new_message_set_content(channel_id.into(), id.into(), content.clone());
        self.storage.send(command).unwrap();
        let request = ServerEvent::new_message_set_content(channel_id, id, content);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    fn on_user_set_pass(&mut self, UserSetPass { id, pass }: UserSetPass, address: Addr) {
        let command = StorageCmd::new_user_set_pass(id.into(), pass);
        self.storage.send(command).unwrap();
        let request = ServerEvent::new_user_set_pass(id);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    fn on_user_set_name(&mut self, UserSetName { id, name }: UserSetName) {
        let command = StorageCmd::new_user_set_name(id.into(), name.clone());
        self.storage.send(command).unwrap();
        let request = ServerEvent::new_user_set_name(id, name);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_user_get_name(&mut self, UserGetName { id }: UserGetName, address: Addr) {
        let (cmd, rec) = StorageCmd::new_user_get_name(id.into());
        self.storage.send(cmd).unwrap();
        let name = rec.await.unwrap();
        let request = ServerEvent::new_user_get_name(id, name);
        let command = SessionCmd::new_send(address, request);
        self.sessions.send(command).unwrap();
    }

    fn on_user_delete(&mut self, UserDelete { id }: UserDelete) {
        let command = StorageCmd::new_user_delete(id.into());
        self.storage.send(command).unwrap();
        let request = ServerEvent::new_user_delete(id);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_user_create(&mut self, UserCreate { name, pass }: UserCreate) {
        let (cmd, rec) = StorageCmd::new_user_create(name.clone(), pass);
        self.storage.send(cmd).unwrap();
        let id = rec.await.unwrap();
        let request = ServerEvent::new_user_create(id.into(), name);
        let command = SessionCmd::new_broadcast(request);
        self.sessions.send(command).unwrap();
    }

    async fn on_user_list(&mut self, _: UserList, address: Addr) {
        let (cmd, rec) = StorageCmd::new_user_list();
        self.storage.send(cmd).unwrap();
        let result = rec.await.unwrap().iter().map(Id::to_u64).collect();
        let request = ServerEvent::new_user_list(result);
        let command = SessionCmd::new_send(address, request);
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
                    if let Err(reason) = self.handle_request(address, request).await {
                        eprintln!("[gateway/warn] exception '{reason}'");
                    }
                } else {
                    println!("[session/info] failed to parse command");
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
