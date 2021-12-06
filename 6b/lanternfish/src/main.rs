use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");
    const RUNNING_DAYS: u32 = 256;
    const HISTOGRAM_DAYS: usize = 8;
    let mut days_histogram : [u64;HISTOGRAM_DAYS + 1] = [0;HISTOGRAM_DAYS + 1];

    let mut day_counter : u32 = 0;

    for start_fish in data.split(","){
        days_histogram[start_fish.parse::<u32>().unwrap() as usize] += 1;
    }
    //println!("days_histogram {:?}", days_histogram);
    while day_counter < RUNNING_DAYS {
        let current_day0 = days_histogram[0];
        for day in 0..HISTOGRAM_DAYS{
            days_histogram[day] = days_histogram[day+1]
        }

        days_histogram[8] = current_day0;
        days_histogram[6] += current_day0;

        //println!("days_histogram {:?}", days_histogram);

        day_counter += 1;
    }

    let duration = start.elapsed();    

    let sum : u64 = days_histogram.iter().sum();
    println!("{}", sum);
    println!("duration {:?}", duration);
}

