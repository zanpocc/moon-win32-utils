use std::env;

use moon_driver_loader::loader::{Communication, DriverLoader};

macro_rules! CTL_CODE {
    ($DeviceType:expr, $Function:expr, $Method:expr, $Access:expr) => {
        ((($DeviceType as u32) << 16)
            | (($Access as u32) << 14)
            | (($Function as u32) << 2)
            | ($Method as u32))
    };
}

const FILE_DEVICE_UNKNOWN: u32 = 0x00000022;
const METHOD_BUFFERED: u32 = 0;
const IOCTL_DEVICE_IO_CONTROL_TEST: u32 =
    CTL_CODE!(FILE_DEVICE_UNKNOWN, 0x2000, METHOD_BUFFERED, 0);

#[repr(C)]
struct DeviceIoTestOut {
    length: u16,         // version
    maximum_length: u16, // vmx abort reason. vmx abort:vmexit fault
}

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
            println!("error to unload:{:?}", r.err().unwrap());
            return;
        }

        println!("success to unload");
    } else {
        let r = loader.srv_load(file_path);
        if r.is_err() {
            println!("error to load:{:?}", r.err().unwrap());
            return;
        }

        let input = DeviceIoTestOut {
            length: 3,
            maximum_length: 4,
        };
        let mut output = DeviceIoTestOut {
            length: 0,
            maximum_length: 0,
        };

        let mut cc = Communication {
            code: IOCTL_DEVICE_IO_CONTROL_TEST,
            input: &input,
            output: &mut output,
        };

        let r = loader.io_ctl("\\\\.\\20240703", &mut cc);
        if r.is_err() {
            println!("ioctl error:{}", r.err().unwrap());
            return;
        } else {
            println!("ioctl success:{},{}", output.length, output.maximum_length);
        }

        println!("success to load");
    }
}
