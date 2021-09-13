use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{Cursor, Error, ErrorKind, Result, Write};

pub(crate) fn load_json<E>(path: String, encode: bool) -> Result<Option<E>>
where
    E: DeserializeOwned,
{
    let file_content = std::fs::read(path);
    match file_content {
        Ok(mut file_content) => {
            if encode {
                let file_cursor = Cursor::new(file_content);
                file_content = zstd::decode_all(file_cursor)?;
            }
            let json_content = String::from_utf8(file_content).unwrap();
            let res = if json_content.is_empty() {
                None
            } else {
                let json_from_str =
                    serde_json::from_str::<E>(&json_content).map_err(std::io::Error::from)?;
                Some(json_from_str)
            };
            Ok(res)
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(None),
            _ => Err(e),
        },
    }
}

pub(crate) fn load_json_vec<E>(path: String, encode: bool) -> Result<Vec<E>>
where
    E: DeserializeOwned,
{
    load_json::<Vec<E>>(path, encode).map(|v| v.unwrap_or_default())
}

pub(crate) fn save_json<E>(path: String, json: E, encode: bool) -> Result<()>
where
    E: Serialize,
{
    let mut content = serde_json::to_string(&json)
        .unwrap_or_default()
        .as_bytes()
        .to_vec();
    if encode {
        content = zstd::encode_all(Cursor::new(content), 0)?;
    }
    File::create(path.clone())?.write_all(&content)
}
