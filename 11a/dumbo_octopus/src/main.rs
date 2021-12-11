use std::time::{Instant};


const COL_SIZE: usize = 10;
const ROW_SIZE: usize = 10;

//const COL_SIZE: usize = 100;
//const ROW_SIZE: usize = 100;
#[derive(Copy, Clone, Debug)]
struct Octopus{
    value: u32,
    flashed: bool
}

type Map = [[Octopus; COL_SIZE]; ROW_SIZE];

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    let mut map : Map = [[Octopus{value:0, flashed:false}; COL_SIZE]; ROW_SIZE];
    //let mut basin : Map = [[0; COL_SIZE]; ROW_SIZE];

    const RADIX: u32 = 10;

    let mut flash_counter: u32 = 0;
    //let mut basin_counters : Vec<u32> = Vec::new();

    //Fill map
    let mut col_counter = 0;
    let mut row_counter = 0;
    for line in data.lines(){
        for chr in line.chars(){
            //println!("col_counter {}", col_counter);
            //println!("row_counter {}", row_counter);
            map[row_counter][col_counter].value = chr.to_digit(RADIX).unwrap();
            col_counter += 1;
        }
        col_counter = 0;
        row_counter += 1;
    }

    //println!("map {:?}", map);
    //println!("");

    //go through map for each step
    for i in 0..1000{
        //Increase all cells with 1
        for row_counter in 0..ROW_SIZE{
            for col_counter in 0..COL_SIZE{
                map[row_counter][col_counter].value += 1;
            }
        }
        //for all cells if cell value > 9 increase surrounding cells (recursively)
        flash_recursive(&mut map);

        //for all cells if cell value > 9 flash cell, inc flashcounter and set to 0
        let mut cell_eq_0_counter = 0;
        for row_counter in 0..ROW_SIZE{
            for col_counter in 0..COL_SIZE{  
                if map[row_counter][col_counter].value > 9{
                    map[row_counter][col_counter].value = 0;
                    flash_counter += 1;
                    cell_eq_0_counter += 1;
                }
                map[row_counter][col_counter].flashed = false;  
            }
        } 

        if cell_eq_0_counter == 100{
            println!("i {}", i + 1);
            break;
        }


        //println!("map {:?}", map);
        //println!("");
    }

    let duration = start.elapsed();  
    

    println!("{}", flash_counter);
    println!("duration {:?}", duration);
}


fn flash_recursive(map: &mut Map) {
    //for all cells if cell value > 9 increase surrounding cells
    for row_counter in 0..ROW_SIZE{
        for col_counter in 0..COL_SIZE{
            if map[row_counter][col_counter].value > 9 && map[row_counter][col_counter].flashed == false{
                //mark current flashed
                map[row_counter][col_counter].flashed = true;

                //inc up value
                if row_counter > 0{
                    map[row_counter - 1][col_counter].value += 1;
                }
                //inc down value
                if row_counter < ROW_SIZE -1{
                    map[row_counter + 1][col_counter].value += 1;
                }
                //inc left value
                if col_counter > 0{
                    map[row_counter][col_counter -1 ].value += 1;
                }
                //inc right value
                if col_counter < COL_SIZE - 1{
                    map[row_counter][col_counter + 1].value += 1;
                }
                //inc upper right
                if row_counter > 0 && col_counter < COL_SIZE - 1{
                    map[row_counter - 1][col_counter + 1].value += 1;
                }
                //inc lower right
                if row_counter < ROW_SIZE -1 && col_counter < COL_SIZE - 1{
                    map[row_counter + 1][col_counter + 1].value += 1;
                }
                //inc lower left
                if row_counter < ROW_SIZE -1 && col_counter > 0{
                    map[row_counter + 1][col_counter - 1].value += 1;
                } 
                //inc upper left
                if row_counter > 0 && col_counter > 0{
                    map[row_counter - 1][col_counter - 1].value += 1;
                }                                                                                  
            }
        }
    }

    //stop recursing if all cells > 9 is marked flashed
    let mut count_gt_9_and_not_flashed = 0;
    for row_counter in 0..ROW_SIZE{
        for col_counter in 0..COL_SIZE{
            if map[row_counter][col_counter].value > 9 && map[row_counter][col_counter].flashed == false{
                count_gt_9_and_not_flashed += 1;
            }
        }
    }
    
    if count_gt_9_and_not_flashed == 0{
        return
    }

    flash_recursive(map)
}