fn main() {
    // If we're using std library, link in KallistiOS's libpthread addon
    if std::env::var("CARGO_FEATURE_STD").is_ok() {
        println!("cargo:rustc-link-lib=pthread");
    }

    // Include library paths from KallistiOS environment
    let kos_ldflags = std::env::var("KOS_LDFLAGS")
        .expect("Missing $KOS_LDFLAGS -- KallistiOS environment not sourced!");
    for lib_path in kos_ldflags
        .split_whitespace()
        .filter(|x| x.starts_with("-L"))
    {
        println!("cargo:rustc-link-search=native={}", lib_path);
    }
}
