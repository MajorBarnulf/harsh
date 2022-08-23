use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sled::Db;

use crate::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    id: Id,
    name: String,
}

impl Channel {
    pub fn new(name: String) -> Self {
        let id = Id::from_now();
        Self { id, name }
    }

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Id,
    name: String,
    pass: String,
}

impl User {
    pub fn new(name: String, pass: String) -> Self {
        let id = Id::from_now();
        Self { id, name, pass }
    }

    pub fn get_id(&self) -> Id {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_pass(&self) -> &str {
        &self.pass
    }

    pub fn set_pass(&mut self, pass: String) {
        self.pass = pass
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    id: Id,
    content: String,
}

impl Message {
    pub fn new(content: String) -> Self {
        let id = Id::from_now();
        Self { id, content }
    }

    pub fn get_id(&self) -> Id {
        self.id
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

pub trait SerDeser: Serialize + DeserializeOwned {
    fn ser(&self) -> Vec<u8>;
    fn deser(input: &[u8]) -> Option<Self>;
    fn read(db: &Db, path: String) -> Option<Self>;
    fn write(&self, db: &Db, path: String);
}

impl<T> SerDeser for T
where
    T: Serialize + DeserializeOwned,
{
    fn ser(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn deser(input: &[u8]) -> Option<Self> {
        serde_json::from_slice(input).ok()
    }

    fn read(db: &Db, path: String) -> Option<Self> {
        let bytes = db.get(path).unwrap()?;
        Self::deser(&bytes)
    }

    fn write(&self, db: &Db, path: String) {
        let bytes = self.ser();
        db.insert(path, bytes).unwrap();
    }
}
