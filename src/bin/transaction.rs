extern crate clap;
extern crate cpecoin;

use clap::{App, Arg};
use cpecoin::blockchain::Transaction;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn main() {
    let args = App::new("CPECoin Transaction Maker")
        .version("1.0")
        .author("Olivier PINON <oliv.pinon@gmail.com>")
        .about("Edits transaction file for CPECoin")
        .arg(
            Arg::with_name("from")
                .short("f")
                .value_name("from")
                .help("Address of the sender of the transaction")
                .required(true),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .value_name("to")
                .help("Address of the receiver of the transaction")
                .required(true),
        )
        .arg(
            Arg::with_name("amount")
                .short("a")
                .value_name("amount")
                .help("Amount to send")
                .required(true),
        )
        .get_matches();

    let from = match args.value_of("from") {
        Some(str) => str,
        _ => "",
    };

    let to = match args.value_of("to") {
        Some(str) => str,
        _ => "",
    };

    let amount: u32 = match args.value_of("amount") {
        Some(str) => str.parse::<u32>().unwrap(),
        _ => 0,
    };

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("transactions.cpecoin")
        .unwrap();

    let to_write = format!("{}\n", Transaction::new(from, to, amount));
    file.write_all(to_write.as_bytes());
}
