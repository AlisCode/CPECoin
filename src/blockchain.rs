use crypto::digest::Digest;
use crypto::sha1::Sha1;
use serde_json::from_str;
use serde_json::to_string_pretty;
use std::fmt;
use std::fs::File;
use std::fs::remove_file;
use std::io::prelude::*;
use std::path::Path;
use time::get_time;

/// Contains data for a transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u32,
}

/// Data of our blocks
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockData {
    pub transactions: Vec<Transaction>,
}

/// Represents a block itself
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: usize,
    pub data: BlockData,
    pub timestamp: i64,
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
    pub fn new(from: &str, to: &str, amount: u32) -> Self {
        Transaction {
            from: String::from(from),
            to: String::from(to),
            amount,
        }
    }

    pub fn parse_from_data(input: String) -> Self {
        let splitted: Vec<&str> = input.split("//").map(|a| a.into()).collect();
        Transaction {
            from: splitted[1].into(),
            to: splitted[2].into(),
            amount: String::from(splitted[3]).parse::<u32>().unwrap(),
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
            timestamp: get_time().sec,
            proof: 1,
            prev_hash: "".into(),
            hash: "".into(),
        }
    }
    /// Creates a block from a reference to the previous one in the blockchain
    pub fn new_from_prev(prev_index: usize, prev_hash: String, data: BlockData) -> Self {
        Block {
            index: prev_index + 1,
            data,
            timestamp: get_time().sec,
            proof: 0,
            prev_hash,
            hash: "".into(),
        }
    }

    /// Computes the hash of a block
    pub fn calc_hash(&mut self) {
        let mut hasher: Sha1 = Sha1::new();
        hasher.input_str(&format!(
            "{}/{}/{}/{}",
            self.index, self.timestamp, self.data, self.prev_hash
        ));
        self.hash = hasher.result_str();
    }
}

/// Checks if a Blockchain is valid or not
pub fn validate_chain(chain: Vec<Block>) -> bool {
    if chain.len() <= 1 {
        return true;
    }
    let mut to_return: bool = true;
    (0..chain.len() - 1).for_each(|i| {
        let old_block = &chain[i];
        let new_block = &chain[i + 1];
        if !valid_proof(old_block.proof, new_block.proof) || old_block.hash != new_block.prev_hash {
            to_return = false;
        }
    });
    to_return
}

/// Loads the blockchain from a file
pub fn load_from_file(path: &str) -> Vec<Block> {
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    from_str(&data).unwrap()
}

/// Finds the next proof given the last one
pub fn proof_of_work(last_proof: u64) -> u64 {
    let mut proof: u64 = last_proof + 1;
    while !valid_proof(last_proof, proof) {
        proof += 1;
    }

    proof
}

/// Verification of a given proof
fn valid_proof(old: u64, new: u64) -> bool {
    let mut hasher: Sha1 = Sha1::new();

    hasher.input_str(&format!("{}", old));
    let old_hash = hasher.result_str();

    let mut hasher: Sha1 = Sha1::new();
    hasher.input_str(&format!("{}", new));
    let new_hash = hasher.result_str();

    old_hash
        .chars()
        .take(5)
        .zip(new_hash.chars().take(5))
        .map(|(a, b)| a != b)
        .position(|a| a)
        .is_none()
}

pub fn get_transactions() -> BlockData {
    let mut block_data = BlockData::new();
    match File::open("transactions.cpecoin") {
        Ok(file) => {
            let mut file = File::open("transactions.cpecoin").unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            // Load all transactions from file
            let transactions = data.lines()
                .map(|line| Transaction::parse_from_data(line.into()))
                .collect();

            // Delete the file so we dont reload the transactions
            remove_file(Path::new("transactions.cpecoin"));

            // Returns the list of all blocks
            block_data.transactions = transactions;
            block_data
        }
        _ => BlockData::new(),
    }
}

pub fn save_blockchain(blocks: &Vec<Block>) {
    let json_chain: String = to_string_pretty(blocks).unwrap();
    let mut file = File::create("blockchain.cpecoin").unwrap();
    file.write_all(json_chain.as_bytes());
}
