use std::{u32, u64};

fn steps_collatz(mut _num: u64) -> u32 {
    let mut step: u32 = 0;
    while _num != 1 {
        _num = match _num % 2 {
            0 => _num / 2,
            1 => 3 * _num + 1,
            _ => 0, // println!("Got a non-expected value!"),
        };
        step +=  1;
        // let step =  step + 1
    }
    step
}


fn main() {
    // 
    let mut max_steps: u32 = 0;
    let mut max_num: u64 = 0;
    let up_to: u64 = 10000000;

    for i in 1..up_to {
        let steps = steps_collatz(i);
        if steps > max_steps {
            max_steps = steps;
            max_num = i;
        } 
    }

    println!("The max number is {} and it takes {} steps", max_num, max_steps);
}
