use crate::common;

pub fn main() {
    let rounds = common::input::read_input("day2.txt");
    let mut score = 0;
    for round in &rounds {
        let choices: Vec<&str> = round.split(' ').collect();
        let opponent_choice = choices[0];
        let result = choices[1];
        let opponent_value = opponent_choice.chars().next().unwrap() as u32; //we can just use the ASCII value of the characters and do math on them
        let result_value = result.chars().next().unwrap() as u32;
        score = score + (result_value - 88) * 3; //the round score is the ASCII value minus 88, times three
        let shape_value = ((opponent_value + result_value - 1) % 3) + 1; // shape score is both ASCII values added together minus 1 mod three plus 1 
        score = score + shape_value
    }
    println!("{score}");
}
