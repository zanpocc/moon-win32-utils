use moon_win32_utils::{ privilege::is_admin_process, system_info::get_kernel_module_address};

pub fn main() {
    let module_name = "win32k.sys";
    let result = get_kernel_module_address(module_name);
    println!("win32k.sys base={:?}",result);

    println!("is_admin_process={:?}",is_admin_process());
}