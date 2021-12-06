use std::time::{Instant};

const ROW_SIZE: usize = 5;
const COL_SIZE: usize = 5;

#[derive(Copy, Clone, Debug)]
struct Cell{
    value: u32,
    marked: bool
}

type Board = [[Cell; COL_SIZE]; ROW_SIZE];

#[derive(Copy, Clone, Debug)]
struct GameRec{
    id: u32,
    board: Board,
    bingo: bool,
    bingo_value: u32
}

type Game = Vec<GameRec>;

fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");
    let mut game : Game = Vec::new();
    
    let mut drawed_numbers: String = String::new();

    let mut looser_board_value = 0;

    //Init game from datafile
    init_game_from_datafile(data, &mut drawed_numbers, &mut game);

    println!("Number of boards {}", game.len());

    //println!("drawed_numbers: {}", drawed_numbers);
    //println!("game: {:?}", game);
    'outer: for drawed_number_str in drawed_numbers.split(","){
        //println!("drawed_numbers_str: {}", drawed_number_str);
        let drawed_number = drawed_number_str.parse::<u32>().unwrap();
        println!("drawed_number {}", drawed_number);
        //let mut game_recs_to_remove : Vec<u32> = Vec::new();
        for game_rec in game.iter_mut() {
            mark_board(&mut game_rec.board, drawed_number);
            if check_board_for_bingo(&mut game_rec.board){
                println!("BINGO!!");
                game_rec.bingo = true;
                game_rec.bingo_value = drawed_number;
            }
        }

        //if only one board left. This is the looser board
        if game.len() == 1 && game[0].bingo == true{
            println!("only one board left. game.len() {}", game.len());
            println!("drawed_number when finished {}", game[0].bingo_value);
            looser_board_value = calc_looser_board_value(&game[0].board, game[0].bingo_value);
            break 'outer;
        }

        //remove all games with bingo from game
        /*
        println!("game.len() before remove {}", game.len());
        let gvtest= game
        .iter()
        .filter(|r| r.bingo == true).count(); 

        println!("games with bingo {}", gvtest);
        */

        //game.retain(|&i|i.bingo == true);
        //Prøvde flere måter å slette på, men bare denne fungerte, retain(slettet feil og bare en rad) og position(se under, slettet bare en rad) slettet feil,
        for i in (0..game.len()).rev() {
            if game[i].bingo == true {
                game.swap_remove(i);
            }
        }
        /*
        if let Some(index) = game.iter().position(|value| value.bingo == true) {
            println!("index to remove {}", index);
            game.swap_remove(index);
        }*/

        //println!("game.len() after remove {}", game.len());
    }    

    //println!("game: {:?}", game);

    let duration = start.elapsed();    

    println!("{}", looser_board_value);
    println!("duration {:?}", duration);
}

fn init_game_from_datafile(data: &str, drawed_numbers: &mut String, 
                            game: &mut Game){
    let mut curr_board : Board = [[Cell{value: 0, marked: false}; COL_SIZE]; ROW_SIZE];
    let mut row_counter = 0;
    let mut col_counter = 0;
    let mut boardid_counter = 1;

    let data_vec: Vec<&str> = data.lines().collect();
    let mut data_vec_iter = data_vec.iter();
    drawed_numbers.push_str(data_vec_iter.next().unwrap());
    
    //skip next line (is blank)
    data_vec_iter.next();

    //build boards
    for line in data_vec_iter{
        //println!("line: {}", line);
        //println!("line length: {}", line.len());
        if line.is_empty() || line.len() <= 2//blank linje med space istarten av neste linje skaper trøbbel. Aner ikke hvorfor???
        {
            //println!("line.is_empty == true");
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
                let gamerec = GameRec{id:boardid_counter, board: curr_board, bingo_value: 0, bingo: false };
                game.push(gamerec);
                boardid_counter += 1;
                //game.push(curr_board.clone());
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


fn calc_looser_board_value(board: &Board, drawed_number: u32) -> u32{
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

fn check_board_for_bingo(board: &mut Board)-> bool {
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