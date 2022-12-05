#![feature(buf_read_has_data_left)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_lines() -> BufReader<File> {
    let f: File = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    reader
}

fn initialise_score_map() -> HashMap<String, i32> {
    let mut score_map: HashMap<String, i32> = HashMap::new();

    // Rock
    score_map.insert("A X".to_string(), 4); // rock, draw
    score_map.insert("A Y".to_string(), 8); // paper, win
    score_map.insert("A Z".to_string(), 3); // scissors, lose

    // Paper
    score_map.insert("B X".to_string(), 1); // rock, lose
    score_map.insert("B Y".to_string(), 5); // paper, draw
    score_map.insert("B Z".to_string(), 9); // scissors, win

    // Scissors
    score_map.insert("C X".to_string(), 7); // rock, win
    score_map.insert("C Y".to_string(), 2); // paper, lose
    score_map.insert("C Z".to_string(), 6); // scissors, draw

    score_map
}

fn initialise_strategy_map() -> HashMap<String, i32> {
    let mut strategy_map: HashMap<String, i32> = HashMap::new();

    // Rock
    strategy_map.insert("A X".to_string(), 3); // lose: scissors
    strategy_map.insert("A Y".to_string(), 4); // draw: rock
    strategy_map.insert("A Z".to_string(), 8); // win: paper

    // Paper
    strategy_map.insert("B X".to_string(), 1); // lose: rock
    strategy_map.insert("B Y".to_string(), 5); // draw: paper
    strategy_map.insert("B Z".to_string(), 9); // win: scissors

    // Scissors
    strategy_map.insert("C X".to_string(), 2); // lose: paper
    strategy_map.insert("C Y".to_string(), 6); // draw: scissors
    strategy_map.insert("C Z".to_string(), 7); // win: rock

    strategy_map
}

fn main() {
    let mut reader = get_lines();
    let mut total_score = 0;
    let mut total_strategy_score = 0;
    let score_map: HashMap<String, i32> = initialise_score_map();
    let strategy_map: HashMap<String, i32> = initialise_strategy_map();
    while reader.has_data_left().unwrap() {
        let mut line: String = String::new();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();

        let score: i32 = *score_map.get(&line).unwrap();
        total_score += score;

        let strategy_score: i32 = *strategy_map.get(&line).unwrap();
        total_strategy_score += strategy_score;
    }
    println!("total score is: {}", total_score); // part 1
    println!("total strategy score is: {}", total_strategy_score); // part 2
}
