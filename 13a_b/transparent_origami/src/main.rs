use std::time::{Instant};


const COL_SIZE: usize = 1500;
const ROW_SIZE: usize = 1500;
//const COL_SIZE: usize = 20;
//const ROW_SIZE: usize = 20;

//type Map = [[u32; COL_SIZE]; ROW_SIZE];

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    //let mut map : Map = [[0; COL_SIZE]; ROW_SIZE];
    let mut map = vec![vec![0 as u32; COL_SIZE]; ROW_SIZE];

    //println!("gvtest 1");

    //Fill map
    let mut line_iter = data.lines().into_iter();
    loop{
    //for line in data.lines(){
        let line = line_iter.next();
        if line.unwrap() == ""{
            break;
        }
        let split_line:Vec<&str> = line.unwrap().split(",").collect();
        map[split_line[1].parse::<usize>().unwrap()][split_line[0].parse::<usize>().unwrap()] = 1;
    }

    //Execute folding
    loop{
        let line = line_iter.next();
        if line.is_none(){
            break;
        }
        let folding_info = line.unwrap();
        let instruction:Vec<&str> = folding_info[11..].split('=').collect();
        if instruction[0] == "x"{
            let x_fold = instruction[1].parse::<i32>().unwrap();
            for row_counter in 0..ROW_SIZE{
                for col_counter in x_fold as usize..COL_SIZE{
                    let mut x = col_counter as i32;
                    if x > x_fold{
                        if map[row_counter][col_counter] > 0{
                            map[row_counter][col_counter] = 0;
                            x = 2*x_fold - col_counter as i32;
                            map[row_counter][if x < 0 {0 as usize } else {x as usize}] = 1;
                        }
                    }
                }
            }                
        }
        else{
            let y_fold = instruction[1].parse::<i32>().unwrap();
            for row_counter in y_fold as usize..ROW_SIZE{
                for col_counter in 0..COL_SIZE{
                    let mut y = row_counter as i32;
                    if y > y_fold{
                        if map[row_counter][col_counter] > 0{
                            map[row_counter][col_counter] = 0;
                            y = 2*y_fold - row_counter as i32;
                            map[if y < 0 {0 as usize } else {y as usize}][col_counter] = 1;
                        }
                    }
                }
            }              
        }
        
    }

    /*
    for row_counter in 0..ROW_SIZE{
        for col_counter in 0..COL_SIZE{
            if map[row_counter][col_counter] > 0{
                print!("{}", "#");
            }
            else{
                print!("{}", ".");
            }
        }
        println!();
    } 
    */   

    //read into new smaller map
    let mut max_col = 0;
    let mut max_row = 0;
    for row_counter in 0..ROW_SIZE{
        for col_counter in 0..COL_SIZE{
            if map[row_counter][col_counter] > 0{
                if col_counter > max_col{
                    max_col = col_counter;
                }
                if row_counter > max_row{
                    max_row = row_counter;
                }
            }
        }
    }       

    //let mut small_map = vec![vec![0 as u32; max_col as usize]; max_row as usize];
    for row_counter in 0..max_row+1{
        for col_counter in 0..max_col+1{
            if map[row_counter][col_counter] > 0{
                print!("{}", "#");
            }
            else{
                print!("{}", ".");
            }
        }
        println!();
    }  

    //count dots
    /*
    let mut dot_counter = 0;
    for row_counter in 0..ROW_SIZE{
        for col_counter in 0..COL_SIZE{
            if map[row_counter][col_counter] > 0{
                dot_counter += 1;
            }
        }
    }        
    */

    let duration = start.elapsed();  
    
    //println!("small_map: {:?}", small_map);
    //println!("{}", dot_counter);
    println!("duration {:?}", duration);
}
