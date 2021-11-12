use std::fs::{read_to_string, write};

use ron;
use serde::{Deserialize, Serialize};

use super::harsh::Error;

///
/// Encapsulate a configuration for the server
///
#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
	pub database_path: String, //
	pub port: u32,
}

static CONFIG_PATHS: [&str; 2] = ["./config.ron", "/etc/harsh/harsh.ron"];

impl Configuration {
	/// try to read the configuration from local file or load default
	pub fn get() -> Self {
		match Self::try_from_file() {
			Ok(config) => config,
			Err(_) => {
				let result = Configuration::default();
				Configuration::write_config(&result);
				result
			}
		}
	}

	fn try_from_file() -> Result<Self, Error> {
		let content = Self::try_read()?;
		let config: Configuration = ron::from_str(&content)?;
		Ok(config)
	}

	fn try_read() -> Result<String, Error> {
		for path in CONFIG_PATHS {
			match read_to_string(path) {
				Ok(serialized) => return Ok(serialized),
				_ => (),
			}
		}
		Err("unable to locate or read config file".to_string().into())
	}

	fn write_config(data: &Self) {
		let serialized = ron::to_string(data).unwrap();
		write(CONFIG_PATHS[0], serialized);
	}
}

impl Default for Configuration {
	fn default() -> Self {
		Self {
			database_path: "./database".to_string(),
			port: 42069, // haha funny number
		}
	}
}
