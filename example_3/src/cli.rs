#![allow(unused)]
use std::process::exit;

use clap::{arg, Command};

use crate::blockchain::BlockChain;
use crate::errors::Result;
use crate::transaction::Transaction;

pub struct Cli{}

impl Cli{
    pub fn new() -> Result<Cli> {
        Ok(Cli{})
    }
    
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("rust-in-blockchain-demo")
            .version("0.1")
            .author("xxxxx")
            .about("a simple blockchain for learning")
            .subcommand(Command::new("printchain")
                                .about("print all the chain blocks"))
            .subcommand(Command::new("getbalance")
                                .about("get balance in the blockchain")
                                .arg(arg!(<ADDRESS> "'The Address it get balance for'")))
            .subcommand(Command::new("create")
                                .about("Create new blockchain")
                                .arg(arg!(<ADDRESS> "'The address to send gensis block reqward to' ")))               
            .subcommand(Command::new("send")
                                .about("send in the blockchain")
                                .arg(arg!(<FROM>" 'Source wallet address'"))
                                .arg(arg!(<TO>" 'Destination wallet address'"))
                                .arg(arg!(<AMOUNT>" 'Destination wallet address'")))
            .get_matches();
        
        if let Some(_) = matches.subcommand_matches("printchain"){
            self.printchain();
        }
        
        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                BlockChain::create_blockchain(address.clone())?;
                println!("create blockchain");
            }
        }
        
        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = BlockChain::new()?;
                let utxos = bc.find_UTXO(&address);
                let mut balance = 0;
                for out in utxos{
                    balance += out.value;
                }
                println!("Balance of {}: {}", address, balance);
            }
        }
        
        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            }else{
                println!("from not supply!: usage");
                exit(1)
            };

            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            }else{
                println!("to not supply!: usage");
                exit(1)
            };

            let amount = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse::<i32>()?
            }else{
                println!("amoutn not supply!: usage");
                exit(1)
            };
            let mut bc = BlockChain::new()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;
            bc.add_block(vec![tx])?;
            println!("success!");
        }
        
        Ok(())
    }
    
    fn printchain(&self) -> Result<()> {
        let bc = BlockChain::new()?;
        for block in bc.iter() {
            println!("{:#?}", block);
        }
        println!("total: {}", bc.iter().count());
        Ok(())
    }

}