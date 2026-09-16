/*
Uses a A pure-Rust implementation of x25519 elliptic curve Diffie-Hellman key exchange, with curve operations provided by curve25519-dalek.

and ChaCha20Poly1305 for symmetric  encryption of messages.

---

cargo run --release --bin encChaCha20 -- "Era uma vez um gato maltês"

*/
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use chacha20poly1305::ChaCha20Poly1305;
use chacha20poly1305::aead::{Aead, KeyInit};
use rand::Rng;
use x25519_dalek::{PublicKey, StaticSecret};

struct KeyPair {
    private_key: StaticSecret,
    public_key: PublicKey,
}

fn generate_key_pair() -> KeyPair {
    let mut private_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut private_bytes);

    let private_key = StaticSecret::from(private_bytes);
    let public_key = PublicKey::from(&private_key);

    KeyPair {
        private_key,
        public_key,
    }
}

fn derive_shared_key(private_key: &StaticSecret, peer_public_key: &PublicKey) -> [u8; 32] {
    let shared_secret = private_key.diffie_hellman(peer_public_key);
    *shared_secret.as_bytes()
}

fn encrypt_string(plaintext: &str, key_bytes: &[u8; 32]) -> String {
    let cipher = ChaCha20Poly1305::new(key_bytes.into());

    let mut nonce_bytes = [0u8; 12];
    rand::rng().fill_bytes(&mut nonce_bytes);
    let nonce = &nonce_bytes.into();

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .expect("encryption failed");

    let mut payload = nonce_bytes.to_vec();
    payload.extend_from_slice(&ciphertext);
    URL_SAFE_NO_PAD.encode(payload)
}

fn decrypt_string(ciphertext_b64: &str, key_bytes: &[u8; 32]) -> String {
    let payload = URL_SAFE_NO_PAD
        .decode(ciphertext_b64)
        .expect("invalid Base64 ciphertext");

    let (nonce_bytes, ciphertext) = payload
        .split_at_checked(12)
        .expect("ciphertext is missing its nonce");

    let nonce: &[u8; 12] = nonce_bytes.try_into().expect("nonce has incorrect length");
    let cipher = ChaCha20Poly1305::new(key_bytes.into());
    let plaintext = cipher
        .decrypt(nonce.into(), ciphertext)
        .expect("decryption failed: wrong key or modified ciphertext");

    String::from_utf8(plaintext).expect("decrypted data is not valid UTF-8")
}

fn main() {
    let plaintext = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "This is a very secret message!".to_owned());

    // Alice and Bob create independent X25519 key pairs.
    let alice = generate_key_pair();
    println!(
        "Alice private key: {}; Alice public key: {}",
        URL_SAFE_NO_PAD.encode(alice.private_key.to_bytes()),
        URL_SAFE_NO_PAD.encode(alice.public_key.as_bytes())
    );
    let bob = generate_key_pair();
    println!(
        "Bob   private key: {}; Bob   public key: {}",
        URL_SAFE_NO_PAD.encode(bob.private_key.to_bytes()),
        URL_SAFE_NO_PAD.encode(bob.public_key.as_bytes())
    );

    // Only public keys are exchanged; both sides derive the same 32-byte secret.
    let alice_shared_key = derive_shared_key(&alice.private_key, &bob.public_key);
    let bob_shared_key = derive_shared_key(&bob.private_key, &alice.public_key);
    assert_eq!(alice_shared_key, bob_shared_key);

    println!(
        "Shared key length: {} ; key= {}\n",
        alice_shared_key.len(),
        URL_SAFE_NO_PAD.encode(alice_shared_key)
    );

    let ciphertext = encrypt_string(&plaintext, &alice_shared_key);
    println!("Ciphertext: {ciphertext}");
    println!(
        "Plaintext length: {}; Ciphertext B64 length: {};",
        plaintext.len(),
        ciphertext.len()
    );

    let decrypted_text = decrypt_string(&ciphertext, &bob_shared_key);
    println!("Decrypted text: {decrypted_text}");

    assert_eq!(plaintext, decrypted_text);
}
