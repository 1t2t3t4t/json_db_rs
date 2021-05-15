use json_db_rs::{Database, JsonDatabase};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyThing {
    name: String,
    age: i32,
    #[serde(default)]
    rank: i32,
    #[serde(default)]
    something: Option<String>,
}

fn elapsed_time(func: impl FnOnce()) {
    let start = std::time::Instant::now();
    func();
    println!(
        "Elapsed time: {} m sec",
        start.elapsed().as_secs_f32() * 1000f32
    );
}

fn read_all() {
    println!("Call");
    let db = JsonDatabase::default();
    let all = db.get_all::<MyThing>();
    println!("{}", all.len())
}

fn extreme_write() {
    let db = JsonDatabase::default();
    let mut thing_to_add = Vec::<MyThing>::new();
    for i in 0..1000 {
        let obj = MyThing {
            name: "YoYo".to_string(),
            rank: i,
            age: i,
            something: Some("Hi".to_string()),
        };
        thing_to_add.push(obj);
    }
    db.push_batch(thing_to_add);
}

#[tokio::main]
async fn main() {
    elapsed_time(|| {
        extreme_write();
        read_all();
    });
}
