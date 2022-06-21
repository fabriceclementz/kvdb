use kvdb::KvDb;

fn main() {
    let mut db = KvDb::new("kvdb");

    println!("{:?}", db);

    db.set("key".into(), "value 1".into());
    db.set("key2".into(), "value key 2".into());
    db.set("key3".into(), "value key 3".into());
    // match db.get("key".into()) {
    //     Some(value) => println!("{}", value),
    //     None => println!("key not found"),
    // }
}
