///
/// library module
///
pub mod lib;

#[tokio::main]
pub async fn main() {
	println!("Hello, harmony!");
	lib::harsh::main().await;
}
