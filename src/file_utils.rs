use rand::Rng;
use windows::{core::PCWSTR, Win32::{Foundation::{GENERIC_ALL, HANDLE, MAX_PATH}, Storage::FileSystem::{CreateFileW, GetTempPathW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING}, System::SystemInformation::GetSystemDirectoryW}};

use crate::{string::u16_slice_to_string, wrapper::handle::Handle};

pub fn open_file(symbolic_link_path: &str) -> Result<Handle, windows::core::Error> {
    let wide_path: Vec<u16> = symbolic_link_path.encode_utf16().chain(Some(0)).collect();

    let handle = unsafe {
        CreateFileW(
            PCWSTR(wide_path.as_ptr()),
            GENERIC_ALL.0,
            FILE_SHARE_NONE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE::default(),
        )?
    };

    Ok(Handle::from_raw(handle))
}

pub fn get_temp_path() -> String {
    let mut buffer: [u16; MAX_PATH as _] = [0; MAX_PATH as _];
    let r = unsafe { GetTempPathW(Some(&mut buffer as _)) };
    if r == 0 || r > MAX_PATH {
        panic!("GetTempPath Error");
    }

    let temp = u16_slice_to_string(&buffer[0..r as usize]);
    temp
}

pub fn get_system_directory() -> String {
    let mut buffer: [u16; MAX_PATH as _] = [0; MAX_PATH as _];
    let r = unsafe { GetSystemDirectoryW(Some(&mut buffer as _)) };
    if r == 0 || r > MAX_PATH {
        panic!("GetSystemDirectoryW Error");
    }

    let temp = u16_slice_to_string(&buffer[0..r as usize]);
    temp
}


pub fn random_file_name(len: Option<u8>) -> String {
    let arr = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut rng = rand::thread_rng();
    

    let name_len = len.unwrap_or_else(||15);
    
    let mut file_name = String::new();
    for _ in 0..name_len{
        let random_index = rng.gen_range(0..arr.len());
        file_name.push(arr[random_index]);
    }

    return file_name;
}