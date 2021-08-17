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
            if (i as u64 - pos) as u64 > max_len {
                max_len = (i as u64 - pos) as u64;
                pos_max = pos;
                chosen = current;
            }
            current = *b;
            pos = i as u64;            
        }   
    }
    println!("The number that repeats itself the most is {}, it repeats itself {} times, starting from the decimal {}", chosen as char, max_len, pos_max -2)

    // let stdin = io::stdin();
    // for (i, byte) in stdin.lock().read_until().enumerate(){

    // }
    // for (i, line) in stdin.lock().lines().enumerate() {
    //     for chr in line.unwrap().as_bytes() {
    //         println!("{}", chr)
    //     };
        
    // }

    // for line in io::stdin().read_line(&mut buffer) {

    // }

    // for digit in buffer {
    //     if digit == '.' || digit == '\n' {
            
    //         decimals = True
    //     }
    //     something
    // }

    // println!("Hello, world!");
}
