use super::config::Configuration;
use super::database::DbManager;
use super::http::HttpManager;
use super::log::Logger;

///
/// main function
///
pub fn main() {
	let configuration = Configuration::read();
}

pub struct Harsh {
	db_manager: DbManager,
	logger: Logger,
	http_manager: HttpManager,
}

impl Harsh {
	pub fn new(configuration: Configuration) -> Self {
		let db_manager = DbManager::new(&configuration);
		let logger = Logger::new(&configuration);
		let http_manager = HttpManager::new();

		Harsh {
			logger,
			db_manager,
			http_manager,
		}
	}

	fn serve(&mut self) {}
}
