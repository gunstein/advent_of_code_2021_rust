use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    let mut vec_fish : Vec<u32> = Vec::new();

    let mut day_counter : u32 = 0;

    for start_fish in data.split(","){
        vec_fish.push(start_fish.parse::<u32>().unwrap());
    }

    while day_counter < 256 {
        let mut vec_new_fish : Vec<u32> = Vec::new();
        for fish in vec_fish.iter_mut(){
            if *fish == 0{
                vec_new_fish.push(8);
                *fish = 6;
            }
            else{
                *fish -= 1;
            }
        }
        //add new fish to main fish vector
        for new_fish in vec_new_fish.iter(){
            vec_fish.push(*new_fish);
        }
        day_counter += 1;
    }

    let duration = start.elapsed();    

    println!("{}", vec_fish.len());
    println!("duration {:?}", duration);
}

