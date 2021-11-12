///
/// encapsulate the logic responsible for reading and parsing the server configuration
///
pub mod config;

///
/// encapsulate the logic responsible for interacting with the database
///
pub mod db;

///
/// main module, exposes the highest level structure holding the whole server runtime logic
///
pub mod harsh;

///
/// encapsulate the logic responsible for receiving and routing http requests
///
pub mod http;

///
/// encapsulate the logic responsible for logging events occuring while the server is running
///
pub mod log;
