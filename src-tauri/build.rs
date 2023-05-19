fn main() {
    #[cfg(target_os = "android")]
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=c++_shared");
    }
    tauri_build::build()
}

#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
