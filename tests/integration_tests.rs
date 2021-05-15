use json_db_rs::{Database, JsonDatabase};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct TestObj {
    name: String,
    age: i32,
    #[serde(default)]
    rank: i32,
    #[serde(default)]
    something: Option<String>,
}

fn write_objs(amount: i32) -> Vec<TestObj> {
    let db = JsonDatabase::default();
    let mut thing_to_add = Vec::<TestObj>::new();
    for i in 0..amount {
        let obj = TestObj {
            name: "YoYo".to_string(),
            rank: i,
            age: i,
            something: Some("Hi".to_string()),
        };
        thing_to_add.push(obj);
    }
    db.save_batch(thing_to_add.clone());
    return thing_to_add;
}

#[test]
fn test_write_and_read() {
    let db = JsonDatabase::default();
    db.drop_db::<TestObj>();
    let obj_amount = 20;
    let objs = write_objs(obj_amount);

    let all = db.get_all::<TestObj>();

    assert_eq!(all.len() as i32, obj_amount);
    assert_eq!(all, objs);
}
