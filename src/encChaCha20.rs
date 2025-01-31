use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::XChaCha20;

fn hex_to_bytes(str_hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for i in (0..str_hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&str_hex[i..i + 2], 16).expect("Invalid hex string");
        bytes.push(byte);
    }
    bytes
}

fn encrypt_string(plaintext: &str, key: &[u8; 32], iv: &[u8; 24]) -> Vec<u8> {
    // Create cipher instance
    let mut cipher = XChaCha20::new(key.into(), iv.into());

    // Convert plaintext to bytes
    let mut ciphertext = plaintext.as_bytes().to_vec();

    // Encrypt the data in-place
    cipher.apply_keystream(&mut ciphertext);

    ciphertext
}

fn decrypt_string(ciphertext: &[u8], key: &[u8; 32], iv: &[u8; 24]) -> String {
    // Create cipher instance (same key and IV as encryption)
    let mut cipher = XChaCha20::new(key.into(), iv.into());

    // Create a copy of the ciphertext to decrypt (important: don't modify in-place if you need the original)
    let mut plaintext = ciphertext.to_vec();

    // Decrypt the data in-place
    cipher.apply_keystream(&mut plaintext);

    // Convert the decrypted bytes back to a String.  Handle potential errors.
    String::from_utf8(plaintext).unwrap_or_else(|e| panic!("Invalid UTF-8: {}", e))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let plaintext = if args.len() > 1 {
        &args[1]
    } else {
        "This is a Very secret message!"
    };
    // Generate a random 256-bit key and 192-bit IV (Initialization Vector).

    let key_vec = hex_to_bytes("3af5f3d48ca94da0c57dd5062b86a0cd19f83cf48b566cee276f29a82c7f1537");
    let iv_vec = hex_to_bytes("329ea73fae3f3ee91ded6ace33d33657e82a24eff56b725a");

    let mut key = [0u8; 32];
    let mut iv = [0u8; 24];

    key.copy_from_slice(&key_vec);
    iv.copy_from_slice(&iv_vec);

    let ciphertext = encrypt_string(plaintext, &key, &iv);

    println!("Ciphertext: {:?}", ciphertext); // Print the ciphertext (bytes)

    let decrypted_text = decrypt_string(&ciphertext, &key, &iv);

    println!("Decrypted text: {}", decrypted_text); // Print the decrypted string

    assert_eq!(plaintext, decrypted_text); // Ensure that decryption worked correctly
}
