fn main() {
    let data = include_str!("data.txt");
    let mut prev_val = 0;
    let mut inc_counter = 0;
    let mut row_counter = 0;
    for str in data.lines(){
        let curr_val : i32 = str.parse().unwrap_or(0);
        if curr_val > prev_val && row_counter > 0{
            inc_counter += 1;
        }
        prev_val = curr_val;
        row_counter +=1;
    }    
    println!("{}", inc_counter);
    println!("total rows {}", row_counter);
}