use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_DATA: &str = "data/input.txt";

fn main() {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    let reader = BufReader::new(File::open(INPUT_DATA).expect("Cannot open file.txt"));
    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<f64> = line
            .split("   ")
            .map(|s| s.parse().unwrap())
            .collect();
        left.push(values[0] as i64);
        right.push(values[1] as i64);
    }
    
    left.sort();
    right.sort();
    let mut distance: i64 = 0;
    assert_eq!(left.len(), right.len());
    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs();
    }
    println!("{}", distance);
}
