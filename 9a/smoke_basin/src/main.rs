use std::time::{Instant};


//const COL_SIZE: usize = 10;
//const ROW_SIZE: usize = 5;

const COL_SIZE: usize = 100;
const ROW_SIZE: usize = 100;

type Map = [[u32; COL_SIZE]; ROW_SIZE];

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    let mut map : Map = [[0; COL_SIZE]; ROW_SIZE];

    const RADIX: u32 = 10;

    let mut low_points_values : Vec<u32> = Vec::new();

    //Fill map
    let mut col_counter = 0;
    let mut row_counter = 0;
    for line in data.lines(){
        for chr in line.chars(){
            //println!("col_counter {}", col_counter);
            //println!("row_counter {}", row_counter);
            map[row_counter][col_counter] = chr.to_digit(RADIX).unwrap();
            col_counter += 1;
        }
        col_counter = 0;
        row_counter += 1;
    }

    //Find low points
    for row_counter in 0..ROW_SIZE{
        for col_counter in 0..COL_SIZE{
            let curr_cell_value = map[row_counter][col_counter];
            //Set default surrounding values to max=9
            let mut up_value = 9;
            let mut left_value = 9;
            let mut right_value = 9;
            let mut down_value = 9;
            //set up value
            if row_counter > 0{
                up_value = map[row_counter - 1][col_counter];
            }
            //set down value
            if row_counter < ROW_SIZE -1{
                down_value = map[row_counter + 1][col_counter];
            }
            //set left value
            if col_counter > 0{
                left_value = map[row_counter][col_counter -1 ];
            }
            //set right value
            if col_counter < COL_SIZE - 1{
                right_value = map[row_counter][col_counter + 1];
            }

            if curr_cell_value < up_value && curr_cell_value < down_value && curr_cell_value < left_value && curr_cell_value < right_value {
                low_points_values.push(curr_cell_value + 1);
            }
        }      
    }


    let duration = start.elapsed();  
    let sum_low_points : u32 = low_points_values.iter().sum();
    println!("{}", sum_low_points);
    println!("duration {:?}", duration);
}