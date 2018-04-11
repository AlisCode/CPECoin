extern crate clap;
extern crate cpecoin;

use clap::{App, Arg};
use cpecoin::blockchain::{Block, load_from_file};

pub enum TransacDirection {
	From,
	To,
}

pub fn main() {
	let args = App::new("CPECoin Balance Checker")
		.version("1.0")
		.author("Olivier PINON <oliv.pinon@gmail.com>")
		.about("Shows balance of a CPECoin wallet")
		.arg(Arg::with_name("address")
			.short("a")
			.value_name("address")
			.help("Wallet Public Address to check the balance of")
			.required(true))
		.get_matches();

	let address = match args.value_of("address") {
		Some(str) => str,
		_ => "".into()
	};

	let blocks = load_from_file("blockchain.cpecoin");
	let due: i64 = compute_balance(&blocks, TransacDirection::From, address.into()) as i64;
	let earned: i64 = compute_balance(&blocks, TransacDirection::To, address.into()) as i64;

	let balance: i64 = earned - due;

	println!("The balance of wallet {} is currently {} CPECoin", address, balance);
}

pub fn compute_balance(blocks: &Vec<Block>, dir: TransacDirection, address: String) -> u32 {
	blocks.iter()
		  .map::<u32, _>(|block| block.data.transactions
									  .iter()
									  .filter_map(|a| match dir {
										  TransacDirection::From if a.from == address => { Some(a.amount) }
										  TransacDirection::To if a.to == address => { Some(a.amount) }
										  _ => None
									  })
									  .sum()
		  )
		  .sum()
}