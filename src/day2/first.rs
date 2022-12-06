use crate::common;

pub fn main() {
    let rounds = common::input::read_input("day2.txt");
    let mut score = 0;
    for round in &rounds {
        let choices: Vec<&str> = round.split(' ').collect();
        let opponent_choice = choices[0];
        let my_choice = choices[1];
        let opponent_value = opponent_choice.chars().next().unwrap() as u32; //we can just use the ASCII value of the characters and do math on them
        let my_value = my_choice.chars().next().unwrap() as u32;
        score = score + (my_value - 87); //the shape score is the ASCII value minus 87
        let result = ((my_value - opponent_value) - 1) % 3;
        score = score + (result * 3); //the round score is the ASCII value of my choice minus the ASCII value of opponent's choice, minus 1, mod three, times three :)
    }
    println!("{score}");
}
