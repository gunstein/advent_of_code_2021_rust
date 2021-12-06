use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    let mut vec0: Vec<i32> = Vec::new();
    let mut vec1: Vec<i32> = Vec::new();
    let oxygen: i32;
    let co2: i32;
    let base: i32 = 2;
    //check first bit and push to correct vector
    let bit_index : u32 = 11;
    for line in data.lines(){
        let diag = isize::from_str_radix(line, 2).unwrap();
        if diag as i32 & base.pow(bit_index) > 0 {
            vec1.push(diag as i32)
        } 
        else {
            vec0.push(diag as i32)
        };
    }

    //println!("vec0 {:?}", vec0);
    //println!("vec1 {:?}", vec1);

    if vec1.len() >= vec0.len(){
        oxygen = recursive_oxygen(&vec1, bit_index - 1);
    }
    else{
        oxygen = recursive_oxygen(&vec0, bit_index - 1);
    }

    if vec0.len() <= vec1.len(){
        co2 = recursive_co2(&vec0, bit_index - 1);
    }
    else{
        co2 = recursive_co2(&vec1, bit_index - 1);
    }

    let duration = start.elapsed();    
    //println!("oxygen {}", oxygen);
    //println!("co2 {}", co2);
    println!("{}", oxygen * co2);
    println!("duration {:?}", duration);
}

fn recursive_oxygen(diag_vec: &Vec<i32>, bit_index: u32) -> i32{
    if diag_vec.len() == 1{
        return diag_vec[0];
    }
    let base: i32 = 2;
    let mut vec0: Vec<i32> = Vec::new();
    let mut vec1: Vec<i32> = Vec::new();
    for diag in diag_vec.iter() {
        if diag & base.pow(bit_index) > 0 {
            vec1.push(diag.clone())
        } 
        else {
            vec0.push(diag.clone())
        };            
    }

    if vec1.len() >= vec0.len(){
        //println!("vec0 {:?}", vec0);
        //println!("vec1 {:?}", vec1);
        return recursive_oxygen(&vec1, if bit_index >= 1 {bit_index - 1} else {0});
    }
    else{
        //println!("vec0 {:?}", vec0);
        //println!("vec1 {:?}", vec1);
        return recursive_oxygen(&vec0, if bit_index >= 1 {bit_index - 1} else {0});
    }
}

fn recursive_co2(diag_vec: &Vec<i32>, bit_index: u32) -> i32{
    if diag_vec.len() == 1{
        return diag_vec[0];
    }
    let base: i32 = 2;
    let mut vec0: Vec<i32> = Vec::new();
    let mut vec1: Vec<i32> = Vec::new();
    for diag in diag_vec.iter() {
        if diag & base.pow(bit_index) > 0 {
            vec1.push(diag.clone())
        } 
        else {
            vec0.push(diag.clone())
        };            
    }

    if vec0.len() <= vec1.len(){
        return recursive_co2(&vec0, if bit_index >= 1 {bit_index - 1} else {0});
    }
    else{
        return recursive_co2(&vec1, if bit_index >= 1 {bit_index - 1} else {0});
    }
}