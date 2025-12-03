use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

#[derive(Clone)]
pub struct Store {
    data: Arc<Mutex<HashMap<String, Entry>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).and_then(|entry| {
            if let Some(expires_at) = entry.expires_at
                && Instant::now() > expires_at
            {
                return None;
            }
            Some(entry.value.clone())
        })
    }

    pub fn exists(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn set(&self, key: String, value: String, ttl: Option<Duration>) {
        let mut data = self.data.lock().unwrap();
        let expires_at = ttl.map(|d| Instant::now() + d);
        data.insert(key, Entry { value, expires_at });
    }

    pub fn del(&self, key: &str) -> bool {
        let mut data = self.data.lock().unwrap();
        data.remove(key).is_some()
    }

    pub fn cleanup_expired(&self) {
        let mut data = self.data.lock().unwrap();
        let now = Instant::now();
        data.retain(|_, entry| match entry.expires_at {
            Some(expires_at) => now < expires_at,
            None => true,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let store = Store::new();
        store.set("name".to_string(), "Alice".to_string(), None);

        let value = store.get("name");
        assert_eq!(value, Some("Alice".to_string()));
    }

    #[test]
    fn test_get_nonexistent() {
        let store = Store::new();

        let value = store.get("unknown");
        assert_eq!(value, None);
    }

    #[test]
    fn test_del() {
        let store = Store::new();
        store.set("name".to_string(), "Alice".to_string(), None);

        let deleted = store.del("name");
        assert!(deleted);
        assert_eq!(store.get("name"), None);
    }

    #[test]
    fn test_del_nonexistent() {
        let store = Store::new();

        let deleted = store.del("unknown");
        assert!(!deleted);
    }

    #[test]
    fn test_exists() {
        let store = Store::new();
        store.set("name".to_string(), "Alice".to_string(), None);

        assert!(store.exists("name"));
        assert!(!store.exists("unknown"));
    }

    #[test]
    fn test_overwrite() {
        let store = Store::new();
        store.set("name".to_string(), "Alice".to_string(), None);
        store.set("name".to_string(), "Bob".to_string(), None);

        assert_eq!(store.get("name"), Some("Bob".to_string()));
    }

    #[test]
    fn test_ttl_expired() {
        let store = Store::new();
        store.set(
            "name".to_string(),
            "Alice".to_string(),
            Some(Duration::from_millis(10)),
        );

        // 期限切れを待つ
        std::thread::sleep(Duration::from_millis(50));

        assert_eq!(store.get("name"), None);
    }

    #[test]
    fn test_ttl_not_expired() {
        let store = Store::new();
        store.set(
            "name".to_string(),
            "Alice".to_string(),
            Some(Duration::from_secs(10)),
        );

        // すぐに取得
        assert_eq!(store.get("name"), Some("Alice".to_string()));
    }
}
