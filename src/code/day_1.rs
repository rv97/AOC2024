use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use utils::utils::read_lines;

fn part_1() -> u32 {
    let file_path = "src/assets/day_1_part_1.txt";
    let mut local_vec1: Vec<i32> = vec![];
    let mut local_vec2: Vec<i32> = vec![];
    let mut heap1 = BinaryHeap::new();
    let mut heap2 = BinaryHeap::new();
    let mut total_diff: u32 = 0;

    let contents = read_lines(file_path);
    for item in contents {
        let tokens: Vec<&str> = item.split_whitespace().collect();
        local_vec1.push(tokens[0].parse::<i32>().unwrap());
        local_vec2.push(tokens[1].parse::<i32>().unwrap());
    }

    for n in local_vec1 {
        heap1.push(Reverse(n));
    }
    for k in local_vec2 {
        heap2.push(Reverse(k));
    }
    while heap1.len() > 0 && heap2.len() > 0 {
        let local_diff = (heap2.pop().unwrap().0 - heap1.pop().unwrap().0).abs();
        total_diff += local_diff as u32;
    }
    total_diff
}

fn part_2() -> u32 {
    let file_path = "src/assets/day_1_part_2.txt";
    let contents = read_lines(file_path);
    let mut num1 = Vec::new();
    let mut frequency_map = HashMap::new();
    let mut total_score: u32 = 0;

    for item in contents {
        let tokens: Vec<&str> = item.split_whitespace().collect();
        num1.push(tokens[0].to_string());
        *frequency_map.entry(tokens[1].to_string()).or_insert(0) += 1;
    }

    for num in num1 {
        match frequency_map.get_mut(&num) {
            Some(&mut freq) => total_score += (num.parse::<i32>().unwrap() * freq) as u32,
            None => {}
        }
    }
    total_score
}
pub fn main (){
    println!("Part1: {}", part_1());
    println!("Part2: {}", part_2());
}