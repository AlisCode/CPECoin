use std::fmt;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use time::precise_time_ns;

/// Contains data for a transaction
pub struct Transaction {
    from: String,
    to: String,
    amount: f32,
}

/// Data of our blocks
pub struct BlockData {
    pub transactions: Vec<Transaction>,
}

/// Represents a block itself
pub struct Block {
    pub index: usize,
    pub data: BlockData,
    pub timestamp: u64,
    pub proof: u64,
    pub prev_hash: String,
    pub hash: String,
}

impl BlockData {
    pub fn new() -> Self {
        BlockData {
            transactions: Vec::new(),
        }
    }
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: f32) -> Self {
        Transaction {
            from: String::from(from),
            to: String::from(to),
            amount,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tx//{}//{}//{}", self.from, self.to, self.amount)
    }
}

impl fmt::Display for BlockData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result: fmt::Result = Ok(());
        self.transactions.iter().for_each(|a| {
            result = write!(f, "{}\n", a);
        });
        
        result
    }
}

impl Block {

    /// Creates the Genesis block (first ever block of a blockchain)
    pub fn genesis() -> Self {
        Block {
            index: 0,
            data: BlockData::new(),
            timestamp: precise_time_ns(),
            proof: 0,
            prev_hash: "".into(),
            hash: "".into(),
        }
    }

    /// Creates a block from a reference to the previous one in the blockchain
    pub fn new_from_prev(prev: &Block, data: BlockData) -> Self {
        Block {
            index: prev.index+1,
            data,
            timestamp: precise_time_ns(),
            proof: prev.proof,
            prev_hash: prev.hash.clone(),
            hash: "".into(),
        }
    }

    /// Computes the hash of a block 
    pub fn calc_hash(&mut self) {
        let mut hasher: Sha1 = Sha1::new();
        hasher.input_str(&format!("{}/{}/{}/{}", self.index, self.timestamp, self.data, self.prev_hash));
        self.hash = hasher.result_str();
    }
}

/// Checks if a Blockchain is valid or not
pub fn validate_chain(chain: Vec<Block>) -> bool {
    if chain.len() <= 1 { return true; }
    let mut to_return: bool = true;
    (0..chain.len() - 1).for_each(|i| {
        let old_block = &chain[i];
        let new_block = &chain[i+1];
        if !validate_proof(old_block.proof, new_block.proof) || old_block.hash != new_block.prev_hash {
            to_return = false;
        }
    });
    to_return
}

/// Check if a proof of work is valid or not, given the current and the old proof 
pub fn validate_proof(old: u64, new: u64) -> bool {
    new % 9 == 0 && new % old == 0
}