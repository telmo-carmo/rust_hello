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

fn encrypt_string(plaintext: &str, key: &[u8; 32], iv: &[u8; 24]) -> String {
    // Create cipher instance
    let mut cipher = XChaCha20::new(key.into(), iv.into());

    // Convert plaintext to bytes
    let mut ciphertext = plaintext.as_bytes().to_vec();

    // Encrypt the data in-place
    cipher.apply_keystream(&mut ciphertext);

    base64::encode(&ciphertext)
}

fn decrypt_string(ciphertext_b64: &str, key: &[u8; 32], iv: &[u8; 24]) -> String {
    let mut plaintext = base64::decode(ciphertext_b64).expect("bad base64 string");
    // Create cipher instance (same key and IV as encryption)
    let mut cipher = XChaCha20::new(key.into(), iv.into());

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
    // Generate a random192-bit IV (Initialization Vector).

    let mut rng = rand::rng();
    let mut iv = [0u8; 24];
    rng.fill_bytes(&mut iv);
    let iv_b64 = &base64::encode(iv);
    println!("Rnd IV: {}",&iv_b64);

    // crate hex :  hex::decode("aabb")  or hex_to_bytes()
    //  32by , 256-bit key :
    let key_vec = hex::decode("3AF5f3d48ca94da0c57dd5062b86a0cd19f83cf48b566cee276f29a82c7f1537").expect("bad hex str");
   
    let mut key = [0u8; 32];
    key.copy_from_slice(&key_vec);
  

    let ciphertext = encrypt_string(plaintext, &key, &iv);
    println!("Ciphertext: {}", &ciphertext); // Print the ciphertext (bytes) in b64

    let iv_vec = base64::decode(iv_b64).expect("b64 IV str");
    let mut iv = [0u8; 24];
    iv.copy_from_slice(&iv_vec);
    let decrypted_text = decrypt_string(&ciphertext, &key, &iv);

    println!("Decrypted text: {}", decrypted_text); // Print the decrypted string

    assert_eq!(plaintext, decrypted_text); // Ensure that decryption worked correctly

    let s1 = "2023.02.03";

    // semantic Version packing:  if ver 1 > ver2 {}
    let parts: Vec<&str> = s1.split('.').collect();
    if parts.len() == 3 {
        let year: u16 = parts[0].parse().expect("Invalid year");
        let month: u8 = parts[1].parse().expect("Invalid month");
        let day: u8 = parts[2].parse().expect("Invalid day");

        let mut ver_bytes = [0u8; 4];
        ver_bytes[0..2].copy_from_slice(&year.to_be_bytes());
        ver_bytes[2] = month;
        ver_bytes[3] = day;

        let ver_packed = u32::from_be_bytes(ver_bytes);
        println!("Packed version: {} - 0x{:X}", ver_packed, ver_packed);
    } else {
        println!("Invalid date format");
    }
}
