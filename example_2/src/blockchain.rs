#![allow(unused)]
use crate::block::Block;
use crate::errors::Result;
const TARGET_HEXT: usize = 4;

#[derive(Debug, Clone)]
pub struct BlockChain{
    current_hash: String,
    db: sled::Db,
}

pub struct BlockchainIter<'a>{
    current_hash: String,
    bc: &'a BlockChain,
}

impl BlockChain {
    pub fn new() -> Result<BlockChain> {
        let db = sled::open("data/blocks")?;
        match db.get("LAST")?{
            Some(hash) => {
                let lasthash = String::from_utf8(hash.to_vec())?;
                Ok(BlockChain{
                    current_hash: lasthash,
                    db,
                })
            },
            None => {
                let block = Block::new_genesis_block();
                db.insert(block.get_hash(), bincode::serialize(&block)?)?;
                db.insert("LAST", block.get_hash().as_bytes())?;
                let bc = BlockChain{
                    current_hash: block.get_hash(),
                    db
                };
                bc.db.flush()?;
                Ok(bc)
            }
        }
    }
    
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let lasthash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(data, String::from_utf8(lasthash.to_vec())?, TARGET_HEXT)?;
        self.db.insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes());
        self.current_hash = new_block.get_hash();
        Ok(())
    }
    
    pub fn iter(&self) -> BlockchainIter{
        BlockchainIter { 
            current_hash: self.current_hash.clone(), 
            bc: &self
        }
    }
}

impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash){
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    }
                    else{
                        None
                    }
                },
                None => None
            }
        }
        None
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_blockchain(){
        let mut chain = BlockChain::new().unwrap();
        chain.add_block("data1".to_string());
        for block in chain.iter() {
            println!("{:#?}", block);
        }
        println!("current size: {}", chain.iter().count());
    }
}