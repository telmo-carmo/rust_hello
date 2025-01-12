/*

cargo build --release
cargo run --release
*/
use csv::ReaderBuilder;
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Cli1 {
    id: u32,
    name: String,
    age: i32,
}

fn main() {
    println!("Hello, RUST!");
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My exe path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);

    let fname = if args.len() > 1 {
        &args[1]
    } else {
        "data1.csv"
    };

    // Create a CSV reader
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';') // Adjust delimiter as needed
        .has_headers(true) // Explicitly indicate header presence
        .trim(csv::Trim::All)
        .from_path(fname)
        .unwrap();

    let headers = rdr.headers().unwrap();
    println!("Headers {:?}", headers);

    let mut rn = 1;
    let mut records: Vec<Cli1> = Vec::new();
 
    // Read records from the CSV file
    for result in rdr.deserialize() {
        // Process the record
        let record: Cli1 = result.unwrap();
        println!("{rn:03}: {:?}", record);
        records.push(record);
        rn += 1;
    }

 // Create a CSV writer
 let output_fname = "output.csv";
 let mut wtr = WriterBuilder::new()
     .delimiter(b';') // Adjust delimiter as needed
     .has_headers(true) // Explicitly indicate header presence
     .from_path(output_fname)
     .unwrap();

 // Write headers
 wtr.write_record(headers.iter()).unwrap();

 // Write records
 for record in records {
     wtr.serialize(record).unwrap();
 }

 wtr.flush().unwrap();
 println!("Data written to {}", output_fname);
}