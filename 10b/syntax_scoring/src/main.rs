use std::time::{Instant};



fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");


    

    //let mut global_score: u32 = 0;

    let legal_beginning_chars = String::from("({[<");

    let mut autocomplete_score: Vec<u64> = Vec::new();

    //let mut incomplete_line_counter = 0;

    //let legal_ending_chars = String::from(")}]>");

    'outer: for line in data.lines(){
        let mut stack: Vec<char> = Vec::new();
        //println!("line {:?}", line);
        for curr_chr in line.chars(){
            if legal_beginning_chars.chars().any(|c| c==curr_chr) == true{
                stack.push(curr_chr);
                //println!("stack {:?}", stack);
            }
            else{
                let beginning_char = stack.pop();
                //println!("stack {:?}", stack);
                let legal_match = match curr_chr{
                    '}' => {if beginning_char.unwrap() == '{' {true} else {false}},
                    ')' => {if beginning_char.unwrap() == '(' {true} else {false}},
                    ']' => {if beginning_char.unwrap() == '[' {true} else {false}},
                    '>' => {if beginning_char.unwrap() == '<' {true} else {false}},
                    _ => panic!()
                };

                if !legal_match{
                    //println!("!legal_match");
                    /*
                    let illegal_score = match curr_chr{
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!()
                    };
                    */
                    //println!("ilegal_score {}", illegal_score);
                    //global_score += illegal_score;
                    //println!("global_score {}", global_score);
                    continue 'outer;
                }
            }
        }

        //Corrupt lines will never come here
        if stack.len() > 0{
            //autocomplete line
            let mut line_autocomplete_score : u64 = 0;
            let mut ending : String = String::new();
            while stack.len() > 0{
                //println!("stack.len() {}", stack.len());
                line_autocomplete_score *= 5;

                /*if incomplete_line_counter == 2{
                    println!("stack {:?}", stack);
                }*/

                let beginning_char = stack.pop();

                //find corresponding endchar score
                let temp_autocomplete_score = match beginning_char.unwrap(){
                    '(' => {ending.push(')');1},
                    '[' => {ending.push(']');2},
                    '{' => {ending.push('}');3},
                    '<' => {ending.push('>');4},
                    _ => panic!()
                };
                line_autocomplete_score += temp_autocomplete_score; 
                
                
            }
            //println!("ending {}", ending);
            //println!("line_autocomplete_score {}", line_autocomplete_score);
            autocomplete_score.push(line_autocomplete_score);

            //incomplete_line_counter += 1;
        }
    }

    let duration = start.elapsed();  

    autocomplete_score.sort();
    //println!("autocomplete_score {:?}", autocomplete_score);
    let score = autocomplete_score[autocomplete_score.len()/2 as usize];
    
    println!("{}", score);
    println!("duration {:?}", duration);
}
