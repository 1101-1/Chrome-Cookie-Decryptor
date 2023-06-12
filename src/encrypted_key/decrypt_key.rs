use std::ptr::null_mut;

#[cfg(windows)]
use winapi::um::{
    dpapi::CryptUnprotectData, winbase::LocalFree, wincrypt::CRYPTOAPI_BLOB, winnt::HANDLE,
};

#[cfg(windows)]
pub async fn decrypt_data_key(mut unencoded_key: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut data_in = CRYPTOAPI_BLOB {
        cbData: unencoded_key.len() as u32,
        pbData: unencoded_key.as_mut_ptr(),
    };
    let mut data_out = CRYPTOAPI_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };
    unsafe {
        CryptUnprotectData(
            &mut data_in,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            &mut data_out,
        );

        let bytes = Vec::from_raw_parts(
            data_out.pbData,
            data_out.cbData as usize,
            data_out.cbData as usize,
        );
        LocalFree(data_out.pbData as HANDLE);
        Ok(bytes)
    }
}
