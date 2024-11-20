use std::env;

use moon_driver_loader::loader::DriverLoader;

fn main() {
    let mut unload = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("unload")) {
        unload = true;
    }

    let loader = DriverLoader::new();
    let file_path = "C:\\Users\\Administrator\\Desktop\\rust_driver.sys";

    if unload {
        let r = loader.srv_unload(&file_path);
        if r.is_err() {
            print!("error to unload:{:?}", r.err().unwrap());
            return;
        }

        println!("success to unload");
    } else {
        let r = loader.srv_load(file_path);
        if r.is_err() {
            print!("error to load:{:?}", r.err().unwrap());
            return;
        }

        println!("success to load");
    }
}
