use std::fs;
use regex::Regex;

fn part_1 (input_string: &str) -> i32 {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total_sum = 0;
    for cap in mul_regex.captures_iter(&input_string) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        total_sum += x * y;
    }
    total_sum
}
fn part_2 (input_string: &str) -> i32 {
    let mul_regex = Regex::new(r"(mul\((\d+),(\d+)\))|do\(\)|don't\(\)").unwrap();
    let mut total_sum = 0;
    let mut enabled = true;
    for cap in mul_regex.captures_iter(&input_string) {
        let string_match = cap[0].parse::<String>().unwrap();
        if string_match == "do()" {
            enabled = true;
        } else if string_match == "don't()" {
            enabled = false;
        } else {
            if enabled {
                let x: i32 = cap[2].parse().unwrap();
                let y: i32 = cap[3].parse().unwrap();
                total_sum += x * y;
            }
        }
    }
    total_sum
}
fn main() {
    let file_path = "src/assets/day_3.txt";
    let input_string = fs::read_to_string(file_path).unwrap();
    println!("Part 1: {}", part_1(&input_string));
    println!("Part 2: {}", part_2(&input_string));
}
