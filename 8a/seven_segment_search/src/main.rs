use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");
    
    let mut easy_digit_counter = 0;
    for line in data.lines(){
        let digits : Vec<&str> = line.split(" | ").collect();
        for digit in digits[1].split_whitespace(){
            let digit_len = digit.len();
            if digit_len == 2 || digit_len == 4 || digit_len == 3 || digit_len == 7{ 
                easy_digit_counter += 1;
            }
        }
    }

    let duration = start.elapsed();  
    
    println!("{}", easy_digit_counter);
    println!("duration {:?}", duration);
}