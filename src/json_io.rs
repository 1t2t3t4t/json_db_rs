use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{Cursor, Write};

pub(crate) fn load_json<E>(path: String, encode: bool) -> Option<E>
where
    E: DeserializeOwned,
{
    let mut file_content = std::fs::read(path).unwrap_or_default();
    if encode {
        let file_cursor = Cursor::new(file_content);
        file_content = zstd::decode_all(file_cursor).unwrap_or_default();
    }
    let json_content = String::from_utf8(file_content).unwrap_or_default();
    if json_content.is_empty() {
        None
    } else {
        Some(serde_json::from_str::<E>(&json_content).expect("Failed to format json"))
    }
}

pub(crate) fn load_json_vec<E>(path: String, encode: bool) -> Vec<E>
where
    E: DeserializeOwned,
{
    load_json::<Vec<E>>(path, encode).unwrap_or_default()
}

pub(crate) fn save_json<E>(path: String, json: E, encode: bool)
where
    E: Serialize,
{
    let mut content = serde_json::to_string(&json)
        .unwrap_or_default()
        .as_bytes()
        .to_vec();
    if encode {
        content = zstd::encode_all(Cursor::new(content), 0).unwrap();
    }
    let mut file = File::create(path.clone()).unwrap();
    file.write_all(&content)
        .expect(&format!("Unable to write to path {}", path));
}
