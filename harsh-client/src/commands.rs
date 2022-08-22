use harsh_common::ClientRequest;

pub enum Command {
    Help,
    Request(ClientRequest),
}

pub fn parse(input: &str) -> Option<Command> {
    let mut parts = smart_split(input).into_iter();
    let command = match parts.next()?.as_str() {
        "help" => return Some(Command::Help),
        "ping" => {
            let rest = parts.collect::<Box<[_]>>();
            let content = rest.join(" ");
            ClientRequest::new_ping(content)
        }
        "chanls" => ClientRequest::new_channel_list(),
        "chanadd" => {
            let name = parts.next()?;
            ClientRequest::new_channel_create(name)
        }
        "chandel" => {
            let id = parts.next()?.parse().ok()?;
            ClientRequest::new_channel_delete(id)
        }
        "changname" => {
            let id = parts.next()?.parse().ok()?;
            ClientRequest::new_channel_get_name(id)
        }
        "chansname" => {
            let id = parts.next()?.parse().ok()?;
            let name = parts.next()?;
            ClientRequest::new_channel_set_name(id, name)
        }
        "msgls" => {
            let channel_id = parts.next()?.parse().ok()?;
            ClientRequest::new_message_list(channel_id)
        }
        "msgadd" => {
            let channel_id = parts.next()?.parse().ok()?;
            let content = parts.next()?;
            ClientRequest::new_message_create(channel_id, content)
        }
        "msgdel" => {
            let channel_id = parts.next()?.parse().ok()?;
            let id = parts.next()?.parse().ok()?;
            ClientRequest::new_message_delete(channel_id, id)
        }
        "msggcont" => {
            let channel_id = parts.next()?.parse().ok()?;
            let id = parts.next()?.parse().ok()?;
            ClientRequest::new_message_get_content(channel_id, id)
        }
        "msgscont" => {
            let channel_id = parts.next()?.parse().ok()?;
            let id = parts.next()?.parse().ok()?;
            let content = parts.next()?;
            ClientRequest::new_message_set_content(channel_id, id, content)
        }
        "usrls" => ClientRequest::new_user_list(),
        "usradd" => {
            let name = parts.next()?;
            let pass = parts.next()?;
            ClientRequest::new_user_create(name, pass)
        }
        "usrdel" => {
            let id = parts.next()?.parse().ok()?;
            ClientRequest::new_user_delete(id)
        }
        "usrgname" => {
            let id = parts.next()?.parse().ok()?;
            ClientRequest::new_user_get_name(id)
        }
        "usrsname" => {
            let id = parts.next()?.parse().ok()?;
            let name = parts.next()?;
            ClientRequest::new_user_set_name(id, name)
        }
        "usrspass" => {
            let id = parts.next()?.parse().ok()?;
            let pass = parts.next()?;
            ClientRequest::new_user_set_pass(id, pass)
        }
        _ => return None,
    };

    Some(Command::Request(command))
}

pub const CMDS: &'static [Description] = &[
    // all commands
    Description::new("help", &[], "returns a help message"),
    Description::new(
        "ping",
        &["content"],
        "sends a ping with the specified content",
    ),
    Description::new("chanls", &[], "list channels"),
    Description::new("chanadd", &["name"], "creates a new channel"),
    Description::new("chandel", &["id"], "delete a channel by its id"),
    Description::new("changname", &["id"], "get a channel's name"),
    Description::new("chansname", &["id", "name"], "set a channel's name"),
    Description::new("msgls", &["channel_id"], "list messages"),
    Description::new("msgadd", &["channel_id", "content"], "create a message"),
    Description::new("msgdel", &["channel_id", "id"], "delete a message"),
    Description::new("msggcont", &["channel_id", "id"], "get a message's content"),
    Description::new(
        "msgscont",
        &["channel_id", "id", "content"],
        "set a message's content",
    ),
    Description::new("usrls", &[], "list users"),
    Description::new("usradd", &["name", "pass"], "add a user"),
    Description::new("usrdel", &["id"], "delete a user"),
    Description::new("usrgname", &["id"], "get a user name"),
    Description::new("usrsname", &["id", "name"], "set a user name"),
    Description::new("usrspass", &["id", "pass"], "set a user pass"),
];

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
            '\\' => ignoring = true,
            '"' => capturing = !capturing,
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
}

pub fn help() {
    for &Description { name, params, desc } in CMDS {
        let mut usage = params.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        usage.insert(0, name.to_string());
        let usage = usage.join(" ");
        println!("{name}:\n\tusage:\t\t{usage}\n\tdescription:\t{desc}");
    }
}
