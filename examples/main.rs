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

fn test() {
    println!("Call");
    elapsed_time(|| {
        let db = JsonDatabase::default();
        let all = db.get_all::<MyThing>();
        println!("{}", all.len())
    });
}

#[tokio::main]
async fn main() {
    spawn_blocking(test).await;
}
