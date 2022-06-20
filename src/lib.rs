use std::collections::HashMap;

pub struct KvDb {
    db: HashMap<String, String>,
}

impl KvDb {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.db.get(&key).map(|s| s.to_string())
    }
}

#[cfg(test)]
mod test {
    fn get_db() -> super::KvDb {
        super::KvDb::new()
    }

    #[test]
    fn set_and_get() {
        let mut db = get_db();
        db.set("key".into(), "value 1".into());
        assert_eq!(db.get("key".into()), Some("value 1".into()));
    }
}
