use crate::block::Block;
use crate::errors::Result;
const TARGET_HEXT: usize = 4;

#[derive(Debug)]
pub struct BlockChain{
    blocks: Vec<Block>
}

impl BlockChain {
    pub fn new() -> BlockChain{
        BlockChain { 
            blocks: vec![Block::new_genesis_block()]
        }
    }
    
    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.get_hash(), TARGET_HEXT)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_blockchain(){
        let mut chain = BlockChain::new();
        chain.add_block("data1".to_string()).unwrap();
        dbg!(chain);
    }
}