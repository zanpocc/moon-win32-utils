use windows::Win32::{Foundation::{CloseHandle, HANDLE}, Security::{GetTokenInformation, TokenElevationType, TokenElevationTypeFull, TOKEN_ELEVATION_TYPE, TOKEN_QUERY}, System::Threading::{GetCurrentProcess, OpenProcessToken}};


pub fn is_admin_process() -> Result<bool, windows::core::Error> {
    let mut token_handle = HANDLE::default();

    // 1. 打开当前进程令牌
    unsafe {
        OpenProcessToken(
            GetCurrentProcess(),  // 当前进程句柄
            TOKEN_QUERY,          // 请求查询权限
            &mut token_handle,    // 返回的令牌句柄
        )?;
    }

    // 2. 查询令牌提升类型
    let mut elevation_type = TOKEN_ELEVATION_TYPE::default();
    let mut return_length = 0u32;

    let result = unsafe {
        GetTokenInformation(
            token_handle,
            TokenElevationType,
            Some(&mut elevation_type as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION_TYPE>() as u32,
            &mut return_length,
        )
    };

    // 3. 确保关闭令牌句柄
    unsafe { let _ = CloseHandle(token_handle); };

    // 4. 处理查询结果
    if result.is_ok() {
        Ok(elevation_type == TokenElevationTypeFull)
    } else {
        Err(windows::core::Error::from_win32())
    }
}


// /// 以管理员权限重启程序
// pub fn restart_as_admin() -> Result<(), Error> {
//     println!("{:?}",std::env::current_exe()?.to_str());
    
//     let mut exec_info = SHELLEXECUTEINFOA {
//         cbSize: std::mem::size_of::<SHELLEXECUTEINFOA>() as u32,
//         fMask: SEE_MASK_NOCLOSEPROCESS | SEE_MASK_DEFAULT,
//         hwnd: HWND(core::ptr::null_mut()),
//         lpVerb: PCSTR("runas\0".as_ptr()), // 关键提权指令
//         lpFile: PCSTR(std::env::current_exe()?.to_str().unwrap().as_ptr() as _),
//         lpParameters: PCSTR::null(),
//         lpDirectory: PCSTR::null(),
//         nShow: SW_SHOW.0,
//         hInstApp: HINSTANCE(core::ptr::null_mut()),
//         ..Default::default()
//     };

//     unsafe {
//         ShellExecuteExA(&mut exec_info)?;
//     }
    
//     Ok(())
// }