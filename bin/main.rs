use json_db_rs::{Database, JsonDatabase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyThing {
    name: String,
    age: i32,
}

fn main() {
    let t = MyThing {
        name: String::from("Boss"),
        age: 25,
    };
    let db = JsonDatabase::default();
    db.save(t.clone());
    let all = db.get_all::<MyThing>();
    println!("{:#?}", all);
}
