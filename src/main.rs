///
/// library module
///
pub mod lib;

#[tokio::main]
pub async fn main() {
	lib::harsh::main().await;
}
