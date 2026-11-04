use sigmaos_apt::{Config, AptComponent, Component};
fn main() {
    env_logger::init();
    let mut c = AptComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS apt running");
    let _ = c.shutdown();
}
