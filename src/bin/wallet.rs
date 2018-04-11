extern crate clap;
extern crate cpecoin;

use clap::{App, Arg};
use cpecoin::wallet::generate_wallet_infos;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main() {
    let args = App::new("CPECoin Wallet Generator")
        .version("1.0")
        .author("Olivier PINON <oliv.pinon@gmail.com>")
        .about("Creates a CPECoin wallet")
        .arg(
            Arg::with_name("file")
                .short("f")
                .value_name("file")
                .help("Name of the file to output the informations to"),
        )
        .get_matches();

    let (s, p) = generate_wallet_infos();
    let file_name = match args.value_of("file") {
        Some(str) => str,
        _ => "wallet.cpecoin",
    };

    let mut file = File::create(Path::new(file_name)).unwrap();

    let secret = format!("Secret: {}\n", s);
    let public = format!("Public: {}", p);

    file.write_all(secret.as_bytes()).unwrap();
    file.write_all(public.as_bytes()).unwrap();
}
