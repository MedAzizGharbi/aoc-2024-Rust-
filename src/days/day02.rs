use std::fs;

pub fn run() {
    let path = "inputs/day02.txt";
    let data = fs::read_to_string(path).unwrap();
    println!("Part 1 => {}", part1(data));
}

fn part1(input: String) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let n = report.len();
        let mut valid = true;
        if report[1] > report[0] {
            for i in 1..n {
                let diff = report[i] - report[i - 1];
                if !(diff > 0 && diff <= 3) {
                    valid = false;
                    break;
                }
            }
            if valid {
                result += 1;
            }
        } else if report[1] < report[0] {
            for i in 1..n {
                let diff = report[i] - report[i - 1];
                if !(diff < 0 && diff >= -3) {
                    valid = false;
                    break;
                }
            }
            if valid {
                result += 1;
            }
        }
    }
    result
}
