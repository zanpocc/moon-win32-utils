use windows::Win32::Foundation::{BOOLEAN, NTSTATUS, STATUS_SUCCESS};

use crate::{ntdll::{NtLoadDriver, NtUnloadDriver, RtlAdjustPrivilege}, string::str_to_unicode_string};


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