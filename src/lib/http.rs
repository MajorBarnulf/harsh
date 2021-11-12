use super::config::Configuration;
use super::database::DbManager;

///
/// listen for incoming requests and handle both incoming and outgoing requests
///
pub struct HttpManager {
	//
}

impl HttpManager {
	pub fn new() -> Self {
		HttpManager {}
	}

	/// listen for and handle received requests
	pub fn serve(_config: &Configuration, _db_manager: &mut DbManager) {
		todo!()
	}
}
