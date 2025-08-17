use std::fs;

pub fn run() {
    let path = "inputs/day02.txt";
    let data = fs::read_to_string(path).unwrap();
    println!("Part 1 => {}", part1(&data));
    println!("Part 2 => {}", part2(&data));
}

fn part1(input: &String) -> i32 {
    input
        .lines()
        .map(|line| {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            if is_safe(&report) { 1 } else { 0 }
        })
        .sum()
}
fn part2(input: &String) -> i32 {
    input
        .lines()
        .map(|line| {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            if is_safe(&report) {
                return 1;
            } else {
                for i in 0..report.len() {
                    let mut reduced = report.clone();
                    reduced.remove(i);
                    if is_safe(&reduced) {
                        return 1;
                    }
                }
            }
            0
        })
        .sum()
}
fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }
    let increasing = report[1] > report[0];

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        if increasing {
            if !(diff > 0 && diff <= 3) {
                return false;
            }
        } else {
            if !(diff < 0 && diff >= -3) {
                return false;
            }
        }
    }
    true
}
