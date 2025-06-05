use rand::Rng;
use windows::Win32::{Foundation::MAX_PATH, Storage::FileSystem::GetTempPathW};

use crate::string::u16_slice_to_string;

pub fn get_temp_path() -> String {
    let mut buffer: [u16; MAX_PATH as _] = [0; MAX_PATH as _];
    let r = unsafe { GetTempPathW(Some(&mut buffer as _)) };
    if r == 0 || r > MAX_PATH {
        panic!("GetTempPath Error");
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