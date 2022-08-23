use telecomande::{Processor, Remote};
use tokio::sync::oneshot::{self, Receiver, Sender};

use crate::{storage::Perm, Id, StorageCmd, StorageProc};

#[derive(Debug)]
pub enum SecurityCmd {
    Verify(Id, Perm, Sender<bool>),
    Authenticate(Id, String, Sender<bool>),
    StorePass(Id, String),
}

impl SecurityCmd {
    pub fn new_verify(user_id: Id, permission: Perm) -> (Self, Receiver<bool>) {
        let (sender, receiver) = oneshot::channel();
        let command = Self::Verify(user_id, permission, sender);
        (command, receiver)
    }

    pub fn new_authenticate(user_id: Id, pass: String) -> (Self, Receiver<bool>) {
        let (sender, receiver) = oneshot::channel();
        let command = Self::Authenticate(user_id, pass, sender);
        (command, receiver)
    }

    pub fn new_store_pass(user_id: Id, pass: String) -> Self {
        Self::StorePass(user_id, pass)
    }
}

pub struct SecurityProc {
    storage: Remote<StorageProc>,
}

impl SecurityProc {
    pub fn new(storage: Remote<StorageProc>) -> Self {
        Self { storage }
    }

    async fn handle_command(&mut self, command: SecurityCmd) {
        match command {
            SecurityCmd::Verify(user, perm, sender) => {
                let (cmd, req) = StorageCmd::new_perm_server_get_op();
                self.storage.send(cmd).unwrap();
                let serv_ops = req.await.unwrap();
                let is_serv_op = serv_ops.into_iter().any(|i| i == user);
                let result = match (is_serv_op, perm) {
                    (true, _) => true,
                    (false, Perm::OpChannel(chan_id)) => {
                        let (cmd, req) = StorageCmd::new_perm_channel_get_op(chan_id);
                        self.storage.send(cmd).unwrap();
                        let channel_ops = req.await.unwrap();
                        channel_ops.into_iter().any(|i| i == user)
                    }
                    _ => false,
                };
                sender.send(result).unwrap();
            }
            SecurityCmd::Authenticate(user, pass, sender) => {
                let (cmd, rec) = StorageCmd::new_user_get_pass(user);
                self.storage.send(cmd).unwrap();
                let stored = rec.await.unwrap();
                let result = stored.map(|stored| stored == hash(pass)).unwrap_or(false);
                sender.send(result).unwrap();
            }
            SecurityCmd::StorePass(user, pass) => {
                let pass = hash(pass);
                let command = StorageCmd::new_user_set_pass(user, pass);
                self.storage.send(command).unwrap();
            }
        }
    }
}

const SALT: &str = ":)";

fn hash(input: String) -> String {
    let hash = blake3::hash((input + SALT).as_bytes());
    format!("{hash}")
}

#[test]
fn test_hash() {
    assert_eq!(
        &hash("arbre".into()),
        "c2d3a87dcb76c21a8a935b8e988745f31663c3650a0d3732430eaa323f12ee0f"
    );
}

#[telecomande::async_trait]
impl Processor for SecurityProc {
    type Command = SecurityCmd;
    type Error = ();

    async fn handle(&mut self, command: Self::Command) -> Result<(), Self::Error> {
        self.handle_command(command).await;
        Ok(())
    }
}
