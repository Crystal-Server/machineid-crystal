use crate::errors::HWIDError;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use uuid::Uuid;

pub(crate) fn file_token(path: &str) -> Result<String, HWIDError> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    if content.is_empty() {
        let uuid = Uuid::new_v4().to_string();
        file.write_all(uuid.as_bytes()).unwrap();
        content = uuid.to_string();
    }
    Ok(content)
}
