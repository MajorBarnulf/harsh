use super::config::Configuration;
use chrono::prelude::*;
use colored::Colorize;

///
/// configurable structure with methods to log informations in different ways
///
pub struct Logger {
	//
}

impl Logger {
	/// adapt the static instance according to parameters specified by the configuration
	pub fn new(_config: Configuration) -> Self {
		Logger {}
	}

	pub fn info(&self, msg: String) {
		let tag_section = "[info]".blue();
		println!("{} {} {}", time_segment(), tag_section, msg);
	}

	pub fn warn(&self, msg: String) {
		let tag_section = "[warn]".yellow();
		println!("{} {} {}", time_segment(), tag_section, msg);
	}

	pub fn fail(&self, msg: String) {
		let tag_section = "[fail]".red();
		println!("{} {} {}", time_segment(), tag_section, msg);
	}
}

/// make the time segment of a log
fn time_segment() -> String {
	let now = Local::now();
	let result = now.format("[%Y.%m.%d-%H:%M:%S]").to_string();
	result
}
