pub mod models;

use std::time::{SystemTime, UNIX_EPOCH};

use serde::{de::DeserializeOwned, Serialize};
use sled::Db;

use self::models::*;

use super::{
	config::Configuration,
	harsh::Error,
	log::{Loggable, Logger},
};

///
/// handle the database access
///
pub struct DbManager {
	path: String,
	handle: Option<Db>,
	logger: Logger,
}

impl DbManager {
	/// constructor
	pub fn new(config: Configuration, logger: Logger) -> Result<Self, Error> {
		let mut result = DbManager {
			path: config.database_path,
			handle: None,
			logger,
		};
		result.info("ininitalized new manager");

		result.open_db()?;
		result.info("openned database");
		Ok(result)
	}

	fn open_db(&mut self) -> Result<(), Error> {
		let handle = sled::open(&self.path)?;
		self.handle = Some(handle);
		Ok(())
	}

	pub fn create_user(&self, username: String, password: String) -> Result<User, Error> {
		let id = new_id(Prefixes::User);

		let user = User {
			username,
			password,
			id,
		};

		let path = format!("/user/{}", id);
		let serialized = serialize(&user)?;
		self.handle().insert(path, serialized)?;

		Ok(user)
	}

	// pub fn verify_user(&self, username: String, password: String) -> Result<User, Error> {}

	fn handle(&self) -> &Db {
		let db = match &self.handle {
			None => unreachable!(),
			Some(h) => h,
		};
		db
	}
}

impl Loggable for DbManager {
	fn name(&self) -> String {
		"db manager".to_string()
	}

	fn logger(&self) -> Logger {
		self.logger.clone()
	}
}

fn new_id(prefix: Prefixes) -> Id {
	let prefix = prefix as u8;
	let timestamp = SystemTime::now();
	let duration = timestamp.duration_since(UNIX_EPOCH).unwrap();
	let result = duration.as_millis() as Id;
	let result = (result << 256) + (prefix as Id);
	result
}

fn serialize<T: Serialize + Sized>(item: &T) -> Result<Vec<u8>, Error> {
	let serialized = serde_json::to_string(item)?;
	let array = serialized.as_bytes();
	let result = Vec::from(array);
	Ok(result)
}

fn deserialize<T: DeserializeOwned + Sized>(serialized: &[u8]) -> Result<T, Error> {
	let data = String::from_utf8(serialized.to_vec())?.clone();
	let result = serde_json::from_str::<T>(&data)?;
	Ok(result)
}
