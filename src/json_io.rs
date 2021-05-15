use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{Cursor, Write};

pub(crate) fn load_json<E>(path: String) -> Vec<E>
where
    E: DeserializeOwned,
{
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

pub(crate) fn save_json<E>(path: String, json: Vec<E>)
where
    E: Serialize,
{
    let json_str = serde_json::to_string(&json).unwrap_or_default();
    let encoded = zstd::encode_all(json_str.as_bytes(), 0).unwrap();
    let mut file = File::create(path.clone()).unwrap();
    file.write_all(&encoded)
        .expect(&format!("Unable to write to path {}", path));
}
