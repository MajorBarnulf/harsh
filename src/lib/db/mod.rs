pub mod models;

use sled::Db;

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

	pub fn open_db(&mut self) -> Result<(), Error> {
		let handle = sled::open(&self.path)?;
		self.handle = Some(handle);
		Ok(())
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
