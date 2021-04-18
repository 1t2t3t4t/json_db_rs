use std::{borrow::Borrow, sync::Mutex};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Result, Value, json};
use std::path::Path;

pub trait Database: Send + Sync {
    fn get_all<E>(&self) -> Vec<E>
    where
        E: DeserializeOwned;

    fn save<E>(&self, entity: E)
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
            fs_mutex: Mutex::new(())
        }
    }

    fn path_to_entity<E>(&self) -> String {
        guard_path_does_not_exist(self.path.clone());
        let type_name = std::any::type_name::<E>()
            .split("::")
            .last()
            .unwrap_or_default();
        format!("{}/{}.json", self.path, type_name)
    }

    fn load_json<E>(&self) -> Vec<E>
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>();
        let file_content = std::fs::read_to_string(path).unwrap_or_default();
        let entities = serde_json::from_str::<Vec<E>>(&file_content);
        entities.unwrap()
    }

    fn save_json<E>(&self, json: Vec<E>)
    where
        E: Serialize,
    {
        let path = self.path_to_entity::<E>();
        let json_str = serde_json::to_string(&json).unwrap_or_default();
        std::fs::write(path.clone(), &json_str)
            .expect(&format!("Unable to write to path {}", path));
    }
}

impl Default for JsonDatabase {
    fn default() -> Self {
        Self {
            path: DEFAULT_FILE_PATH.to_string(),
            fs_mutex: Mutex::new(())
        }
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
        let mut all = self.get_all::<E>();
        all.push(entity);
        self.save_json(all);
    }
}
