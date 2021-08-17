use std::io::{self, BufRead};

fn main() {
    let mut chosen: u8 = 0;
    let mut current: u8  = 0;
    let mut pos: u64 = 0;
    let mut max_len: u64 = 0;
    let mut pos_max: u64 = 0;
    let mut buffer: Vec<u8> = Vec::new();
    
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_until(b'-', &mut buffer).expect("Could not read the file");
    
    for (i, b) in buffer.iter().enumerate() {
        if b != &current {
            if i as u64 - pos > max_len {
                max_len = i as u64 - pos;
                pos_max = pos;
                chosen = current;
            }
            current = *b;
            pos = i as u64;            
        }   
    }
    println!("The number that repeats itself the most is {}, it repeats itself {} times, starting from the decimal {}", chosen as char, max_len, pos_max -2)
}
