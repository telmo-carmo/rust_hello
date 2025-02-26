/*


Read a file encoded in EBCDIC 1140 and convert it to UTF-8:

"1acção" == HEX bytes:
ebcdic 1140  F1 81 83 B1 C6 96
ISO-8859-1   31 61 63 E7 E3 6F          (also in Win CP 1250)
UTF-8        31 61 63 C3 A7 C3 A3 6F
---


[dependencies]
encoding_rs = "0.8"
encoding_rs_io = "0.1"

---

use encoding_rs::{IBM1140, Encoding, DecoderResult};

fn ebcdic_to_utf8(ebcdic_data: &[u8], encoding: &'static Encoding) -> Result<String, String> {
    let mut decoder = encoding.new_decoder();
    let mut utf8_buffer = String::new();

    let (result, _, _) = decoder.decode_to_string(ebcdic_data, &mut utf8_buffer, true);

    match result {
        DecoderResult::InputEmpty | DecoderResult::OutputFull => Ok(utf8_buffer),
        DecoderResult::Malformed(_, _) => Err("Malformed EBCDIC data".to_string()),
    }
}
        
    let ebcdic_bytes: &[u8] = &[0xC1, 0xC2, 0xC3]; // Example EBCDIC for A,B,C
    let encoding = IBM1140;

    match ebcdic_to_utf8(ebcdic_bytes, encoding) {
        Ok(utf8_string) => println!("UTF-8: {}", utf8_string),
        Err(err) => eprintln!("Error: {}", err),
    }


*/

use std::fs::File;
use std::io::{self, Read, Write};
use encoding_rs::IBM1140;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn main() -> io::Result<()> {
    // Open the input file encoded in EBCDIC 1140
    let input_path = "input_ebcdic.txt";
    let input_file = File::open(input_path)?;

    // Create a decoder for the EBCDIC 1140 encoding
    let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(IBM1140))
        .build(input_file);

    // Read the decoded contents into a string
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;

    // Write the decoded contents to an output file encoded in UTF-8
    let output_path = "output_utf8.txt";
    let mut output_file = File::create(output_path)?;
    output_file.write_all(contents.as_bytes())?;

    Ok(())
}
