use crate::common;
use std::collections::HashSet;

pub fn main() {
    let sacks = common::input::read_input("day3.txt");
    let mut groups: Vec<[&str; 3]> = Vec::new();
    let mut n = 0;
    while n < sacks.len() {
        let mut group: [&str; 3] = [""; 3];
        group[0] = &sacks[n];
        group[1] = &sacks[n+1];
        group[2] = &sacks[n+2];
        groups.push(group);
        n = n + 3;
    }
    let mut priorities = 0;
    for group in groups {
        let first_set: HashSet<char> = group[0].chars().collect();
        let second_set: HashSet<char> = group[1].chars().collect();
        let third_set: HashSet<char> = group[2].chars().collect();

        let first_two_sets: HashSet<&char> = first_set.intersection(&second_set).collect();
        let rust_silliness: HashSet<&char> = third_set.iter().collect();
        for badge in first_two_sets.intersection(&rust_silliness){
            priorities = priorities + compute_priority(badge);
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
