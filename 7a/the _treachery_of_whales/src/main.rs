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
    let median = median(&mut crabs_hor_pos);
    //println!("median {}", median);
    let fuel = compute_fuel(&crabs_hor_pos, median);

    let duration = start.elapsed();    

    println!("{}", fuel);
    println!("duration {:?}", duration);
}

fn median(array: &mut Vec<u32>)->u32{
        array.sort();
        if (array.len() % 2)==0 {
            let ind_left = array.len()/2-1;
            let ind_right = array.len()/2 ;

            (array[ind_left]+array[ind_right]) as u32 / 2
        } else {
             array[(array.len()/2)]
        }
}

fn compute_fuel(array: &Vec<u32>, median: u32) -> u32{
    let mut fuel : u32 = 0;
    //println!("array {:?}", array);
    for element in array{
        fuel += i32::abs(*element as i32 - median as i32) as u32;
        //println!("fuel {}", fuel);
    }
    fuel
}