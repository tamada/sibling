pub(super) fn init() {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    }
    let _ = env_logger::builder().is_test(true).try_init();
}
