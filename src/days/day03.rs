use regex::Regex;
use std::fs;
pub fn run() {
    let input = "inputs/day03.txt";
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let data = fs::read_to_string(input).unwrap();

    // Brief explanation of the regex
    // mul\( → literally matches mul(.
    //
    // (\d{1,3}) → first capture group: matches 1 to 3 digits
    //
    // , → literal comma between numbers.
    //
    // (\d{1,3}) → second capture group: another 1–3 digit number.
    //
    // \) → literal closing parenthesis.

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(&data)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let data = fs::read_to_string(input).unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for cap in re.captures_iter(&data) {
        if let Some(mul) = cap.get(0) {
            let token = mul.as_str();
            match token {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ if token.starts_with("mul(") && enabled => {
                    let x: i32 = cap[1].parse().unwrap();
                    let y: i32 = cap[2].parse().unwrap();
                    sum += x * y;
                }
                _ => {}
            }
        }
    }
    sum
}
