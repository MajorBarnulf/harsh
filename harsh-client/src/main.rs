use std::{
    io::{stdout, Write},
    process::exit,
};

use harsh_common::ServerEvent;
use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

const ADDRESS: &str = "localhost:42000";

#[tokio::main]
async fn main() {
    println!("[main/info] starting client ...");
    let stream = TcpStream::connect(ADDRESS).await.unwrap();
    println!("[main/info] connected to '{ADDRESS}'");
    let (reader, writer) = stream.into_split();
    tokio::spawn(async {
        let mut reader = BufReader::new(reader);
        loop {
            let mut line = String::new();
            if let Ok(0) = reader.read_line(&mut line).await {
                break;
            }
            if let Some(parsed) = ServerEvent::try_parse(&line) {
                println!("[main/info] received '{parsed:?}'");
            }
        }
        println!("[main/info] connection closed, goodbye.");
        exit(0);
    });

    let input_loop = tokio::spawn(async {
        let mut input = BufReader::new(stdin());
        let mut writer = writer;

        loop {
            print!("$> ");
            stdout().lock().flush().unwrap();
            let mut line = String::new();
            input.read_line(&mut line).await.unwrap();
            let input = commands::parse(&line);
            match input {
                None => println!("[main/warn] failed to parse command"),
                Some(commands::Command::Help) => commands::help(),
                Some(commands::Command::Request(cmd)) => {
                    println!("[main/info] sending..");
                    writer.write_all(cmd.serialize().as_bytes()).await.unwrap();
                    writer.write_all(b"\n").await.unwrap();
                }
            }
        }
    });

    println!("[main/info] awaiting input ...");
    input_loop.await.unwrap();
}

mod commands;
