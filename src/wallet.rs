use crypto::digest::Digest;
use crypto::sha1::Sha1;
use rand::{Rng, thread_rng};
use time::precise_time_ns;

/// Checks the validity of a secret key given a wallet address.
/// This is part of the transaction verification process
pub fn check_wallet_validity(secret: &str, public: &str) -> bool {
	let mut hasher: Sha1 = Sha1::new();
	hasher.input_str(secret);
	hasher.result_str() == public
}

/// Garanties a unique identifier (actually it is possible that 
/// two machines generate the exact same secret key, but it is 
/// extremely unlikely)
pub fn generate_wallet_infos() -> (String, String) {
	let mut secret: String = format!("{}", precise_time_ns());
	let mut rng = thread_rng();
	(0..40 - secret.len()).for_each(|_| {
		let mut random: f32 = rng.gen();
		random = (random * 9.).floor();
		secret.push_str(&format!("{}", random));
	});

	let mut hasher: Sha1 = Sha1::new();
	hasher.input_str(&secret);

	(secret, hasher.result_str().into())
}