use crypto_box::{aead::OsRng, PublicKey, SecretKey};

struct KeyPair {
	public: PublicKey,
	private: SecretKey,
}

impl KeyPair {
	pub fn new() -> Self {
		let keypair = SecretKey::generate(&mut OsRng);
		Self {
			public: keypair.public_key(),
			private: keypair,
		}
	}
}

#[cfg(test)]
mod tests {
	use crypto_box::{aead::{Aead, AeadCore}, SalsaBox};
	use once_cell::sync::OnceCell;

	use super::*;

	static CLIENT: OnceCell<KeyPair> = OnceCell::new();
	static SERVER: OnceCell<KeyPair> = OnceCell::new();

	impl KeyPair {
		fn get_client_server_keys() -> (&'static Self, &'static Self) {
			(Self::get_singleton(&CLIENT),
			 Self::get_singleton(&SERVER))
		}
		fn get_singleton(cell: &'static OnceCell<KeyPair>) -> &'static Self {
			cell.get_or_init(|| Self::new())
		}
	}

	#[test]
	fn diffie_hellman_symmetry() {
		let (client_keys, server_keys) = KeyPair::get_client_server_keys();
		let client_box = SalsaBox::new(&server_keys.public, &client_keys.private);
		let server_box = SalsaBox::new(&client_keys.public, &server_keys.private);
		let nonce = SalsaBox::generate_nonce(&mut OsRng);

		let plaintext = b"test message";

		// Need same nonce between encrypt and decrypt, so nonce is commonly sent with ciphertext.
		// The same nonce should never be used between two messages, especially if the messages
		// have the same content.

		let client_encrypted = client_box.encrypt(&nonce, &plaintext[..]).unwrap();
		let server_encrypted = server_box.encrypt(&nonce, &plaintext[..]).unwrap();

		assert_eq!(client_encrypted, server_encrypted);
		assert_eq!(plaintext, &server_box.decrypt(&nonce, &client_encrypted[..]).unwrap()[..]);
		assert_eq!(plaintext, &client_box.decrypt(&nonce, &server_encrypted[..]).unwrap()[..]);
	}
}
