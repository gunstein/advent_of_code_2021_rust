use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    let mut prev_win_sum = 0;
    let mut curr_win_sum;
    let mut inc_counter = 0;
    let mut row_counter = 0;
    let mut win_1 = 0;
    let mut win_2 = 0;
    let mut win_3 = 0;
    for str in data.lines(){
        let curr_val : i32 = str.parse().unwrap_or(0);
        if row_counter < 3{
            if row_counter == 0{
                win_1 = curr_val;
            }
            else if row_counter == 1{
                win_2 = curr_val;
            }
            else{
                win_3 = curr_val;
                prev_win_sum = win_1 + win_2 + win_3;
            }
        }
        else{
            win_1 = win_2;
            win_2 = win_3;
            win_3 = curr_val;
            curr_win_sum = win_1 + win_2 + win_3;
            if curr_win_sum > prev_win_sum{
                inc_counter += 1;
            }
            prev_win_sum = curr_win_sum;
        }

        row_counter +=1;
    }
    let duration = start.elapsed();    
    println!("{}", inc_counter);
    println!("total rows {}", row_counter);
    println!("duration {:?}", duration);
}