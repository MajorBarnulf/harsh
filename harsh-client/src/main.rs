use tokio::{
    io::{stdin, AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    println!("starting client ...");
    let stream = TcpStream::connect("localhost:8080").await.unwrap();
    println!("connected to 'localhost:8080'");
    let (reader, writer) = stream.into_split();
    tokio::spawn(async {
        let mut reader = BufReader::new(reader);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).await.unwrap();
            println!("received '{line}'");
        }
    });

    let input_loop = tokio::spawn(async {
        let mut input = BufReader::new(stdin());
        let mut writer = writer;

        loop {
            let mut line = String::new();
            input.read_line(&mut line).await.unwrap();
            let input = commands::parse(&line);
            match input {
                None => println!("failed to parse command"),
                Some(commands::Command::Help) => commands::help(),
                Some(commands::Command::Request(cmd)) => {
                    println!("sending..");
                    writer.write_all(cmd.serialize().as_bytes()).await.unwrap();
                    writer.write_all(b"\n").await.unwrap();
                }
            }
        }
    });

    println!("awaiting input ...");
    input_loop.await.unwrap();
}

mod commands {

    pub enum Command {
        Help,
        Request(harsh_common::ClientRequest),
    }

    pub fn parse(input: &str) -> Option<Command> {
        let mut parts = smart_split(input).into_iter();
        let command = match parts.next()?.as_str() {
            "help" => return Some(Command::Help),
            "ping" => {
                let rest = parts.collect::<Box<[_]>>();
                let content = rest.join(" ");
                harsh_common::ClientRequest::new_ping(content)
            }
            _ => return None,
        };

        Some(Command::Request(command))
    }

    pub fn smart_split(input: &str) -> Vec<String> {
        let input = input.trim();
        let mut result = Vec::new();

        let mut capturing = false;
        let mut ignoring = false;
        let mut current = String::new();
        for char in input.chars() {
            let char: char = char;
            if ignoring {
                current.push(char);
                ignoring = false;
                continue;
            }

            match char {
                '\\' => {
                    ignoring = true;
                }
                '"' => {
                    capturing = !capturing;
                }
                ' ' if !capturing => {
                    result.push(current);
                    current = String::new();
                }
                _ => current.push(char),
            }
        }
        result.push(current);
        result
    }

    #[test]
    fn test_smart_split() {
        assert_eq!(
            smart_split("hello world"),
            vec!["hello".to_string(), "world".to_string()]
        );
        assert_eq!(
            smart_split(r#""lorem ipsum" "dolor amit""#),
            vec!["lorem ipsum".to_string(), "dolor amit".to_string()]
        );
        assert_eq!(
            smart_split(r#"lorem "ipsum do"lor "amit""#),
            vec![
                "lorem".to_string(),
                "ipsum dolor".to_string(),
                "amit".to_string()
            ]
        );
    }

    pub struct Description {
        name: &'static str,
        params: &'static [&'static str],
        desc: &'static str,
    }

    impl Description {
        pub const fn new(
            name: &'static str,
            params: &'static [&'static str],
            desc: &'static str,
        ) -> Self {
            Self { name, desc, params }
        }

        pub const ALL: &'static [Self] = &[
            // all commands
            Self::new("help", &[], "returns a help message"),
            Self::new(
                "ping",
                &["content"],
                "sends a ping with the specified content",
            ),
        ];
    }

    pub fn help() {
        for &Description { name, params, desc } in Description::ALL {
            let mut usage = params.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            usage.insert(0, name.to_string());
            let usage = usage.join(" ");
            println!("{name}:\n\tusage:\n\t\t{usage}\n\n\tdescription:\n\t\t{desc}\n");
        }
    }
}
