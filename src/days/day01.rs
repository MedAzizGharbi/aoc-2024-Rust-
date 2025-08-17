use std::collections::{HashMap, HashSet};
use std::fs;
pub fn run() {
    let input = "inputs/day01.txt";
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let data = fs::read_to_string(input).unwrap();
    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();
    for line in data.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        x.push(v[0].parse::<i32>().unwrap());
        y.push(v[1].parse::<i32>().unwrap());
    }
    x.sort();
    y.sort();
    let mut result: i32 = 0;
    for i in 0..x.len() {
        result += (x[i] - y[i]).abs();
    }
    result
}

fn part2(input: &str) -> i32 {
    let data = fs::read_to_string(input).unwrap();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut right = Vec::new();
    for line in data.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        right.push(v[0].parse::<i32>().unwrap());
        *map.entry(v[1].parse::<i32>().unwrap()).or_insert(0) += 1;
    }
    let mut result = 0;
    for i in right {
        let x = map.get(&i).unwrap_or(&0);
        result += i * x;
    }
    result
}
