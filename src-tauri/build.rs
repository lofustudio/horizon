fn main() {
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=c++_shared");
    }
    tauri_build::build()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(u64::MAX))
    }
}
