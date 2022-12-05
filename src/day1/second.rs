use crate::common;

pub fn main() {
    let calories = common::input::read_input("day1.txt");
    let mut elf_calories = Vec::new();
    let mut count = 0;
    for item in &calories {
        if item.len() == 0 {
            elf_calories.push(count);
            count = 0;
        } else {
            count = count + item.parse::<i32>().unwrap();
        }
    }
    elf_calories.push(count); //add the final elf
    elf_calories.sort();
    elf_calories.reverse();
    let sum = elf_calories[0] + elf_calories[1] + elf_calories[2];
    println!("{sum}");
}
