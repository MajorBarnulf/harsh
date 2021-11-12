use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
	name: String,
	password: String,
}

pub struct EditUser {
	//
}

pub struct DeleteUser {
	name: String,
	password: String,
}
