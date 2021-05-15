use json_db_rs::{Database, JsonDatabase};
use serde::{Deserialize, Serialize};
use futures::executor::block_on;
use tokio::task::spawn_blocking;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyThing {
    name: String,
    age: i32,
    #[serde(default)]
    rank: i32,
    #[serde(default)]
    something: Option<String>
}

fn elapsed_time(func: impl FnOnce()) {
    let start = std::time::Instant::now();
    func();
    println!("Elapsed time: {} sec", start.elapsed().as_secs_f32());
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
    for i in 0..20000000 {
        if i % 1000000 == 0 {
            println!("At {}", i);
        }
        let obj = MyThing {
            name: "YoYo".to_string(),
            rank: i,
            age: i,
            something: Some("Hi".to_string())
        };
        thing_to_add.push(obj);
    }
    db.save_batch(thing_to_add);
}

#[tokio::main]
async fn main() {
    elapsed_time(|| {
        extreme_write();
        read_all();
    });
}
