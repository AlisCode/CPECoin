#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate cpecoin;
#[macro_use]
extern crate serde_derive;

use cpecoin::blockchain::Block;
use rocket::State;

#[derive(Deserialize)]
pub struct MinerConfig {
    /// The public address that will be rewarded coins when we mine blocks
    pub miner_address: String,
    /// The peers of this miner node
    pub peers: Vec<String>,
}

/// A struct representing the state of a miner
pub struct Miner {
    pub blockchain: Vec<Block>,
    pub config: MinerConfig,
}

impl Miner {
    pub fn new_empty() -> Self {
        Miner {
            blockchain: Vec::new(),
            config: MinerConfig {
                miner_address: "".into(),
                peers: Vec::new(),
            },
        }
    }
}

#[get("/blocks/len")]
pub fn blockchain_length(miner: State<Miner>) -> String {
    format!("{}", miner.blockchain.len())
}

pub fn main() {
    println!("Miner!");
    let mut miner: Miner = Miner::new_empty();
    rocket::ignite()
        .mount("/", routes![blockchain_length])
        .manage(miner)
        .launch();
}
