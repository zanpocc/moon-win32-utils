extern crate alloc;

use core::{ffi::CStr, slice};

use alloc::{string::String, vec::Vec};
use windows::{core::PWSTR, Win32::Foundation::UNICODE_STRING};

pub struct SafeUnicodeString {
    pub unicode_string: UNICODE_STRING,
    _buffer: Box<[u16]>,
}

impl SafeUnicodeString {
    pub fn as_ptr(&mut self) -> *mut UNICODE_STRING {
        &mut self.unicode_string as _
    }

    pub fn as_ref(&self) -> &UNICODE_STRING {
        &self.unicode_string
    }
}

fn u16_to_u8_le(u16_vec: &[u16]) -> Vec<u8> {
    let mut result = Vec::with_capacity(u16_vec.len() * 2);
    for &num in u16_vec {
        result.extend(num.to_le_bytes()); 
    }
    result
}

pub fn string_to_u16_slice(input: &str) -> Vec<u16> {
    let utf16_iter = input.encode_utf16();
    let utf16_vec: Vec<u16> = utf16_iter.collect();
    utf16_vec
}

pub fn string_to_u16_bytes2(s: &str,cstr_end: bool) -> Vec<u8> {
    let mut utf16_vec: Vec<u16> = s.encode_utf16().collect();
    if cstr_end {
        utf16_vec.push(0);
    }
    
    u16_to_u8_le(&utf16_vec)
}

pub fn u16_slice_to_unicode_string(s: &[u16]) -> UNICODE_STRING {
    let len = s.len();

    let n = if len > 0 && s[len - 1] == 0 {
        len - 1
    } else {
        len
    };

    UNICODE_STRING {
        Length: (n * 2) as u16,
        MaximumLength: (len * 2) as u16,
        Buffer: PWSTR(s.as_ptr() as _),
    }
}

pub fn u16_slice_to_string(s: &[u16]) -> String {
    match String::from_utf16(s) {
        Ok(s) => return s,
        Err(_) => {
            println!("from utf16 error");
        }
    }

    String::new()
}

pub fn str_to_unicode_string(s: &str) -> SafeUnicodeString {
    let wide: Vec<u16> = s.encode_utf16().chain(Some(0)).collect();
    let buffer = wide.into_boxed_slice();

    let unicode_string = UNICODE_STRING {
        Length: ((buffer.len() - 1) * 2) as u16,
        MaximumLength: (buffer.len() * 2) as u16,
        Buffer: PWSTR(buffer.as_ptr() as *mut _),
    };

    SafeUnicodeString {
        unicode_string,
        _buffer: buffer,
    }
}

pub fn unicode_string_to_string(s: &UNICODE_STRING) -> String {
    let buffer_slice = unsafe { slice::from_raw_parts(s.Buffer.as_ptr(), s.Length as usize / 2) };
    u16_slice_to_string(buffer_slice)
}

pub fn cstr_to_rust_str(cstr_ptr: *mut u8) -> String {
    unsafe {
        let c_str = CStr::from_ptr(cstr_ptr as _);
        c_str.to_string_lossy().into_owned()
    }
}
