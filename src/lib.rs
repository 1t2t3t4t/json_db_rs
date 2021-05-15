use std::sync::Mutex;

use serde::{de::DeserializeOwned, Serialize};

use std::path::Path;

mod json_io;

pub trait Database: Send + Sync {
    fn get_one<E>(&self) -> Option<E>
    where
        E: DeserializeOwned;
    fn get_all<E>(&self) -> Vec<E>
    where
        E: DeserializeOwned;

    fn save<E>(&self, entity: E)
    where
        E: Serialize + DeserializeOwned;
    fn push<E>(&self, entity: E)
    where
        E: Serialize + DeserializeOwned;
    fn push_batch<E>(&self, entities: Vec<E>)
    where
        E: Serialize + DeserializeOwned;

    fn drop_db<E>(&self)
    where
        E: DeserializeOwned;
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
    std::fs::create_dir_all(p).unwrap();
}

impl JsonDatabase {
    pub fn new_with_path(path: &str) -> Self {
        Self {
            path: path.to_string(),
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
}

impl Default for JsonDatabase {
    fn default() -> Self {
        Self::new_with_path(DEFAULT_FILE_PATH)
    }
}

impl Database for JsonDatabase {
    fn get_one<E>(&self) -> Option<E>
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>();
        json_io::load_json(path)
    }

    fn get_all<E>(&self) -> Vec<E>
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>();
        let json = json_io::load_json_vec::<E>(path);
        json
    }

    fn save<E>(&self, entity: E)
    where
        E: Serialize + DeserializeOwned,
    {
        let _guard = self.fs_mutex.lock();
        let path = self.path_to_entity::<E>();
        json_io::save_json(path, entity);
    }

    fn push<E>(&self, entity: E)
    where
        E: Serialize + DeserializeOwned,
    {
        let _guard = self.fs_mutex.lock();
        let mut all = self.get_all::<E>();
        all.push(entity);

        let path = self.path_to_entity::<E>();
        json_io::save_json(path, all);
    }

    fn push_batch<E>(&self, mut entities: Vec<E>)
    where
        E: Serialize + DeserializeOwned,
    {
        let _guard = self.fs_mutex.lock();
        let mut all = self.get_all::<E>();
        all.append(&mut entities);

        let path = self.path_to_entity::<E>();
        json_io::save_json(path, all);
    }

    fn drop_db<E>(&self)
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>();
        let path_obj = std::path::Path::new(&path);
        if path_obj.exists() {
            std::fs::remove_file(path_obj).expect("Unable to drop");
        }
    }
}
