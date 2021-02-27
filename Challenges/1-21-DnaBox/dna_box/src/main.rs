mod kmer_functions;
use crate::kmer_functions::freq_kmers;
//use std::fmt::format; 
use std::io::{self, Write};
use bio::io::fasta;
use std::str::from_utf8;
use std::fs::File;
//use std::io::prelude::*;
use std::fs;
use std::path::Path;


pub fn main() {
    // Create a directory where to put the results
    match fs::create_dir("./Results_freqs") {
        Err(why) => panic!("couldn't create directory: {}", why),
        Ok(_) => (),
    };

    // Read the fasta files from stdin
    let reader = fasta::Reader::new(io::stdin());

    for result in reader.records() {
        let record = result.expect("Error parsing record"); 
        //println!(">{}", record.id());

        let sequence = from_utf8(record.seq()).unwrap_or_default();
        let freqs = freq_kmers(&sequence, 9);
        
        // Create file to put output
        
        // let name = String::from("./Results/").push_str(record.id());
        let name = format!("./Results_freqs/{}", record.id());
        let path = Path::new(&name);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        
        let body: String = freqs.iter().map(ToString::to_string).collect::<Vec<String>>().join("\n");
        match file.write_all(body.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

}
