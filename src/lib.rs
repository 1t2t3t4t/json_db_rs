use std::sync::Mutex;

use serde::{de::DeserializeOwned, Serialize};

use std::fs::File;
use std::io::{Cursor, Write};
use std::path::Path;

pub trait Database: Send + Sync {
    fn get_all<E>(&self) -> Vec<E>
    where
        E: DeserializeOwned;

    fn save<E>(&self, entity: E)
    where
        E: Serialize + DeserializeOwned;
    fn save_batch<E>(&self, entities: Vec<E>)
    where
        E: Serialize + DeserializeOwned;
}

const DEFAULT_FILE_PATH: &str = "db";

#[derive(Debug)]
pub struct JsonDatabase {
    fs_mutex: Mutex<()>,
    path: String,
}

fn guard_path_does_not_exist(p: String) {
    let p = Path::new(&p);
    if p.exists() {
        return;
    }
    std::fs::create_dir(p).unwrap();
}

impl JsonDatabase {
    pub fn new_with_path(path: String) -> Self {
        Self {
            path,
            fs_mutex: Mutex::new(()),
        }
    }

    fn path_to_entity<E>(&self) -> String {
        guard_path_does_not_exist(self.path.clone());
        let type_name = std::any::type_name::<E>()
            .split("::")
            .last()
            .unwrap_or_default();
        format!("{}/{}.jsondb", self.path, type_name)
    }

    pub fn drop_db<E>(&self)
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>();
        let path_obj = std::path::Path::new(&path);
        if path_obj.exists() {
            std::fs::remove_file(path_obj).expect("Unable to drop");
        }
    }

    fn load_json<E>(&self) -> Vec<E>
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>();
        let file_content = std::fs::read(path).unwrap_or_default();
        let file_cursor = Cursor::new(file_content);
        let decoded = zstd::decode_all(file_cursor).unwrap_or_default();
        let json_content = String::from_utf8(decoded).unwrap_or_default();
        if json_content.is_empty() {
            return vec![];
        }
        let entities = serde_json::from_str::<Vec<E>>(&json_content);
        entities.unwrap()
    }

    fn save_json<E>(&self, json: Vec<E>)
    where
        E: Serialize,
    {
        let path = self.path_to_entity::<E>();
        let json_str = serde_json::to_string(&json).unwrap_or_default();
        let encoded = zstd::encode_all(json_str.as_bytes(), 0).unwrap();
        let mut file = File::create(path.clone()).unwrap();
        file.write_all(&encoded)
            .expect(&format!("Unable to write to path {}", path));
    }
}

impl Default for JsonDatabase {
    fn default() -> Self {
        Self::new_with_path(DEFAULT_FILE_PATH.to_string())
    }
}

impl Database for JsonDatabase {
    fn get_all<E>(&self) -> Vec<E>
    where
        E: DeserializeOwned,
    {
        let json = self.load_json::<E>();
        json
    }

    fn save<E>(&self, entity: E)
    where
        E: Serialize + DeserializeOwned,
    {
        let _guard = self.fs_mutex.lock();
        let mut all = self.get_all::<E>();
        all.push(entity);
        self.save_json(all);
    }

    fn save_batch<E>(&self, mut entities: Vec<E>)
    where
        E: Serialize + DeserializeOwned,
    {
        let _guard = self.fs_mutex.lock();
        let mut all = self.get_all::<E>();
        all.append(&mut entities);
        self.save_json(all);
    }
}
