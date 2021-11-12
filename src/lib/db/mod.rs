pub mod models;

use sled::Db;

use super::{config::Configuration, harsh::Error};

///
/// handle the database access
///
pub struct DbManager {
	handle: Db,
}

impl DbManager {
	/// constructor
	pub fn new(config: Configuration) -> Result<Self, Error> {
		let handle = sled::open(config.database_path)?;
		Ok(DbManager { handle })
	}
}
