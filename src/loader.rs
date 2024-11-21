use std::path::Path;

use windows::{
    core::{Error, PCWSTR},
    Win32::{
        Foundation::{
            CloseHandle, GENERIC_ALL, HANDLE, NTSTATUS, STATUS_ALREADY_REGISTERED,
            STATUS_INVALID_PARAMETER, STATUS_SUCCESS, STATUS_UNSUCCESSFUL, UNICODE_STRING,
        },
        Storage::FileSystem::{CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_NONE, OPEN_EXISTING},
        System::{
            Registry::{HKEY_LOCAL_MACHINE, REG_DWORD, REG_EXPAND_SZ},
            IO::DeviceIoControl,
        },
    },
};

use crate::{
    registry::{create_registry_key, delete_registry_key, registry_key_exists, set_registry_value},
    string::{str_to_unicode_string, string_to_u16_bytes2},
};

#[repr(C)]
#[derive(Debug)]
pub struct Communication<'a, T, U> {
    pub code: u32,
    pub input: &'a T,
    pub output: &'a mut U,
}

pub struct DriverLoader {}

extern "system" {
    fn RtlAdjustPrivilege(privilege: u64, enable: u8, client: u8, was_enabled: *mut u8)
        -> NTSTATUS;
    fn NtLoadDriver(service_name: *mut UNICODE_STRING) -> NTSTATUS;
    fn NtUnloadDriver(service_name: *mut UNICODE_STRING) -> NTSTATUS;
}

pub fn open_file(symbolic_link_path: &str) -> windows::core::Result<HANDLE> {
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
        )
    };

    if handle.is_err() {
        return Err(handle.err().unwrap());
    }

    Ok(handle.unwrap())
}

pub fn adjust_privilege() -> Result<(), NTSTATUS> {
    let mut was_enabled: u8 = 0;
    let r = unsafe { RtlAdjustPrivilege(10, 1, 0, &mut was_enabled as _) };
    if r == STATUS_SUCCESS {
        return Ok(());
    }

    return Err(r);
}

impl DriverLoader {
    pub fn new() -> Self {
        Self {}
    }

    // service load driver
    pub fn srv_load(&self, file: &str) -> Result<(), NTSTATUS> {
        let image_path = format!("\\\\?\\{}", file);
        match std::fs::metadata(image_path) {
            Ok(meta) => println!("Driver exists, size: {}", meta.len()),
            Err(err) => {
                println!("Failed to access driver file: {}", err);
                return Err(STATUS_UNSUCCESSFUL);
            }
        }

        // 1
        adjust_privilege()?;

        // 2
        let file_name = Path::new(file).file_stem().and_then(|stem| stem.to_str());
        if file_name.is_none() {
            return Err(STATUS_INVALID_PARAMETER);
        }

        let app_sub_key = format!(
            "System\\CurrentControlSet\\Services\\{}",
            file_name.unwrap()
        );
        let driver_sub_key = format!(
            "\\Registry\\Machine\\System\\CurrentControlSet\\Services\\{}",
            file_name.unwrap()
        );

        if registry_key_exists(HKEY_LOCAL_MACHINE, &app_sub_key) {
            return Err(STATUS_ALREADY_REGISTERED);
        }

        {
            let hkey = create_registry_key(HKEY_LOCAL_MACHINE, &app_sub_key)
                .map_err(|_| STATUS_UNSUCCESSFUL)?;

            let image_path = format!("\\??\\{}", file);

            set_registry_value(
                &hkey,
                "ImagePath",
                REG_EXPAND_SZ,
                string_to_u16_bytes2(&image_path).as_slice(),
            )
            .map_err(|_| STATUS_UNSUCCESSFUL)?;

            set_registry_value(&hkey, "Type", REG_DWORD, &1u32.to_le_bytes())
                .map_err(|_| STATUS_UNSUCCESSFUL)?;

            set_registry_value(&hkey, "Start", REG_DWORD, &4u32.to_le_bytes())
                .map_err(|_| STATUS_UNSUCCESSFUL)?;
        }

        // 3
        let mut p = str_to_unicode_string(&driver_sub_key);

        let r = unsafe { NtLoadDriver(p.as_ptr()) };
        if r != STATUS_SUCCESS {
            return Err(r);
        }

        return Ok(());
    }

    // service load driver
    pub fn srv_unload(&self, file: &str) -> Result<(), NTSTATUS> {
        let image_path = format!("\\\\?\\{}", file);
        match std::fs::metadata(image_path) {
            Ok(meta) => println!("Driver exists, size: {}", meta.len()),
            Err(err) => {
                println!("Failed to access driver file: {}", err);
                return Err(STATUS_UNSUCCESSFUL);
            }
        }

        // 1
        adjust_privilege()?;

        // 2
        let file_name = Path::new(file).file_stem().and_then(|stem| stem.to_str());
        if file_name.is_none() {
            return Err(STATUS_INVALID_PARAMETER);
        }

        let app_sub_key = format!(
            "System\\CurrentControlSet\\Services\\{}",
            file_name.unwrap()
        );
        let driver_sub_key = format!(
            "\\Registry\\Machine\\System\\CurrentControlSet\\Services\\{}",
            file_name.unwrap()
        );

        if !registry_key_exists(HKEY_LOCAL_MACHINE, &app_sub_key) {
            return Err(STATUS_UNSUCCESSFUL);
        }

        let mut p = str_to_unicode_string(&driver_sub_key);
        let r = unsafe { NtUnloadDriver(p.as_ptr()) };
        if r != STATUS_SUCCESS {
            return Err(r);
        }

        // 3
        let r = delete_registry_key(HKEY_LOCAL_MACHINE, &app_sub_key);
        if r.is_err() {
            return Err(STATUS_UNSUCCESSFUL);
        }

        return Ok(());
    }

    // mapping memory load driver
    // pub fn mapping_load(&self, file: &str) {}

    pub fn io_ctl<T, U>(
        &self,
        symbol_link: &str,
        cc: &mut Communication<T, U>,
    ) -> Result<(), Error> {
        let handle = open_file(symbol_link);
        if handle.is_err() {
            return Err(handle.err().unwrap());
        }

        let handle = handle.unwrap();

        let mut return_byte: u32 = 0;

        let r = unsafe {
            DeviceIoControl(
                handle,
                cc.code,
                Some(cc.input as *const _ as _),
                core::mem::size_of::<T>() as _,
                Some(cc.output as *mut _ as _),
                core::mem::size_of::<U>() as _,
                Some(&mut return_byte as *mut _),
                None,
            )
        };

        if r.is_err() {
            return Err(r.err().unwrap());
        }

        unsafe {
            let _ = CloseHandle(handle);
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_driver_load() {}
}
