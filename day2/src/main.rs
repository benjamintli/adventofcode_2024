use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_DATA: &str = "data/input.txt";

fn is_increasing(level: &Vec<i32>) -> bool{
    let mut prev = level[0];
    for i in 1..level.len() {
        if level[i] <= prev {
            return false;
        }
        prev = level[i];
    }
    return true;
}

fn is_decreasing(level: &Vec<i32>) -> bool{
    let mut prev = level[0];
    for i in 1..level.len() {
        if level[i] >= prev {
            return false;
        }
        prev = level[i];
    }
    return true;
}

fn main() {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    let reader: BufReader<File> = BufReader::new(File::open(INPUT_DATA).expect("Cannot open file.txt"));
    for line in reader.lines() {
        let line = line.expect("Cannot read line");
        let level = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        reports.push(level);
    }
    let mut safe_levels = reports.len();
    for level in reports {
        let mut prev: i32 = level[0];
        if is_increasing(&level) || is_decreasing(&level) {
            for i in 1..level.len() {
                let diff: i32 = level[i] - prev;
                if !(1 <= diff.abs() && diff.abs() <= 3) {
                    println!("{:?} is unsafe", level);
                    safe_levels -= 1;
                    break;
                }
                prev = level[i];
            }
        } else {
            safe_levels -= 1;
        }
    }
    println!("{}", safe_levels);
}
