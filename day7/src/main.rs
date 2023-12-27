mod part1;
mod part2;
use std::{path::PathBuf};
use part1::part1;
use part2::part2;

fn main() {
    let input = PathBuf::from("input.txt");
    let res_1 = part1(&input);
    println!("{}", res_1);
    let res_2 = part2(&input);
    println!("{}", res_2);
}


