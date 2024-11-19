extern crate alloc;

use core::{ffi::CStr, slice};

use alloc::{string::String, vec::Vec};
use windows::{core::PWSTR, Win32::Foundation::UNICODE_STRING};

pub struct SafeUnicodeString {
    pub unicode_string: UNICODE_STRING,
    _buffer: Box<[u16]>, // 持有缓冲区，保证生命周期
}

impl SafeUnicodeString {
    pub fn as_ptr(&mut self) -> *mut UNICODE_STRING {
        &mut self.unicode_string as _
    }

    pub fn as_ref(&self) -> &UNICODE_STRING {
        &self.unicode_string
    }
}

pub fn string_to_u16_slice(input: &str) -> Vec<u16> {
    let utf16_iter = input.encode_utf16();
    let utf16_vec: Vec<u16> = utf16_iter.collect();
    utf16_vec
}

pub fn string_to_u16_bytes2(s: &str) -> Vec<u8> {
    let mut u16_bytes = Vec::with_capacity(s.len() * 2);
    for c in s.chars() {
        let u16_value = c as u16;
        u16_bytes.push((u16_value & 0xFF) as u8);
        u16_bytes.push((u16_value >> 8) as u8);
    }
    u16_bytes
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
    let wide: Vec<u16> = s.encode_utf16().chain(Some(0)).collect(); // 包含 NULL 终止符
    let buffer = wide.into_boxed_slice();

    let unicode_string = UNICODE_STRING {
        Length: ((buffer.len() - 1) * 2) as u16,  // 不包含 NULL
        MaximumLength: (buffer.len() * 2) as u16, // 包含 NULL
        Buffer: PWSTR(buffer.as_ptr() as *mut _),
    };

    SafeUnicodeString {
        unicode_string,
        _buffer: buffer, // 持有缓冲区
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
