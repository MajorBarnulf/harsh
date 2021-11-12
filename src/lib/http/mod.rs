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

///
/// listen for incoming requests and handle both incoming and outgoing requests
///
pub struct HttpManager {
	_shared_state: SharedState,
	app: Router,
}

impl HttpManager {
	/// constructor
	pub fn new(_config: Configuration, shared_state: SharedState) -> Self {
		let app = setup_routes(Router::new()).layer(AddExtensionLayer::new(shared_state.clone()));

		HttpManager {
			_shared_state: shared_state.clone(),
			app,
		}
	}

	/// listen for and handle received requests
	pub async fn serve(self) {
		let address = "0.0.0.0:3000".parse().unwrap();
		axum::Server::bind(&address)
			.serve(self.app.into_make_service())
			.await
			.unwrap();
	}
}
