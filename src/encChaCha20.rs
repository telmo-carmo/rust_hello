use base64::{engine::general_purpose::STANDARD, Engine as _};
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::XChaCha20;
use rand::RngCore;

// fn hex_to_bytes(str_hex: &str) -> Vec<u8> {
//     let mut bytes = Vec::new();
//     for i in (0..str_hex.len()).step_by(2) {
//         let byte = u8::from_str_radix(&str_hex[i..i + 2], 16).expect("Invalid hex string");
//         bytes.push(byte);
//     }
//     bytes
// }

fn encrypt_string(plaintext: &str, key: &[u8; 32]) -> String {
    let mut rng = rand::rng();
    let mut iv = [0u8; 24];
    rng.fill_bytes(&mut iv);
    // Create cipher instance Key 256 bits, Nonce 192 bits
    let mut cipher = XChaCha20::new(key.into(), &iv.into());

    // Convert plaintext to bytes
    let mut ciphertext = plaintext.as_bytes().to_vec();

    // Encrypt the data in-place
    cipher.apply_keystream(&mut ciphertext);
    let mut result = iv.to_vec();
    result.extend(ciphertext);
    STANDARD.encode(&result)
}

fn decrypt_string(ciphertext_b64: &str, key: &[u8; 32]) -> String {
    let vby = STANDARD.decode(ciphertext_b64).expect("bad base64 string");
    let iv = &vby[..24];
    let ctext = &vby[24..];
    // Create cipher instance (same key and IV as encryption)
    let mut cipher = XChaCha20::new(key.into(), iv.into());
    let mut plaintext = ctext.to_vec();
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

    const KEY_HEX : &str = "3AF5f3d48ca94da0c57dd5062b86a0cd19f83cf48b566cee276f29a82c7f1537";

    // crate hex :  hex::decode("aabb")  or hex_to_bytes()
    //  32by , 256-bit key :
    let key_vec = hex::decode(KEY_HEX).expect("bad hex str");
   
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_vec);
  

    let ciphertext = encrypt_string(plaintext, &key);
    println!("Ciphertext: {}", &ciphertext); // Print the ciphertext (bytes) in b64
    let decrypted_text = decrypt_string(&ciphertext, &key);

    println!("Decrypted text: {}", decrypted_text); // Print the decrypted string

    assert_eq!(plaintext, decrypted_text); // Ensure that decryption worked correctly
}
