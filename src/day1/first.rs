use crate::common;

pub fn main() {
    let calories = common::input::read_input("day1_1.txt");
    let mut count = 0;
    let mut highest = 0;
    for item in &calories {
        if item.len() == 0 {
            count = 0;
        } else {
            count = count + item.parse::<i32>().unwrap();
        }
        if count > highest {
            highest = count;
        }
    }
    println!("{highest}");
}
