use std::cell::RefCell;

use super::config::Configuration;
use chrono::prelude::*;
use colored::Colorize;

///
/// a static custom logger
///
pub struct Logger {
	//
}

/// static Logger instance
pub const LOGGER: RefCell<Logger> = RefCell::new(Logger {});

impl Logger {
	/// adapt the static instance according to parameters specified by the configuration
	pub fn configure(_config: &Configuration) {
		let refcell = LOGGER;
		let _logger = refcell.try_borrow_mut().unwrap();
		todo!()
	}

	fn log_info(&self, msg: String) {
		let tag_section = "[info]".blue();
		println!("{} {} {}", time_section(), tag_section, msg);
	}

	fn log_warn(&self, msg: String) {
		let tag_section = "[warn]".yellow();
		println!("{} {} {}", time_section(), tag_section, msg);
	}

	fn log_fail(&self, msg: String) {
		let tag_section = "[fail]".red();
		println!("{} {} {}", time_section(), tag_section, msg);
	}
}

fn time_section() -> String {
	let now = Local::now();
	let result = now.format("[%Y.%m.%d-%H:%M:%S]").to_string();
	result
}

pub mod shorthands {
	pub fn log_info(msg: String) {
		super::LOGGER.try_borrow().unwrap().log_info(msg);
	}

	pub fn log_warn(msg: String) {
		super::LOGGER.try_borrow().unwrap().log_warn(msg);
	}

	pub fn log_fail(msg: String) {
		super::LOGGER.try_borrow().unwrap().log_fail(msg);
	}
}
