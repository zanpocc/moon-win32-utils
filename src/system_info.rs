use std::ffi::CStr;

use windows::Win32::{Foundation::{NTSTATUS, STATUS_INFO_LENGTH_MISMATCH, STATUS_NO_MEMORY, STATUS_SUCCESS}, System::Memory::{VirtualAlloc, VirtualFree, MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE}};

use crate::ntdll::NtQuerySystemInformation;

#[repr(C)]
struct RtlProcessModuleInformation {
    section: *mut std::ffi::c_void,
    mapped_base: *mut std::ffi::c_void,
    image_base: *mut std::ffi::c_void,
    image_size: u32,
    flags: u32,
    load_order_index: u16,
    init_order_index: u16,
    load_count: u16,
    offset_to_file_name: u16,
    full_path_name: [u8; 256],
}

#[repr(C)]
struct RtlProcessModules {
    number_of_modules: u32,
    modules: [RtlProcessModuleInformation; 1], // 实际大小由 number_of_modules 决定
}

const SYSTEM_MODULE_INFORMATION: u32 = 11;

pub fn get_kernel_module_address(module_name: &str) -> Result<u64,NTSTATUS> {
    let mut buffer: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut buffer_size: u32 = 0;
    let mut status: NTSTATUS;

    // 首次调用获取所需缓冲区大小
    unsafe {
        status = NtQuerySystemInformation(
            SYSTEM_MODULE_INFORMATION as _,
            core::ptr::null_mut(),
            buffer_size,
            &mut buffer_size as *mut _ as _,
        );
    }

    // 循环分配缓冲区直到大小合适
    while status == STATUS_INFO_LENGTH_MISMATCH {
        if !buffer.is_null() {
            unsafe {
                let _ = VirtualFree(buffer, 0, MEM_RELEASE);
            }
            buffer = std::ptr::null_mut();
        }

        // 分配新缓冲区
        buffer = unsafe {
            VirtualAlloc(
                None,
                buffer_size as usize,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            )
        };

        if buffer.is_null() {
            return Err(STATUS_NO_MEMORY);
        }

        // 再次查询系统信息
        unsafe {
            status = NtQuerySystemInformation(
                SYSTEM_MODULE_INFORMATION,
                buffer as _,
                buffer_size,
                &mut buffer_size as *mut _ as _,
            );
        }
    }

    // 检查最终状态
    if status != STATUS_SUCCESS {
        if !buffer.is_null() {
            unsafe {
                let _ = VirtualFree(buffer, 0, MEM_RELEASE);
            }
        }
        return Err(status);
    }

    // 解析模块信息
    let result = unsafe {
        let modules_info = &*(buffer as *const RtlProcessModules);
        let mut found_address = 0u64;

        for i in 0..modules_info.number_of_modules {
            let module = &modules_info.modules.as_ptr().add(i as usize).read();
            
            // 提取模块名（ASCII格式）
            let full_path = &module.full_path_name;
            let offset = module.offset_to_file_name as usize;
            let name_slice = &full_path[offset..];
            
            // println!("{},{:X},{:X}",cstr_to_rust_str(name_slice.as_ptr() as _),module.image_base as u64,module.mapped_base as u64);
            
            // 转换为C风格字符串
            if let Ok(cstr) = CStr::from_bytes_until_nul(name_slice) {
                // 比较模块名（不区分大小写）
                if cstr.to_string_lossy().eq_ignore_ascii_case(module_name) {
                    found_address = module.image_base as u64;
                    break;
                }
            }
        }
        
        found_address
    };

    // 释放缓冲区
    if !buffer.is_null() {
        unsafe {
            let _ = VirtualFree(buffer, 0, MEM_RELEASE);
        }
    }

    Ok(result)
}


pub mod tests{
    use crate::system_info::get_kernel_module_address;

    #[test]
    fn test_get_kernel_module_address_valid_path() {
        let module_name = "win32k.sys";
        let result = get_kernel_module_address(module_name);
        assert!(result.is_ok());
        let module = result.unwrap();
        assert!(module != 0);
    }

    #[test]
    fn test_get_kernel_module_address_invalid_path() {
        let module_name = "nonexistent";
        let result = get_kernel_module_address(module_name);
        assert!(result.is_err());
    }

}