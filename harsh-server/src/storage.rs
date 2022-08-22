use sled::Db;
use telecomande::Processor;
use tokio::sync::oneshot::{self, Receiver, Sender};

use crate::Id;

#[derive(Debug)]
pub enum StorageCmd {
    ChannelCreate(String, Sender<Id>),
    ChannelDelete(Id),
    ChannelList(Sender<Vec<Id>>),
    ChannelGetName(Id, Sender<Option<String>>),
}

impl StorageCmd {
    pub fn new_channel_create(name: impl ToString) -> (Self, Receiver<Id>) {
        let (s, r) = oneshot::channel();
        (Self::ChannelCreate(name.to_string(), s), r)
    }
    pub fn new_channel_delete(id: Id) -> Self {
        Self::ChannelDelete(id)
    }
    pub fn new_channel_list() -> (Self, Receiver<Vec<Id>>) {
        let (s, r) = oneshot::channel();
        (Self::ChannelList(s), r)
    }
    pub fn new_channel_get_name(id: Id) -> (Self, Receiver<Option<String>>) {
        let (s, r) = oneshot::channel();
        (Self::ChannelGetName(id, s), r)
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
        T::read(&self.base, path)
    }
    fn set<S, T>(&self, path: S, item: T)
    where
        S: ToString,
        T: SerDeser,
    {
        let path = path.to_string();
        item.write(&self.base, path)
    }

    fn list(&self, path: impl ToString) -> Vec<Id> {
        let path = path.to_string();
        list(&self.base, path).collect() // TODO: turn into iterator with limits
    }

    // firsts (x)
    // lasts (x)
    // from (id, x)
    // to (id, x)

    fn remove(&self, path: impl ToString) {
        let path = path.to_string();
        remove(&self.base, path)
    }
}

#[telecomande::async_trait]
impl Processor for StorageProc {
    type Command = StorageCmd;

    type Error = ();

    async fn handle(&mut self, command: Self::Command) -> Result<(), Self::Error> {
        match command {
            // channels
            StorageCmd::ChannelDelete(id) => self.remove(format!("/channels/{id}")),
            StorageCmd::ChannelCreate(name, sender) => {
                let item = Channel::new(name);
                let id = item.get_id();
                self.set(format!("/channels/{id}"), item);
                sender.send(id).unwrap();
            }
            StorageCmd::ChannelList(sender) => {
                let results = self.list("/channels/");
                sender.send(results).unwrap();
            }
            StorageCmd::ChannelGetName(id, sender) => {
                let result = self
                    .get::<_, Channel>(format!("/channels/{id}"))
                    .map(|channel| channel.get_name().to_string());
                sender.send(result).unwrap();
            } //
              // ChannelGetParent

              // messages
              // c
              // d
              // l
              // gcontent
        };

        Ok(())
    }
}

mod models;
pub use models::{Channel, Msg, SerDeser, User};

fn list(db: &Db, path: impl ToString) -> impl Iterator<Item = Id> {
    let path = path.to_string();
    let len = path.len();
    db.scan_prefix(path)
        .filter_map(move |result| -> Option<Id> {
            let (key, _) = result.ok()?;
            let string = String::from_utf8(key.iter().cloned().collect()).unwrap();
            let suffix = &string[len..];
            Id::from_string(suffix)
        })
}

fn remove(db: &Db, path: impl ToString) {
    let path = path.to_string();
    db.remove(path).unwrap();
}

#[test]
fn test_list() {
    let db = sled::open("/tmp/test-db").unwrap();
    db.insert("/some/path/123", b"hello1").unwrap();
    db.insert("/some/path/1234", b"hello2").unwrap();
    db.insert("/some/path/12345", b"hello3").unwrap();
    let results = list(&db, "/some/path/".to_string());
    let vec = results.collect::<Vec<_>>();
    assert_eq!(
        vec,
        vec![
            Id::from_string("123").unwrap(),
            Id::from_string("1234").unwrap(),
            Id::from_string("12345").unwrap()
        ]
    );
}

#[tokio::test]
async fn test_channels() {
    use telecomande::{Executor, SimpleExecutor};
    // cleaning;
    std::fs::remove_dir_all("/tmp/db-test").ok();

    // instantiation
    let store = SimpleExecutor::new(StorageProc::new("/tmp/db-test")).spawn();
    let remote = store.remote();

    // insertion
    let (cmd, rec) = StorageCmd::new_channel_create("a-channel");
    remote.send(cmd).unwrap();
    let id = rec.await.unwrap();

    // query all
    let (cmd, rec) = StorageCmd::new_channel_list();
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.len(), 1);
    let first = result[0];
    assert_eq!(first, id);

    // query property
    let (cmd, rec) = StorageCmd::new_channel_get_name(id);
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.unwrap(), "a-channel".to_string());

    // insertion
    let (cmd, rec) = StorageCmd::new_channel_create("b-channel");
    remote.send(cmd).unwrap();
    let id2 = rec.await.unwrap();

    // query all
    let (cmd, rec) = StorageCmd::new_channel_list();
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.len(), 2);

    // query property
    let (cmd, rec) = StorageCmd::new_channel_get_name(id2);
    remote.send(cmd).unwrap();
    let result = rec.await.unwrap();
    assert_eq!(result.unwrap(), "b-channel".to_string());
}
