use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut current: u8  = 0;
    let mut pos: u64 = 0;
    let mut buffer: Vec<u8> = Vec::new();
    
    // Create the data structure: A vector of Hash maps 
    // with the numbers and their frequencies initialized
    let mut hash_struct: Vec<HashMap<String, u64>> = Vec::with_capacity(6);
    for len  in 0..6 {
        let mut tmp_hash: HashMap<String, u64> = HashMap::with_capacity(10);
        for num in 0..10 {
            let key = num.to_string().repeat(len + 5);
            tmp_hash.insert(key, 0u64);
        }
        hash_struct.push(tmp_hash);
    }
    
    // let mut numbers = Numbers::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_until(b'-', &mut buffer).expect("Could not read the file");
    
    for (i, b) in buffer.iter().enumerate() {
        if b != &current {
            let size = i - pos as usize;
            if size >= 5 {
                let num = String::from(current as char).repeat(size);
                if let Some(freq) = hash_struct[size - 5].get_mut(&num) {
                    *freq += 1;
                }
            }
            current = *b;
            pos = i as u64;
        }   
    }

    // Finally, let us print the table of frequencies.
    println!("Table of frequencies");
    println!("Numbers\t{}", (0..10).into_iter().map(|x| x.to_string() + "\t").collect::<String>());
    println!("Length");
    for (i,hash) in hash_struct.iter().enumerate() {
        let mut print_vec = hash.iter().collect::<Vec<_>>();
        print_vec.sort_by(|x,y| x.0.cmp(y.0));
        let freqs = print_vec.iter().map(|x| x.1).collect::<Vec<_>>();

        println!("{}\t{}", i+5, freqs.iter().map(|x| x.to_string() + "\t").collect::<String>());
    }
}