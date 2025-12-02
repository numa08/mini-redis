use crate::store::Store;

pub enum Command {
    Ping,
    Get { key: String },
    Set { key: String, value: String },
    Del { key: String },
    Unknown,
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["PING"] => Command::Ping,
            ["GET", key] => Command::Get { key: key.to_string() },
            ["SET", key, value] => Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            },
            ["DEL", key] => Command::Del { key: key.to_string() },
            _ => Command::Unknown,
        }
    }

    pub fn execute(&self, store: &Store) -> String {
        match self {
            Command::Ping => "+PING\r\n".to_string(),
            Command::Get { key } => match store.get(key) {
                Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
                None => "$-1\r\n".to_string()
            },
            Command::Set { key, value } => {
                store.set(key.clone(), value.clone());
                "+OK\r\n".to_string()
            }
            Command::Del { key } => {
                if store.del(key) {
                    ":1\r\n".to_string()
                } else {
                    ":0\r\n".to_string()
                }
            }
            Command::Unknown => "-ERR unknown command\r\n".to_string()
        }
    }
}