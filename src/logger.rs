use env_logger::{Builder, Env};

pub fn setup_logger() {
    let env = Env::default().filter_or("RUST_LOG", "info");
    Builder::from_env(env).init();
    log::debug!("configured logger");
}
