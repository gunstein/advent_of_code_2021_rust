use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");
    let mut crabs_hor_pos : Vec<u32> = Vec::new();

    for crab_pos in data.split(","){
        crabs_hor_pos.push(crab_pos.parse::<u32>().unwrap());
    }

    //Calculate median
    //let median = median(&crabs_hor_pos);
    //println!("median {}", median);
    let mut curr_pos = 0;
    let mut prev_fuel = compute_fuel(&crabs_hor_pos, curr_pos);
    loop {
        curr_pos += 1;
        //println!("prev_fuel {}", prev_fuel);
        let curr_fuel = compute_fuel(&crabs_hor_pos, curr_pos);
        //println!("curr_fuel {}", curr_fuel);
        if prev_fuel < curr_fuel{
            break;
        }
        prev_fuel = curr_fuel;  
    }

    let duration = start.elapsed();  
    
    println!("opt_pos {}", curr_pos - 1);

    println!("{}", prev_fuel);
    println!("duration {:?}", duration);
}

fn compute_single_crab_move_cost(crab_pos: u32)->u32{
    let mut cost:u32 = 0;
    for i in 1..crab_pos + 1{
        cost += i;
    }
    cost
}

fn compute_fuel(array: &Vec<u32>, opt_pos: u32) -> u32{
    let mut fuel : u32 = 0;
    //println!("array {:?}", array);
    for element in array{
        fuel += compute_single_crab_move_cost(i32::abs(*element as i32 - opt_pos as i32) as u32);
        //println!("fuel {}", fuel);
    }
    fuel
}