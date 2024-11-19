use moon_driver_loader::loader::DriverLoader;

fn main() {
    // let mut was_enabled: u8 = 0;
    // let r = unsafe { RtlAdjustPrivilege(10, 1, 0, &mut was_enabled as _) };
    // if r != STATUS_SUCCESS {
    //     println!("RtlAdjustPrivilege failed: {:#?}", r);
    // }

    // println!("Privilege adjustment result: {}", was_enabled);

    // let sub_key = r"\Registry\Machine\System\CurrentControlSet\Services\rust_driver";
    // println!("NtLoadDriver: {}", sub_key);
    // let mut p = str_to_unicode_string(sub_key);

    // print_unicode_string(p.as_ref());

    // let r = unsafe { NtLoadDriver(p.as_ptr()) };
    // if r != STATUS_SUCCESS {
    //     let r: u32 = unsafe { core::mem::transmute(r) };
    //     println!("NtLoadDriver failed: {:#X}", r);
    // }

    let loader = DriverLoader::new();

    let file_path = "C:\\Users\\Administrator\\Desktop\\rust_driver.sys";
    let image_path = r"\\?\C:\Users\Administrator\Desktop\rust_driver.sys";
    match std::fs::metadata(image_path) {
        Ok(meta) => println!("Driver exists, size: {}", meta.len()),
        Err(err) => println!("Failed to access driver file: {}", err),
    }

    let r = loader.srv_load(file_path);
    if r.is_err() {
        print!("error to load:{:?}", r.err().unwrap());
        return;
    }

    println!("success to load");

    let r = loader.srv_unload(&file_path);
    if r.is_err() {
        print!("error to unload:{:?}", r.err().unwrap());
        return;
    }

    println!("success to unload");
}
