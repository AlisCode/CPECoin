extern crate clap;
extern crate cpecoin;

use clap::{App, Arg};
use cpecoin::blockchain::{get_transactions, proof_of_work, Block, Transaction};
use std::path::Path;

const COINS_PER_BLOCK: u32 = 10;

/// A struct representing the state of a miner
pub struct Miner {
    pub blockchain: Vec<Block>,
}

impl Miner {
    pub fn new_empty() -> Self {
        Miner {
            blockchain: Vec::new(),
        }
    }
}

pub fn main() {
    let args = App::new("CPECoin Miner")
        .version("1.0")
        .author("Olivier PINON <oliv.pinon@gmail.com>")
        .about("Mines CPECoin for the given address")
        .arg(
            Arg::with_name("address")
                .short("a")
                .value_name("address")
                .help("Wallet Public Address to mine for")
                .required(true),
        )
        .get_matches();

    let miner_address = match args.value_of("address") {
        Some(str) => str,
        _ => "".into(),
    };

    let mut miner: Miner = Miner::new_empty();
    let path_file: &str = "blockchain.cpecoin";
    let path = Path::new(path_file);
    if path.exists() {
        miner.blockchain = cpecoin::blockchain::load_from_file(path_file);
    } else {
        let mut block_genesis = Block::genesis();
        block_genesis.calc_hash();
        miner.blockchain.push(block_genesis);
    }

    println!("Now mining!");
    loop {
        // Gets the latest block in the chain
        let (old_proof, index, prev_hash) = {
            let block = miner.blockchain.last().unwrap();
            (block.proof, block.index, block.hash.clone())
        };
        // Blocks until the proof is found
        let new_proof = proof_of_work(old_proof);

        let mut data = get_transactions();
        data.transactions.push(Transaction::new(
            "network".into(),
            miner_address.clone(),
            COINS_PER_BLOCK,
        ));
        let mut new_block = Block::new_from_prev(index, prev_hash, data);
        new_block.proof = new_proof;
        new_block.calc_hash();
        println!("Mined a block with hash : {}", new_block.hash);
        miner.blockchain.push(new_block);

        // Save the blockchain to the file
        cpecoin::blockchain::save_blockchain(&miner.blockchain);
    }
}
