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

		let client_encrypted = client_box.encrypt(&nonce, plaintext.as_slice()).unwrap();
		let server_encrypted = server_box.encrypt(&nonce, plaintext.as_slice()).unwrap();

		// Using the same nonce, both boxes encrypt and produce the same ciphertext:
		assert_eq!(client_encrypted, server_encrypted);

		// Boxes can decrypt ciphertext encrypted by the opposite box:
		let server_decrypted = server_box.decrypt(&nonce, &*client_encrypted).unwrap();
		let client_decrypted = client_box.decrypt(&nonce, &*server_encrypted).unwrap();
		assert_eq!(server_decrypted, client_decrypted);  // Decrypted text is the same
		assert_eq!(plaintext, &*server_decrypted);  // and equal to the plaintext

		// Boxes can decrypt ciphertext that they themselves encrypted:
		let client_decrypted = client_box.decrypt(&nonce, &*client_encrypted).unwrap();
		let server_decrypted = server_box.decrypt(&nonce, &*server_encrypted).unwrap();
		assert_eq!(server_decrypted, client_decrypted);  // Decrypted text is the same
		assert_eq!(plaintext, &*server_decrypted);  // and equal to the plaintext
	}
}
