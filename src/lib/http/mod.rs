///
/// contains every route handlers
///
mod routes;

///
/// contains every models used by the http server
///
pub mod models;

use axum::{AddExtensionLayer, Router};

use self::routes::setup_routes;

use super::config::Configuration;
use super::harsh::SharedState;
use super::log::Loggable;
use super::log::Logger;

///
/// listen for incoming requests and handle both incoming and outgoing requests
///
pub struct HttpManager {
	shared_state: SharedState,
	port: u32,
	app: Router,
	logger: Logger,
}

impl HttpManager {
	/// constructor
	pub fn new(config: Configuration, shared_state: SharedState) -> Self {
		let app = setup_routes(Router::new()).layer(AddExtensionLayer::new(shared_state.clone()));
		let logger = shared_state.logger.try_read().unwrap().clone();

		let result = HttpManager {
			shared_state: shared_state.clone(),
			port: config.port,
			app,
			logger,
		};
		result.info("initialized new manager");

		result
	}

	/// listen for and handle received requests
	pub async fn serve(self) {
		let address = format!("0.0.0.0:{}", self.port).parse().unwrap();
		self.info(format!("listening to address '{}'", address));
		let server = axum::Server::bind(&address).serve(self.app.into_make_service());
		server.await.unwrap();
	}
}

impl Loggable for HttpManager {
	fn name(&self) -> String {
		"http manager".to_string()
	}

	fn logger(&self) -> Logger {
		self.logger.clone()
	}
}
