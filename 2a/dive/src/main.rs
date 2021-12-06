use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    let mut hor_pos = 0;
    let mut vert_pos = 0;
    for line in data.lines(){
        let split = line.split_whitespace();
        let vec: Vec<&str> = split.collect();
        let val : i32 = vec[1].parse().unwrap_or(0);
        match vec[0]{
            "forward" => hor_pos += val,
            "down" => vert_pos += val,
            "up" => vert_pos -= val,
            _ => println!("something else!"),
        }
    }
    let duration = start.elapsed();    
    println!("{}", hor_pos * vert_pos);
    println!("duration {:?}", duration);
}