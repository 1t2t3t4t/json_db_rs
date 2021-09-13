use std::sync::{LockResult, Mutex, MutexGuard};

use serde::{de::DeserializeOwned, Serialize};

pub use std::io::Result;
use std::path::Path;

mod json_io;

pub trait DatabaseOps {
    fn get_one<E>(&self) -> Result<Option<E>>
    where
        E: DeserializeOwned;
    fn get_all<E>(&self) -> Result<Vec<E>>
    where
        E: DeserializeOwned;

    fn save<E>(&self, entity: E) -> Result<()>
    where
        E: Serialize + DeserializeOwned;
    fn push<E>(&self, entity: E) -> Result<()>
    where
        E: Serialize + DeserializeOwned;
    fn push_batch<E>(&self, entities: Vec<E>) -> Result<()>
    where
        E: Serialize + DeserializeOwned;
}

pub trait Database: Send + Sync + DatabaseOps {
    fn transaction(&self, func: impl FnOnce());

    fn drop<E>(&self, pluralize: bool)
    where
        E: DeserializeOwned;
}

const DEFAULT_FILE_PATH: &str = "db";

#[derive(Debug)]
pub struct JsonDatabase {
    fs_mutex: Mutex<()>,
    transaction_mutex: Mutex<()>,
    path: String,
    encode: bool,
}

fn guard_path_does_not_exist(p: String) {
    let p = Path::new(&p);
    if p.exists() {
        return;
    }
    std::fs::create_dir_all(p).unwrap();
}

fn get_lock<T>(mutex_result: LockResult<MutexGuard<T>>) -> MutexGuard<T> {
    match mutex_result {
        Ok(lock) => lock,
        Err(poison) => poison.into_inner(),
    }
}

impl JsonDatabase {
    pub fn set_encode(&mut self, flag: bool) {
        self.encode = flag;
    }

    pub fn new_with_path(path: &str) -> Self {
        Self {
            path: path.to_string(),
            fs_mutex: Mutex::new(()),
            transaction_mutex: Mutex::new(()),
            encode: true,
        }
    }

    fn path_to_entity<E>(&self, pluralize: bool) -> String {
        guard_path_does_not_exist(self.path.clone());
        let type_name = std::any::type_name::<E>()
            .split("::")
            .last()
            .unwrap_or_default();
        let file_type = if self.encode { "jsondb" } else { "json" };
        if pluralize {
            format!("{}/{}s.{}", self.path, type_name, file_type)
        } else {
            format!("{}/{}.{}", self.path, type_name, file_type)
        }
    }
}

impl Default for JsonDatabase {
    fn default() -> Self {
        Self::new_with_path(DEFAULT_FILE_PATH)
    }
}

impl Database for JsonDatabase {
    fn transaction(&self, func: impl FnOnce()) {
        let guard_result = self.transaction_mutex.lock();
        let _guard = get_lock(guard_result);
        func();
    }

    fn drop<E>(&self, pluralize: bool)
    where
        E: DeserializeOwned,
    {
        let guard_result = self.fs_mutex.lock();
        let _guard = get_lock(guard_result);
        let path = self.path_to_entity::<E>(pluralize);
        let path_obj = std::path::Path::new(&path);
        if path_obj.exists() {
            std::fs::remove_file(path_obj).expect("Unable to drop");
        }
    }
}

impl DatabaseOps for JsonDatabase {
    fn get_one<E>(&self) -> Result<Option<E>>
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>(false);
        json_io::load_json(path, self.encode)
    }

    fn get_all<E>(&self) -> Result<Vec<E>>
    where
        E: DeserializeOwned,
    {
        let path = self.path_to_entity::<E>(true);
        json_io::load_json_vec::<E>(path, self.encode)
    }

    fn save<E>(&self, entity: E) -> Result<()>
    where
        E: Serialize + DeserializeOwned,
    {
        let guard_result = self.fs_mutex.lock();
        let _guard = get_lock(guard_result);
        let path = self.path_to_entity::<E>(false);
        json_io::save_json(path, entity, self.encode)
    }

    fn push<E>(&self, entity: E) -> Result<()>
    where
        E: Serialize + DeserializeOwned,
    {
        let guard_result = self.fs_mutex.lock();
        let _guard = get_lock(guard_result);
        let mut all = self.get_all::<E>()?;
        all.push(entity);

        let path = self.path_to_entity::<E>(true);
        json_io::save_json(path, all, self.encode)
    }

    fn push_batch<E>(&self, mut entities: Vec<E>) -> Result<()>
    where
        E: Serialize + DeserializeOwned,
    {
        let guard_result = self.fs_mutex.lock();
        let _guard = get_lock(guard_result);
        let mut all = self.get_all::<E>()?;
        all.append(&mut entities);

        let path = self.path_to_entity::<E>(true);
        json_io::save_json(path, all, self.encode)
    }
}
