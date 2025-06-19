use std::ops::{Deref, DerefMut};

use windows::Win32::Foundation::{CloseHandle, HANDLE};


pub struct Handle {
    raw: HANDLE,
}

impl Default for Handle {
    fn default() -> Self {
        Self {
            raw: HANDLE::default(),
        }
    }
}

impl Handle {
    pub fn from_raw(raw: HANDLE) -> Self{
        Self { raw }
    }

    pub fn as_ptr(&mut self) -> *mut HANDLE {
        &mut self.raw as *mut _
    }

    pub fn as_raw(&mut self) -> HANDLE {
        self.raw
    }
}

impl Deref for Handle {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl DerefMut for Handle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        if !self.raw.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.raw);
            };
        }
    }
}
