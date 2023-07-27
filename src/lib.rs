#[cfg(windows)]
use std::{env::args, io::ErrorKind, path::Path};

#[cfg(windows)]
use cookie::{get_raw_cookies::take_cookies_from_db, handle_raw_cookie::handle_chrome_cookies};

#[cfg(windows)]
use tokio::io;

#[cfg(windows)]
mod cookie;
#[cfg(windows)]
mod encrypted_key;
#[cfg(windows)]
mod marco;

#[cfg(not(windows))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Err("Platform does not supported. Only windows".into())
}

#[cfg(windows)]
#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    let mut path_to_folder: String = String::from("");
    if let Some(path) = args.iter().nth(1) {
        if Path::is_dir(Path::new(path)) {
            path_to_folder = path.to_string()
        }
    }

    let os_username = whoami::username();
    let chrome_cookies = match take_cookies_from_db(os_username.clone()).await {
        Ok(cookies) => cookies,
        Err(e) => {
            println!("HELP:\nTo solve this problem, you need to close your chrome browser\n");
            return Err(tokio::io::Error::new(ErrorKind::Other, format!("{}", e)));
        }
    };

    tokio::spawn(async move {
        match handle_chrome_cookies(os_username.clone(), chrome_cookies, path_to_folder.as_str()).await {
            Ok(()) => {
            return Ok(())
        },
        Err(e) => {
            println!("{}", e);
            return Err(tokio::io::Error::new(ErrorKind::Other, format!("{}", e)));
        }
        }
    })
    .await
    .unwrap()
}
