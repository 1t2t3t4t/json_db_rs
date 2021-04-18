use serde::{Deserialize, Serialize};
use json_db_rs::{JsonDatabase, Database};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyThing {
    name: String,
    age: i32
}

fn main() {
    let t = MyThing { name: String::from("Boss"), age: 25 };
    let db = JsonDatabase::default();
    db.save(t.clone());
    let all = db.get_all::<MyThing>();
    println!("{:#?}", all);
}
