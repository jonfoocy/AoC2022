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

fn get_lowest_index_and_number(numbers: &[i32]) -> (usize, i32) {
    let mut lowest_index: i32 = -1;
    let mut lowest_number: i32 = -1;
    for (i, number) in numbers.iter().enumerate() {
        if lowest_index == -1 && lowest_number == -1 {
            lowest_index = i as i32;
            lowest_number = *number;
        } else {
            if number <= &lowest_number {
                lowest_number = *number;
                lowest_index = i as i32;
            }
        }
    }

    (lowest_index as usize, lowest_number)
}

fn get_total_highest_3_calories() -> Result<i32> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut highest_calories: [i32; 3] = [0; 3];
    let mut lowest_idx: usize = 0;
    let mut lowest_elf_total: i32 = 0;

    let mut current_elf_total: i32 = 0;
    while reader.has_data_left().unwrap() {
        let mut line: String = String::new();
        reader.read_line(&mut line).unwrap();
        if line == "\n" {
            if current_elf_total > lowest_elf_total {
                highest_calories[lowest_idx] = current_elf_total;
                (lowest_idx, lowest_elf_total) = get_lowest_index_and_number(&highest_calories);
            }
            current_elf_total = 0;
        } else {
            let num_calories: i32 = line.trim().parse().unwrap();
            current_elf_total += num_calories;
        }
    }

    Ok(highest_calories.iter().sum())
}

fn main() {
    let highest_calories = get_highest_calories().unwrap();
    println!("Highest total calories: {highest_calories}");
    let highest_3_calories: i32 = get_total_highest_3_calories().unwrap();
    println!("Highest 3 total calories: {highest_3_calories}");
}
