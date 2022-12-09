#![feature(buf_read_has_data_left)]

use std::{collections::HashSet, io::BufRead};

extern crate utils;

fn get_priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return c as u32 - 96;
    } else {
        return c as u32 - 38;
    }
}

fn part1() {
    let file_path: String = "input.txt".to_string();
    let mut reader = utils::get_file_reader(&file_path);

    let mut total_priority: u32 = 0;
    while reader.has_data_left().unwrap() {
        // Step 1: get line
        let mut line: String = String::new();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();

        // Step 2: split line into half
        let (first_half, second_half) = line.split_at(line.len() / 2);

        // Step 3: find common item in each half
        // build hashset for firsthalf
        let first_half_set: HashSet<char> = first_half.chars().collect();
        let common_char: char = second_half
            .chars()
            .find(|&c| first_half_set.contains(&c))
            .unwrap();
        let priority: u32 = get_priority(common_char);
        total_priority += priority;
    }
    println!("part 1 total priority: {}", total_priority);
}

fn part2() {
    let file_path: String = "input.txt".to_string();
    let mut reader = utils::get_file_reader(&file_path);

    let mut line_num: u8 = 1;
    let mut total_priority: u32 = 0;
    let mut first_set: HashSet<char> = HashSet::new();
    let mut first_second_set: HashSet<char> = HashSet::new();
    let mut common_char: char;
    while reader.has_data_left().unwrap() {
        let mut line: String = String::new();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();

        match line_num {
            1 => {
                first_set = line.chars().collect();
                line_num = 2;
            }
            2 => {
                first_second_set = line.chars().filter(|&c| first_set.contains(&c)).collect();
                line_num = 3;
            }
            3 => {
                common_char = line
                    .chars()
                    .find(|&c| first_second_set.contains(&c))
                    .unwrap();
                line_num = 1;
                // println!("common char is {common_char}");
                total_priority += get_priority(common_char);
            }
            _ => println!("wrong line_num {line_num}"),
        }
    }
    println!("part 2 total priority is {}", total_priority);
}

fn main() {
    part1();
    part2();
}
