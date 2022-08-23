use telecomande::{Executor, SimpleExecutor};
use tokio::net::TcpListener;

const ADDRESS: &str = "localhost:42000";
const DB_PATH: &str = "./db.test";

#[tokio::main]
async fn main() {
    println!("[main/info] starting server ...");

    let sessions = SimpleExecutor::new(SessionProc::default()).spawn();
    println!("[main/info] spawned sessions");

    let storage = SimpleExecutor::new(StorageProc::new(DB_PATH)).spawn();
    println!("[main/info] spawned storage");

    let security = SimpleExecutor::new(SecurityProc::new(storage.remote())).spawn();

    let gateway = SimpleExecutor::new(GatewayProc::new(
        sessions.remote(),
        storage.remote(),
        security.remote(),
    ))
    .spawn();
    println!("[main/info] spawned gateway");

    let listener = TcpListener::bind(ADDRESS).await.unwrap();
    println!("[main/info] listening on '{ADDRESS}' ...");
    let client_handler = sessions.remote();
    loop {
        let (stream, address) = listener.accept().await.unwrap();
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

mod security;
pub use security::{SecurityCmd, SecurityProc};
