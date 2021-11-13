#![allow(unused, dead_code)]

use serde::{Deserialize, Serialize};

pub type Id = u64;

pub enum Prefixes {
	Message = 0,
	User = 10,
	Channel = 20,
}

#[derive(Serialize, Deserialize)]
pub struct User {
	pub username: String,
	pub password: String,
	pub id: Id,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
	pub name: String,
	pub id: Id,
	pub messages: Vec<Id>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
	pub content: String,
	pub id: Id,
}
