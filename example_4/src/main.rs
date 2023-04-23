use cli::Cli;
use errors::Result;

mod block;
mod blockchain;
mod transaction;
mod errors;
mod cli;
mod tx;
mod wallet;

fn main() -> Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    
    Ok(())
}
