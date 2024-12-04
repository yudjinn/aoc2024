use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;

fn main() {
    // let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
    let input = read_to_string("input.txt").unwrap();
    let res = split_and_compare(&input).unwrap();
    println!("{}", res);
    let res = calculate_similarity(&input).unwrap();
    println!("{}", res);
}

fn split_and_compare(input: &str) -> Result<i32> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut i = line.split_whitespace();
        left.push(i.next().unwrap().parse().unwrap());
        right.push(i.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    let mut output = 0;
    for i in 0..left.len() {
        output += (left[i] - right[i]).abs();
    }
    return Ok(output);
}

fn calculate_similarity(input: &str) -> Result<i32> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut i = line.split_whitespace();
        left.push(i.next().unwrap().parse().unwrap());
        right.push(i.next().unwrap().parse().unwrap());
    }

    let mut sim: HashMap<i32, i32> = HashMap::new();
    for v in left.clone() {
        sim.insert(v, right.iter().filter(|&n| *n == v).count() as i32);
    }
    let out: Vec<i32> = left.iter().map(|x| *x * sim.get(x).unwrap_or(&0)).collect();
    Ok(out.iter().sum())
}
