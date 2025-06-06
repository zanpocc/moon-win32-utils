use windows::{core::Error, Win32::{Foundation::{BOOLEAN, HMODULE, NTSTATUS, STATUS_SUCCESS}, System::LibraryLoader::LoadLibraryW}};

use crate::{ntdll::{NtLoadDriver, NtUnloadDriver, RtlAdjustPrivilege}, string::{str_to_unicode_string, string_to_u16_slice_u16 }};

pub const SE_LOAD_DRIVER_PRIVILEGE: u32 = 10; // SE_LOAD_DRIVER_PRIVILEGE constant value
pub const SE_DEBUG_PRIVILEGE: u32 = 20;       // SE_DEBUG_PRIVILEGE constant value    

pub fn adjust_privilege(privilege: u32) -> Result<(), NTSTATUS> {
    let mut was_enabled: BOOLEAN = BOOLEAN(0);
    let r = unsafe {
        RtlAdjustPrivilege(
            privilege,
            BOOLEAN(1),
            BOOLEAN(0),
            &mut was_enabled as *mut BOOLEAN,
        )
    };
    if r != STATUS_SUCCESS {
        return Err(r);
    }

    return Ok(());
}

pub fn nt_load_driver(service_name: &str) -> Result<(), NTSTATUS>{
    let driver_sub_key = format!(
        "\\Registry\\Machine\\System\\CurrentControlSet\\Services\\{}",
        service_name
    );

    let mut p = str_to_unicode_string(&driver_sub_key);

    let r = unsafe { NtLoadDriver(p.as_ptr()) };
    if r != STATUS_SUCCESS {
        return Err(r);
    }

    Ok(())
}

pub fn nt_unload_driver(service_name: &str) -> Result<(), NTSTATUS>{
    let driver_sub_key = format!(
        "\\Registry\\Machine\\System\\CurrentControlSet\\Services\\{}",
        service_name
    );

    let mut p = str_to_unicode_string(&driver_sub_key);

    let r = unsafe { NtUnloadDriver(p.as_ptr()) };
    if r != STATUS_SUCCESS {
        return Err(r);
    }

    Ok(())
}

pub fn load_library(dll_path: &str) -> Result<HMODULE, Error>{
    let dll_path = string_to_u16_slice_u16(dll_path, true);
    let dll_path_pcwstr = windows::core::PCWSTR(dll_path.as_ptr());
    unsafe {  LoadLibraryW(dll_path_pcwstr) }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_library_valid_path() {
        let dll_path = "C:\\Windows\\System32\\kernel32.dll";
        let result = load_library(dll_path);
        assert!(result.is_ok());
        let module: HMODULE = result.unwrap();
        assert!(!module.is_invalid());
    }

    #[test]
    fn test_load_library_invalid_path() {
        let dll_path = "C:\\Invalid\\Path\\nonexistent.dll";
        let result = load_library(dll_path);
        assert!(result.is_err());
    }
}
