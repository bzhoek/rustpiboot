use rustpiboot::{boot, Options};

fn main() {
    env_logger::Builder::from_default_env()
      .filter_level(log::LevelFilter::Debug)
      .init();
    boot(Options::default()).unwrap();
}
