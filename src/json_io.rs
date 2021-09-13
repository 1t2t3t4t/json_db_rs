use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{Cursor, ErrorKind, Result, Write};

pub(crate) fn load_json<E>(path: String, encode: bool) -> Result<Option<E>>
where
    E: DeserializeOwned,
{
    match std::fs::read(path) {
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
    let content = if encode {
        let content = serde_json::to_vec(&json)?;
        zstd::encode_all(Cursor::new(content), 0)?
    } else {
        serde_json::to_vec_pretty(&json)?
    };
    File::create(path)?.write_all(&content)
}
