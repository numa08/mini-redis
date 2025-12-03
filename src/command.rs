use std::time::Duration;

use crate::store::Store;

pub enum Command {
    Ping,
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        ttl: Option<Duration>,
    },
    Del {
        key: String,
    },
    Exists {
        key: String,
    },
    Unknown,
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["PING"] => Command::Ping,
            ["GET", key] => Command::Get {
                key: key.to_string(),
            },
            ["SET", key, value] => Command::Set {
                key: key.to_string(),
                value: value.to_string(),
                ttl: None,
            },
            ["SET", key, value, "EX", seconds] => {
                let ttl = seconds.parse::<u64>().ok().map(Duration::from_secs);
                Command::Set {
                    key: key.to_string(),
                    value: value.to_string(),
                    ttl,
                }
            }
            ["DEL", key] => Command::Del {
                key: key.to_string(),
            },
            ["EXISTS", key] => Command::Exists {
                key: key.to_string(),
            },
            _ => Command::Unknown,
        }
    }

    pub fn execute(&self, store: &Store) -> String {
        match self {
            Command::Ping => "+PING\r\n".to_string(),
            Command::Get { key } => match store.get(key) {
                Some(value) => format!("${}\r\n{}\r\n", value.len(), value),
                None => "$-1\r\n".to_string(),
            },
            Command::Set { key, value, ttl } => {
                store.set(key.clone(), value.clone(), *ttl);
                "+OK\r\n".to_string()
            }
            Command::Del { key } => {
                if store.del(key) {
                    ":1\r\n".to_string()
                } else {
                    ":0\r\n".to_string()
                }
            }
            Command::Exists { key } => {
                if store.exists(key) {
                    ":1\r\n".to_string()
                } else {
                    ":0\r\n".to_string()
                }
            }
            Command::Unknown => "-ERR unknown command\r\n".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ping() {
        let cmd = Command::parse("PING");
        assert!(matches!(cmd, Command::Ping));
    }

    #[test]
    fn test_parse_get() {
        let cmd = Command::parse("GET name");
        assert!(matches!(cmd, Command::Get { key } if key == "name"));
    }

    #[test]
    fn test_parse_set() {
        let cmd = Command::parse("SET name Alice");
        assert!(matches!(
            cmd,
            Command::Set { key, value, ttl: None } if key == "name" && value == "Alice"
        ));
    }

    #[test]
    fn test_parse_set_with_ttl() {
        let cmd = Command::parse("SET name Alice EX 10");
        assert!(matches!(
            cmd,
            Command::Set { key, value, ttl: Some(d) }
                if key == "name" && value == "Alice" && d.as_secs() == 10
        ));
    }

    #[test]
    fn test_parse_del() {
        let cmd = Command::parse("DEL name");
        assert!(matches!(cmd, Command::Del { key } if key == "name"));
    }

    #[test]
    fn test_parse_unknown() {
        let cmd = Command::parse("INVALID");
        assert!(matches!(cmd, Command::Unknown));
    }
}
