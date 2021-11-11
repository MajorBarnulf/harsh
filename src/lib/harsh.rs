use super::config::Configuration;
use super::database::DbManager;
use super::http::serve;
use super::log::Logger;

///
/// main function
///
pub fn main() {
	let configuration = Configuration::read();
	Logger::configure(&configuration);
	let mut db_manager = DbManager::new(&configuration);
	serve(&configuration, &mut db_manager);
}
