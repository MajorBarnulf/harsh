use telecomande::{Executor, SimpleExecutor};
use tokio::net::TcpListener;

#[tokio::main]

async fn main() {
    println!("[main/info] starting server ...");
    let sessions = SimpleExecutor::new(SessionProc::default()).spawn();
    println!("[main/info] spawned sessions");
    let storage = SimpleExecutor::new(StorageProc::new("/tmp/db.test")).spawn();
    println!("[main/info] spawned storage");
    let gateway =
        SimpleExecutor::new(GatewayProc::new(sessions.remote(), storage.remote())).spawn();
    println!("[main/info] spawned gateway");

    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    println!("[main/info] listening on 'localhost:8080' ...");

    let client_handler = sessions.remote();
    loop {
        let (stream, address) = listener.accept().await.unwrap();
        println!("[main/info] new connection from '{address:?}'");

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

mod storage;
pub use storage::{StorageCmd, StorageProc};
