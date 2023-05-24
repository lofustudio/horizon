use std::env;

fn main() {
    // https://kazlauskas.me/entries/writing-proper-buildrs-scripts
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("android") => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
            println!("cargo:rustc-link-lib=c++_shared");
        }
        _ => {}
    }
    tauri_build::build()
}

#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
