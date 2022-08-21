use telecomande::{Executor, SimpleExecutor};
use tokio::net::TcpListener;

#[tokio::main]

async fn main() {
    println!("starting server ...");
    let client_handler = SimpleExecutor::new(SessionProc::default()).spawn();
    let storage = SimpleExecutor::new(StorageProc::new("./db")).spawn();
    let gateway =
        SimpleExecutor::new(GatewayProc::new(client_handler.remote(), storage.remote())).spawn();
    println!("spawned gateway");

    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("listening on 'localhost:8080' ...");

    let client_handler = client_handler.remote();
    loop {
        let (stream, address) = listener.accept().await.unwrap();
        println!("new connection from '{address:?}'");

        client_handler
            .send(sessions::SessionCmd::AddSession(
                stream,
                address,
                gateway.remote(),
            ))
            .unwrap();
    }
}

mod utils;
pub use utils::{Addr, Id};

mod gateway;
pub use gateway::{GatewayCmd, GatewayProc};

mod sessions;
pub use sessions::{SessionCmd, SessionProc};

pub use storage::{StorageCmd, StorageProc};
mod storage;
