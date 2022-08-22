use sled::Db;
use telecomande::Processor;
use tokio::sync::oneshot::{self, Receiver, Sender};

use crate::Id;

#[derive(Debug)]
pub enum StorageCmd {
    ChannelList(Sender<Vec<Id>>),
    ChannelCreate(String, Sender<Id>),
    ChannelDelete(Id),
    ChannelGetName(Id, Sender<Option<String>>),
    ChannelSetName(Id, String),
    MessageList(Id, Sender<Vec<Id>>),
    MessageCreate(Id, String, Sender<Id>),
    MessageDelete(Id, Id),
    MessageGetContent(Id, Id, Sender<Option<String>>),
    MessageSetContent(Id, Id, String),
}

impl StorageCmd {
    pub fn new_channel_list() -> (Self, Receiver<Vec<Id>>) {
        let (s, r) = oneshot::channel();
        (Self::ChannelList(s), r)
    }

    pub fn new_channel_create(name: impl ToString) -> (Self, Receiver<Id>) {
        let (s, r) = oneshot::channel();
        (Self::ChannelCreate(name.to_string(), s), r)
    }

    pub fn new_channel_delete(id: Id) -> Self {
        Self::ChannelDelete(id)
    }

    pub fn new_channel_get_name(id: Id) -> (Self, Receiver<Option<String>>) {
        let (s, r) = oneshot::channel();
        (Self::ChannelGetName(id, s), r)
    }

    pub fn new_channel_set_name(id: Id, name: String) -> Self {
        Self::ChannelSetName(id, name)
    }

    pub fn new_message_list(channel_id: Id) -> (Self, Receiver<Vec<Id>>) {
        let (sender, receiver) = oneshot::channel();
        let cmd = Self::MessageList(channel_id, sender);
        (cmd, receiver)
    }

    pub fn new_message_create(channel_id: Id, content: String) -> (Self, Receiver<Id>) {
        let (sender, receiver) = oneshot::channel();
        let cmd = Self::MessageCreate(channel_id, content, sender);
        (cmd, receiver)
    }

    pub fn new_message_delete(channel_id: Id, id: Id) -> Self {
        Self::MessageDelete(channel_id, id)
    }

    pub fn new_message_get_content(channel_id: Id, id: Id) -> (Self, Receiver<Option<String>>) {
        let (sender, receiver) = oneshot::channel();
        let cmd = Self::MessageGetContent(channel_id, id, sender);
        (cmd, receiver)
    }

    pub fn new_message_set_content(channel_id: Id, id: Id, content: String) -> Self {
        Self::MessageSetContent(channel_id, id, content)
    }
}

pub struct StorageProc {
    base: Db,
}

impl StorageProc {
    pub fn new<S>(path: S) -> Self
    where
        S: ToString,
    {
        let path = path.to_string();
        let base = sled::open(path).unwrap();
        Self { base }
    }

    fn get<S, T>(&self, path: S) -> Option<T>
    where
        S: ToString,
        T: SerDeser,
    {
        let path = path.to_string();
        println!("[storage/info] setting entry at '{path}'");
        T::read(&self.base, path)
    }
    fn set<S, T>(&self, path: S, item: T)
    where
        S: ToString,
        T: SerDeser,
    {
        let path = path.to_string();
        println!("[storage/info] getting entry at '{path}'");
        item.write(&self.base, path)
    }

    fn list(&self, path: impl ToString) -> Vec<Id> {
        let path = path.to_string();
        println!("[storage/info] listing entries in '{path}'");
        let db = &self.base;
        list(db, path)
    }

    // firsts (x)
    // lasts (x)
    // from (id, x)
    // to (id, x)

    fn remove(&self, path: impl ToString) {
        let path = path.to_string();
        println!("[storage/info] removing entry at '{path}'");
        self.base.remove(path).unwrap();
    }

