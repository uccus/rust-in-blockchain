#![allow(unused)]
use clap::{arg, Command};

use crate::blockchain::BlockChain;
use crate::errors::Result;

pub struct Cli{
    bc: BlockChain
}

impl Cli{
    pub fn new() -> Result<Cli> {
        Ok(Cli{
            bc: BlockChain::new()?
        })
    }
    
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("rust-in-blockchain-demo")
            .version("0.1")
            .author("xxxxx")
            .about("a simple blockchain for learning")
            .subcommand(Command::new("printchain")
                                .about("print all the chain blocks"))
            .subcommand(Command::new("addblock")
                                .about("add a block in the blockchain")
                                .arg(arg!(<DATA>" 'the block data")))
            .get_matches();
        
        if let Some(ref matches) = matches.subcommand_matches("addblock"){
            if let Some(data) = matches.get_one::<String>("DATA"){
                self.addblock(String::from(data))?;
            }
            else{
                println!("Not printing testing lists...");
            }
        }
        
        if let Some(_) = matches.subcommand_matches("printchain"){
            self.print_chain();
        }

        Ok(())
    }

    fn addblock(&mut self, data: String) -> Result<()>{
        self.bc.add_block(data)
    }
    
    fn print_chain(&self) {
        for block in self.bc.iter() {
            println!("block: {:#?}", block);
        }
    }
}