extern crate cpecoin;

use cpecoin::wallet::generate_wallet_infos;

pub fn main() {

    let (s,p) = generate_wallet_infos();

    println!("Secret: {}", s);
    println!("Public: {}", p);
}
