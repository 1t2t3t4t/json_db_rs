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

fn setup<T: Database>(db: &T) {
    db.drop_db::<TestObj>();
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
    db.push_batch(thing_to_add.clone());
    return thing_to_add;
}

#[test]
fn test_write_and_read_single() {
    let db = JsonDatabase::new_with_path("db/test_write_and_read_single");
    setup(&db);
    let obj = TestObj {
        name: "YoYo".to_string(),
        rank: 1,
        age: 23,
        something: Some("Hi".to_string()),
    };

    db.save(obj.clone());

    let saved_obj = db.get_one::<TestObj>();
    assert_eq!(Some(obj), saved_obj);
}

#[test]
fn test_write_and_read_vec() {
    let db = JsonDatabase::default();
    setup(&db);
    let obj_amount = 20;
    let objs = write_objs(obj_amount);

    let all = db.get_all::<TestObj>();

    assert_eq!(all.len() as i32, obj_amount);
    assert_eq!(all, objs);
}
