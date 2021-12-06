use std::time::{Instant};

const ROW_SIZE: usize = 5;
const COL_SIZE: usize = 5;

#[derive(Copy, Clone, Debug)]
struct Cell{
    value: u32,
    marked: bool
}

type Board = [[Cell; COL_SIZE]; ROW_SIZE];

type Game = Vec<Board>;

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");
    let mut game : Game = Vec::new();
    
    let mut drawed_numbers: String = String::new();

    let mut winner_board_value = 0;

    //Init game from datafile
    init_game_from_datafile(data, &mut drawed_numbers, &mut game);

    //println!("drawed_numbers: {}", drawed_numbers);
    //println!("game: {:?}", game);
    'outer: for drawed_number_str in drawed_numbers.split(","){
        //println!("drawed_numbers_str: {}", drawed_number_str);
        let drawed_number = drawed_number_str.parse::<u32>().unwrap();
        for board in game.iter_mut() {
            mark_board(board, drawed_number);
            if check_board_for_bingo(board){
                winner_board_value = calc_winner_board_value(board, drawed_number);
                break 'outer;
            }
        }
    }    

    //println!("game: {:?}", game);

    let duration = start.elapsed();    

    println!("{}", winner_board_value);
    println!("duration {:?}", duration);
}

fn init_game_from_datafile(data: &str, drawed_numbers: &mut String, game: &mut Game){
    let mut curr_board : Board = [[Cell{value: 0, marked: false}; COL_SIZE]; ROW_SIZE];
    let mut row_counter = 0;
    let mut col_counter = 0;

    let data_vec: Vec<&str> = data.lines().collect();
    let mut data_vec_iter = data_vec.iter();
    drawed_numbers.push_str(data_vec_iter.next().unwrap());
    
    //skip next line (is blank)
    data_vec_iter.next();

    //build boards
    for line in data_vec_iter{
        //println!("line: {}", line);
        if line.is_empty(){
            continue;
        }
        else{
            //println!("row_counter: {:?}", row_counter);

            for cell_val_str in line.split_whitespace(){
                curr_board[row_counter][col_counter].value = cell_val_str.parse::<u32>().unwrap();
                col_counter += 1;
            }
            col_counter = 0;
            if row_counter == ROW_SIZE - 1{
                //println!("curr_board: {:?}", curr_board);
                game.push(curr_board.clone());
                curr_board = [[Cell{value: 0, marked: false}; COL_SIZE]; ROW_SIZE];
                row_counter = 0;
                col_counter = 0;
            }
            else{
                row_counter += 1;
            }
        }
    }
}


fn calc_winner_board_value(board: &Board, drawed_number: u32) -> u32{
    //go through all cells and add to sum if is not marked
    let mut sum = 0;
    for row in 0..ROW_SIZE{
        for col in 0..COL_SIZE{
            if board[row][col].marked == false{
                sum += board[row][col].value;
            }
        }
    }
    //println!("winnerboard: {:?}", board);
    //println!("winnerboard sum {}", sum);
    //println!("winnerboard drawed_number {}", drawed_number);

    return sum * drawed_number;
}

fn mark_board(board: &mut Board, drawed_number: u32){
    //go through all cells and mark cell if value found
    /*if drawed_number == 24{
        println!("24");
        println!("board 24: {:?}", board);
    }*/

    for row in 0..ROW_SIZE{
        for col in 0..COL_SIZE{
            if board[row][col].value == drawed_number{
                board[row][col].marked = true;
                //println!("found value so mark");
                //println!("board 24 after mark: {:?}", board);
            }
        }
    }
}

fn check_board_for_bingo(board: &Board)-> bool {
    //go through all rows and check if all cells in a row are marked       
    for row in 0..ROW_SIZE{
        for col in 0..COL_SIZE{
            //stop if not marked
            if board[row][col].marked == false{
                break;
            }
            else if col == COL_SIZE - 1 && board[row][col].marked == true{
                //return true if last element and this is also marked
                //println!("BINGO horisontally row {}!!", row);
                return true;
            }
        }
    }
    //go through all columns and check if all cells in a column are marked
    for col in 0..COL_SIZE{
        for row in 0..ROW_SIZE{
            if board[row][col].marked == false {
                break;
            }
            else if row == ROW_SIZE - 1 && board[row][col].marked == true{
                //println!("BINGO vertically col {}!!", col);
                return true;
            }
        }
    }

    return false;
}