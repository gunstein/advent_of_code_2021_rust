use std::time::{Instant};



fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");


    let mut stack: Vec<char> = Vec::new();

    let mut global_score: u32 = 0;

    let legal_beginning_chars = String::from("({[<");
    //let legal_ending_chars = String::from(")}]>");
    'outer: for line in data.lines(){
        /*
        let count_begin_chars = line.matches("(").count() + line.matches("{").count() + line.matches("[").count() + line.matches("<").count();
        //let count_end_chars = 16;
        let count_end_chars = line.matches(")").count() + line.matches("}").count() + line.matches("]").count() + line.matches(">").count();
        println!("count_begin_chars {}", count_begin_chars);
        println!("count_end_chars {}", count_end_chars);
        
        if count_begin_chars == count_end_chars {
            println!("continue ");
            continue 'outer;
        }
        */
        //println!("line {}", line);
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
                    let illegal_score = match curr_chr{
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => panic!()
                    };
                    //println!("ilegal_score {}", illegal_score);
                    global_score += illegal_score;
                    //println!("global_score {}", global_score);
                    continue 'outer;
                }
            }
        }
    }

    let duration = start.elapsed();  
    
    println!("{}", global_score);
    println!("duration {:?}", duration);
}
