use std::{borrow::Borrow, ops::Deref};

use super::config::Configuration;
use chrono::prelude::*;
use colored::Colorize;

///
/// configurable structure with methods to log informations in different ways
///
#[derive(Clone)]
pub struct Logger {
	//
}

impl Logger {
	/// adapt the static instance according to parameters specified by the configuration
	pub fn new(_config: Configuration) -> Self {
		Logger {}
	}

	pub fn info<T: Into<String>>(&self, msg: T) {
		println!("{} {} {}", time_segment(), "[info]".blue(), msg.into());
	}

	pub fn warn<T: Into<String>>(&self, msg: T) {
		println!("{} {} {}", time_segment(), "[warn]".yellow(), msg.into());
	}

	pub fn fail<T: Into<String>>(&self, msg: T) {
		println!("{} {} {}", time_segment(), "[fail]".red(), msg.into());
	}
}

/// make the time segment of a log
fn time_segment() -> String {
	let now = Local::now();
	let result = now.format("[%Y.%m.%d-%H:%M:%S]").to_string();
	result
}

pub trait Loggable {
	fn name(&self) -> String;

	fn logger(&self) -> Logger;

	fn info<T: Into<String>>(&self, msg: T) {
		self.logger().info(self.format_message(msg));
	}

	fn warn<T: Into<String>>(&self, msg: T) {
		self.logger().warn(self.format_message(msg));
	}

	fn fail<T: Into<String>>(&self, msg: T) {
		self.logger().fail(self.format_message(msg));
	}

	fn format_message<T: Into<String>>(&self, msg: T) -> String {
		let message: String = msg.into();
		let formatted = format!("[{}] {}", self.name(), message);
		formatted
	}
}
