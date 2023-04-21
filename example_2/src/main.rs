use cli::Cli;
use errors::Result;

mod block;
mod blockchain;
mod errors;
mod cli;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    
    Ok(())
}
