#![allow(unused, dead_code)]

type Id = u64;

pub struct User {
	username: String,
	password: String,
	id: Id,
}

pub struct Channel {
	name: String,
	id: Id,
	messages: Vec<Id>,
}

pub struct Message {
	content: String,
	id: Id,
}
