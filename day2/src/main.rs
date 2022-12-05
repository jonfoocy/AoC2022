#![feature(buf_read_has_data_left)]

use std::{
    clone,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum RPSValue {
    Rock,
    Paper,
    Scissors,
    Gun,
}

#[derive(Debug, PartialEq, Eq)]
enum RPSResult {
    Win,
    Draw,
    Loss,
    Invalid,
}

fn get_lines() -> BufReader<File> {
    let f: File = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    reader
}

fn get_rps_value(c: char) -> RPSValue {
    match c {
        'A' | 'X' => RPSValue::Rock,
        'B' | 'Y' => RPSValue::Paper,
        'C' | 'Z' => RPSValue::Scissors,
        _ => RPSValue::Gun,
    }
}

fn get_result(opp: RPSValue, me: RPSValue) -> RPSResult {
    if opp == me {
        RPSResult::Draw
    } else {
        if opp == RPSValue::Rock {
            if me == RPSValue::Paper {
                RPSResult::Win
            } else {
                RPSResult::Loss
            }
        } else if opp == RPSValue::Paper {
            if me == RPSValue::Rock {
                RPSResult::Loss
            } else {
                RPSResult::Win
            }
        } else if opp == RPSValue::Scissors {
            if me == RPSValue::Rock {
                RPSResult::Win
            } else {
                RPSResult::Loss
            }
        } else {
            RPSResult::Invalid
        }
    }
}

fn get_score(me: RPSValue, result: RPSResult) -> i32 {
    let mut score: i32 = 0;
    if me == RPSValue::Rock {
        score += 1;
    } else if me == RPSValue::Paper {
        score += 2;
    } else {
        score += 3;
    }

    if result == RPSResult::Win {
        score += 6;
    } else if result == RPSResult::Draw {
        score += 3;
    } else {
        score += 0;
    }

    score
}

fn main() {
    let mut reader = get_lines();
    let mut totalScore = 0;
    while reader.has_data_left().unwrap() {
        let mut line: String = String::new();
        reader.read_line(&mut line).unwrap();

        let opp = get_rps_value(line.chars().nth(0).unwrap());
        let me = get_rps_value(line.chars().nth(2).unwrap());
        let me_clone = me.clone();
        // println!("opp: {:?}, me: {:?}", opp, me);

        let result = get_result(opp, me);
        // println!("result: {:?}", result);
        let score = get_score(me_clone, result);
        // println!("score: {}", score);
        totalScore += score;

        // let println!("{line}");
        // count += 1;
        // if count == 2 {
        //     break;
        // }
    }
    println!("total score is: {}", totalScore);
}
