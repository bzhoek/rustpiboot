use rustpiboot::{boot, Options};

fn main() {
    let level = env_logger::Env::default().default_filter_or("debug");
    env_logger::Builder::from_env(level).init();
    boot(Options::default()).unwrap();
}
