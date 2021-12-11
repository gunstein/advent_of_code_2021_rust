use std::time::{Instant};
use std::collections::HashSet;


fn main() {
    let start = Instant::now();
    let data = include_str!("data.txt");
    //let data = include_str!("data_small.txt");

    let mut sum_all_output_values: u32 = 0;

    for line in data.lines(){
        let all_digits : Vec<&str> = line.split(" | ").collect();
        let input_digits = all_digits[0].split_whitespace();

        const EMPTY_STRING: String = String::new();
        let mut placed_digits : [String; 10]  = [EMPTY_STRING; 10];
        
        //find the easy four
        let mut group_0_6_9 : Vec<String> = Vec::new();
        let mut group_2_3_5 : Vec<String> = Vec::new();
        for digit in input_digits{
            let digit_len = digit.len();
            match digit_len{
                2 => {placed_digits[1] = digit.to_string();},
                3 => {placed_digits[7] = digit.to_string();},
                4 => {placed_digits[4] = digit.to_string();},
                5 => {group_2_3_5.push(digit.to_string())}, 
                6 => {group_0_6_9.push(digit.to_string())}, 
                7 => {placed_digits[8] = digit.to_string();},
                _ => {},
            }
        }

        //find 6, 6chr lack 1chr in (0, 6, 9)
        let set1: HashSet<char> = placed_digits[1].chars().collect();
        for digit in group_0_6_9.iter(){
            let curr_digit: HashSet<char> = digit.chars().collect();
            if !set1.is_subset(&curr_digit){//all digits of one is in 6 and 9, but not 6
                let temp : String = curr_digit.iter().collect();
                placed_digits[6] = temp.to_string();
            }
        }
        
        //find 3, 3chr is only one containing all digits from 1chr in (2, 3, 5)
        for digit in group_2_3_5.iter(){
            let curr_digit: HashSet<char> = digit.chars().collect();
            if set1.is_subset(&curr_digit){//all digits of one is in 3, but not in 2 and 5
                let temp : String = curr_digit.iter().collect();
                placed_digits[3] = temp.to_string();
            }
        } 
   
        //find 9, 9chr is only one containing all digits from 3chr in (0, 6, 9)
        let set3: HashSet<char> = placed_digits[3].chars().collect();
        for digit in group_0_6_9.iter(){
            let curr_digit: HashSet<char> = digit.chars().collect();
            if set3.is_subset(&curr_digit){
                let temp : String = curr_digit.iter().collect();
                placed_digits[9] = temp.to_string();
            }
        }  
        
        //find 0. The one remaining in (0, 6, 9)
        let set6: HashSet<char> = placed_digits[6].chars().collect();
        let set9: HashSet<char> = placed_digits[9].chars().collect();
        for digit in group_0_6_9.iter(){
            let curr_digit: HashSet<char> = digit.chars().collect();
            if !set6.is_subset(&curr_digit) && !set9.is_subset(&curr_digit){
                let temp : String = curr_digit.iter().collect();
                placed_digits[0] = temp.to_string();
            }
        } 
        
        //find upper_left_digit, needed to find 5 in next step. Digits i 9 - 3.
        let upper_left_digit: HashSet<char> = set9.difference(&set3).copied().collect();
        //find 5, 5chr is only one containing upper_left_digit in (2, 3, 5)
        for digit in group_2_3_5.iter(){
            let curr_digit: HashSet<char> = digit.chars().collect();
            if upper_left_digit.is_subset(&curr_digit){//all digits of one is in 3, but not in 2 and 5
                let temp : String = curr_digit.iter().collect();
                placed_digits[5] = temp.to_string();
            }
        } 

        //find 2. The one remaining in (2, 3, 5)
        let set5: HashSet<char> = placed_digits[5].chars().collect();
        for digit in group_2_3_5.iter(){
            let curr_digit: HashSet<char> = digit.chars().collect();
            if !set3.is_subset(&curr_digit) && !set5.is_subset(&curr_digit){
                let temp : String = curr_digit.iter().collect();
                placed_digits[2] = temp.to_string();
            }
        }         
        
        //println!("placed_digits {:?}", placed_digits);
        //Decode output digits
        let mut str_output_number : String = String::new();
        let output_digits = all_digits[1].split_whitespace();
        //println!("output_digits {:?}", output_digits);
        for digit in output_digits{
            let set_curr_digit: HashSet<char> = digit.chars().collect();
            for i in 0..9+1{
                let set_curr_output: HashSet<char> = placed_digits[i].chars().collect();
                if set_curr_digit == set_curr_output{
                    str_output_number = format!("{}{}", str_output_number, i);
                    break;
                }
            }
        }
        //println!("str_output_number {}", str_output_number);

        sum_all_output_values += str_output_number.parse::<u32>().unwrap();

    }
    
    let duration = start.elapsed();  
    
    println!("{}", sum_all_output_values);
    println!("duration {:?}", duration);
}

/*
//from https://www.rosettacode.org/wiki/Strip_a_set_of_characters_from_a_string#Rust
fn strip_characters(original : &str, to_strip : &str) -> String {
    original.chars().filter(|&c| !to_strip.contains(c)).collect()
}*/