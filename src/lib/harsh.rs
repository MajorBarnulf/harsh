use std::error;
use std::sync::{Arc, RwLock};

use super::config::Configuration;
use super::db::DbManager;
use super::http::HttpManager;
use super::log::Logger;

///
/// main function
///
pub async fn main() {
	let configuration = Configuration::get();
	let instance = Harsh::new(configuration);
	instance.serve().await;
}

pub struct Harsh {
	pub shared_state: SharedState,
	pub http_manager: HttpManager,
}

impl Harsh {
	pub fn new(configuration: Configuration) -> Self {
		let logger = Logger::new(configuration.clone());
		let db_manager = DbManager::new(configuration.clone(), logger.clone())
			.expect("failed to open / create database");
		let shared_state = State::new_shared(db_manager, logger.clone());
		let http_manager = HttpManager::new(configuration.clone(), shared_state.clone());

		Harsh {
			shared_state,
			http_manager,
		}
	}

	pub async fn serve(self) {
		self.http_manager.serve().await;
	}
}

///
/// shared state arround the app
///
pub struct State {
	pub db_manager: RwLock<DbManager>,
	pub logger: RwLock<Logger>,
}

impl State {
	pub fn new_shared(db_manager: DbManager, logger: Logger) -> SharedState {
		logger.info("initialized shared state");
		let state = State {
			db_manager: RwLock::new(db_manager),
			logger: RwLock::new(logger),
		};
		Arc::new(state)
	}
}

///
/// safe pointer to the shared state
///
pub type SharedState = Arc<State>;

///
/// error type for now..
///
pub type Error = Box<dyn error::Error>;