    async fn handle_command(&mut self, command: StorageCmd) {
        match command {
            StorageCmd::ChannelList(sender) => {
                self.on_channel_list(sender);
            }
            StorageCmd::ChannelCreate(name, sender) => {
                self.on_channel_create(name, sender);
            }
            StorageCmd::ChannelDelete(id) => self.on_channel_remove(id),
            StorageCmd::ChannelGetName(id, sender) => {
                self.on_channel_get_name(id, sender);
            }
            StorageCmd::ChannelSetName(id, name) => {
                self.on_channel_set_name(id, name);
            }
            // ChannelGetParent

            //
            StorageCmd::MessageList(channel_id, sender) => {
                self.on_message_list(channel_id, sender);
            }
            StorageCmd::MessageCreate(channel_id, content, sender) => {
                self.on_message_create(channel_id, content, sender);
            }
            StorageCmd::MessageDelete(channel_id, id) => {
                self.on_message_delete(channel_id, id);
            }
            StorageCmd::MessageGetContent(channel_id, id, sender) => {
                self.on_message_get_content(channel_id, id, sender);
            }
            StorageCmd::MessageSetContent(channel_id, id, content) => {
                self.on_message_set_content(channel_id, id, content);
            }
        };
    }

    fn on_message_set_content(&mut self, channel_id: Id, id: Id, content: String) {
        let path = format!("/messages/{channel_id}/{id}");
        if let Some(mut message) = self.get::<_, Message>(&path) {
            message.set_content(content);
            self.set(path, message);
        }
    }

    fn on_message_get_content(&mut self, channel_id: Id, id: Id, sender: Sender<Option<String>>) {
        let message = self.get::<_, Message>(format!("/messages/{channel_id}/{id}"));
        let content = message.map(|m| m.get_content().to_string());
        sender.send(content).unwrap()
    }

    fn on_message_delete(&mut self, channel_id: Id, id: Id) {
        self.remove(format!("/messages/{channel_id}/{id}"));
    }

    fn on_message_create(&mut self, channel_id: Id, content: String, sender: Sender<Id>) {
        let message = Message::new(content);
        let id = message.get_id();
        self.set(format!("/messages/{channel_id}/{id}"), message);
        sender.send(id).unwrap();
    }

    fn on_message_list(&mut self, channel_id: Id, sender: Sender<Vec<Id>>) {
        let items = self.list(format!("/messages/{channel_id}/"));
        sender.send(items).unwrap();
    }

    //
    // Channels
    //
    fn on_channel_list(&mut self, sender: Sender<Vec<Id>>) {
        let results = self.list("/channels/");
        sender.send(results).unwrap();
    }

    fn on_channel_create(&mut self, name: String, sender: Sender<Id>) {
        let item = Channel::new(name);
        let id = item.get_id();
        self.set(format!("/channels/{id}"), item);
        sender.send(id).unwrap();
    }

    fn on_channel_remove(&mut self, id: Id) {
        self.remove(format!("/channels/{id}"))
    }

    fn on_channel_get_name(&mut self, id: Id, sender: Sender<Option<String>>) {
        let channel = self.get::<_, Channel>(format!("/channels/{id}"));
        let name = channel.map(|channel| channel.get_name().to_string());
        sender.send(name).unwrap();
    }

    fn on_channel_set_name(&mut self, id: Id, name: String) {
        let path = format!("/channels/{id}");
        if let Some(mut channel) = self.get::<_, Channel>(&path) {
            channel.set_name(name);
            self.set(path, channel);
        }
    }

    //
    // Messages
    //
}

#[telecomande::async_trait]
impl Processor for StorageProc {
    type Command = StorageCmd;

    type Error = ();

    async fn handle(&mut self, command: Self::Command) -> Result<(), Self::Error> {
        self.handle_command(command).await;
        Ok(())
    }
}

mod models;
pub use models::{Channel, Message, SerDeser, User};

fn list(db: &Db, path: String) -> Vec<Id> {
    let len = path.len();
    db.scan_prefix(path)
        .filter_map(move |result| -> Option<Id> {
            let (key, _) = result.ok()?;
            let string = String::from_utf8(key.iter().cloned().collect()).unwrap();
            let suffix = &string[len..];
            Id::from_string(suffix)
        })
        .collect() // TODO: turn into iterator with limits
}

#[cfg(test)]
mod tests;
