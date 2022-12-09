#![feature(buf_read_has_data_left)]

extern crate utils;

fn convert_range_to_i32_tuple(str: &str) -> (i32, i32) {
    let (low_str, high_str) = str.split_once('-').unwrap();
    let low: i32 = low_str.parse().unwrap();
    let high: i32 = high_str.parse().unwrap();
    (low, high)
}

fn is_enclosing(first: (i32, i32), second: (i32, i32)) -> bool {
    let first_low = first.0;
    let first_high = first.1;
    let second_low = second.0;
    let second_high = second.1;
    first_low <= second_low && first_high >= second_high
}

fn is_overlap(first: (i32, i32), second: (i32, i32)) -> bool {
    let first_low = first.0;
    let first_high = first.1;
    let second_low = second.0;
    let second_high = second.1;
    is_enclosing(first, second)
        || second_high >= first_low && second_high <= first_high
        || second_low <= first_high && second_high >= first_high
}

fn part1() {
    let file_path = "input.txt".to_string();
    let mut reader = utils::get_file_reader(&file_path);

    let mut total_enclosures: i32 = 0;
    while reader.has_data_left().unwrap() {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();

        let (first, second) = line.split_once(',').unwrap();
        let first_range = convert_range_to_i32_tuple(first);
        let second_range = convert_range_to_i32_tuple(second);
        let encloses =
            is_enclosing(first_range, second_range) || is_enclosing(second_range, first_range);
        if encloses {
            total_enclosures += 1;
        }
    }

    println!("total enclosures: {total_enclosures}");
}

fn part2() {
    let file_path = "input.txt".to_string();
    let mut reader = utils::get_file_reader(&file_path);

    let mut total_overlaps: i32 = 0;
    while reader.has_data_left().unwrap() {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();

        let (first, second) = line.split_once(',').unwrap();
        let first_range = convert_range_to_i32_tuple(first);
        let second_range = convert_range_to_i32_tuple(second);
        if is_overlap(first_range, second_range) {
            total_overlaps += 1;
        }
    }
    println!("total overlaps: {total_overlaps}");
}

fn main() {
    part1();
    part2();
}
