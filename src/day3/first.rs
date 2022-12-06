use crate::common;
use std::collections::HashSet;

pub fn main() {
    let sacks = common::input::read_input("day3.txt");
    let mut priorities = 0;
    for sack in &sacks {
        let size = sack.len();
        let (first_half, second_half) = sack.split_at(size / 2);
        let first_set: HashSet<char> = first_half.chars().collect();
        let second_set: HashSet<char> = second_half.chars().collect();
        for item in first_set.intersection(&second_set) {
            priorities = priorities + compute_priority(item);
        }
    }
    println!("{priorities}");
}

fn compute_priority(item: &char) -> u32 {
    if item.is_ascii_uppercase() {
        return *item as u32 - 38;
    } else {
        return *item as u32 - 96;
    }
}
