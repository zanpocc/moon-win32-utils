# Load WDM Driver

## Start

~~~shell
cargo add moon-driver-loader
~~~

### Simple exampel
~~~rust
pub fn main() {
    let loader = DriverLoader::new();
    let driver_file = "C:\\Users\\Administrator\\Desktop\\driver.sys";
    let r = loader.srv_load(driver_file);
    if r.is_ok() {
        println!("success to load");
        let r = loader.srv_unload(driver_file);
        if r.is_ok() {
            println!("success to unload")
        }
    }
}
~~~