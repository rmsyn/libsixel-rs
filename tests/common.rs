#[cfg(feature = "e2e-tests")]
pub fn init() {
    env_logger::try_init().ok();
}
