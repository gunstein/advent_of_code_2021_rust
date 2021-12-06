use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    let mut bca: [i32; 12] = [0; 12];//bitcounter array
    //let mut bca: [i32; 5] = [0; 5];//bitcounter array
    let mut row_counter = 0;
    let base: i32 = 2;
    for line in data.lines(){
        let diag = isize::from_str_radix(line, 2).unwrap();
        //println!("diag {:?}", diag);
        for i in 0..12 {
        //for i in 0..5 {
            //println!("i {:?}", i);
            //println!("base.pow(i) {:?}", base.pow(i));
            bca[i as usize] += if diag as i32 & base.pow(i) > 0 {1} else {0};//add 1 if bit in position i is set
            //println!("bca[i as usize] {:?}", bca[i as usize]);
        }
        row_counter += 1;
    }
    //println!("bca {:?}", bca);
    //find gamma_rate
    let mut gamma_rate = 0;
    let num_rows_div_2 = row_counter / 2;
    for i in 0..12 {
    //for i in 0..5 {
        if bca[i as usize] > num_rows_div_2{
            gamma_rate = gamma_rate | base.pow(i);
        }
    }

    //println!("gamma_rate {}", gamma_rate);

    let epsilon_rate = gamma_rate^isize::from_str_radix("111111111111", 2).unwrap() as i32;//Toggle all bits

    //println!("epsilon_rate {:?}", epsilon_rate);

    let duration = start.elapsed();    
    println!("{}", gamma_rate * epsilon_rate);
    println!("duration {:?}", duration);
}