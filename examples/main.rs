use std::{sync::Arc, thread};

use futures::{executor::block_on, join};
use json_db_rs::{DatabaseOps, JsonDatabase};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;

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
    println!("{}", all.len());
}

fn extreme_write(db: &JsonDatabase) {
    println!("{:?}", thread::current().id());
    for i in 0..100 {
        let obj = MyThing {
            name: "YoYo".to_string(),
            rank: i,
            age: i,
            something: Some("Hi".to_string()),
        };
        db.push(obj);
    }
}

#[tokio::main]
async fn main() {
    let db = Arc::new(JsonDatabase::default());
    let db2 = db.clone();
    elapsed_time(|| {
        let join1 = spawn_blocking(move || {
            extreme_write(&*db);
        });
        let join2 = spawn_blocking(move || {
            extreme_write(&*db2);
        });
        block_on(async {
            join!(join1, join2);
        });
    });
    elapsed_time(|| {
        read_all();
    });
}
