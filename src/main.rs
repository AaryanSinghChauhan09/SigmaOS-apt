use anyhow::Result;
use clap::Parser;
use apt::apt;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = apt::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = apt::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-apt operational");
    Ok(())
}
