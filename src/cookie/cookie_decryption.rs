use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit,
};

// #[cfg(windows)]
pub async fn decrypt_cookie(key: Vec<u8>, encrypted_value: Vec<u8>) -> String {
    let iv = &encrypted_value[3..15];
    let encrypted_value = &encrypted_value[15..];

    let cipher = Aes256Gcm::new(&GenericArray::from_slice(&key));

    if let Ok(decrypted) = cipher.decrypt(GenericArray::from_slice(iv), encrypted_value) {
        if let Ok(decoded) = String::from_utf8(decrypted) {
            return decoded;
        }
    }

    return String::new();
}
