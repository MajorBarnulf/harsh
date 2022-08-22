#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub use client::ClientRequest;
pub mod client;

pub use server::ServerRequest;
pub mod server;
