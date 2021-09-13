#[cfg(test)]
mod tests {
    use json_db_rs::{Database, DatabaseOps, JsonDatabase};

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
        db.drop::<TestObj>(true);
        db.drop::<TestObj>(false);
    }

    fn write_objs(amount: i32, db: &JsonDatabase) -> Vec<TestObj> {
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
        db.push_batch(thing_to_add.clone()).unwrap();
        thing_to_add
    }

    fn setup_db(path: &str) -> JsonDatabase {
        let mut db = JsonDatabase::new_with_path(path);
        db.set_encode(false);
        db
    }

    #[test]
    fn test_write_and_read_single_not_existed() {
        let db = setup_db("db/test/test_write_and_read_single_not_existed");
        setup(&db);

        let saved_obj = db.get_one::<TestObj>();

        assert_eq!(None, saved_obj.unwrap());
    }

    #[test]
    fn test_write_and_read_single() {
        let db = setup_db("db/test/test_write_and_read_single");
        setup(&db);
        let obj = TestObj {
            name: "YoYo".to_string(),
            rank: 1,
            age: 23,
            something: Some("Hi".to_string()),
        };

        db.save(obj.clone()).unwrap();

        let saved_obj = db.get_one::<TestObj>();
        assert_eq!(Some(obj), saved_obj.unwrap());
    }

    #[test]
    fn test_write_and_read_vec() {
        let db = setup_db("db/test/test_write_and_read_vec");
        setup(&db);
        let obj_amount = 20;
        let objs = write_objs(obj_amount, &db);

        let all = db.get_all::<TestObj>().unwrap();

        assert_eq!(all.len() as i32, obj_amount);
        assert_eq!(all, objs);
    }

    #[test]
    fn test_write_and_read_vec_not_existed() {
        let db = setup_db("db/test/test_write_and_read_vec_not_existed");
        setup(&db);

        let all = db.get_all::<TestObj>().unwrap();

        assert_eq!(all.len() as i32, 0i32);
    }
}
