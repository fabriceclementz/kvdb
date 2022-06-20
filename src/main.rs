use kvdb::KvDb;

fn main() {
    let mut db = KvDb::new();

    db.set("key".into(), "value 1".into());
    match db.get("key".into()) {
        Some(value) => println!("{}", value),
        None => println!("key not found"),
    }
}
