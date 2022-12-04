#![feature(buf_read_has_data_left)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;

fn get_highest_calories() -> Result<i32> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut highest_elf_total: i32 = 0;
    let mut current_elf_total: i32 = 0;
    while reader.has_data_left().unwrap() {
        let mut line: String = String::new();
        reader.read_line(&mut line).unwrap();
        if line == "\n" {
            highest_elf_total = if current_elf_total > highest_elf_total {
                current_elf_total
            } else {
                highest_elf_total
            };
            current_elf_total = 0;
        } else {
            let num_calories: i32 = line.trim().parse().unwrap();
            current_elf_total += num_calories;
        }
    }

    Ok(highest_elf_total)
}

fn main() {
    let highest_calories = get_highest_calories().unwrap();
    println!("Highest total calories: {highest_calories}");
}
