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
    let mut ts = Vec::<MyThing>::new();
    for i in 0..1000000 {
        if i % 1000 == 0 {
            println!("At {}", i);
        }
        ts.push(t.clone());
    }
    db.save_batch(ts);
    let all = db.get_all::<MyThing>();
    println!("{}", all.len());
}
