use utils::utils::read_lines;
fn part_1(file_contents: &[String]) -> u32 {
    let mut report_count: u32 = 0;
    for item in file_contents {
        let tokens: Vec<i32> = item
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_safe(&tokens) {
            report_count += 1;
        }
    }

    report_count
}
fn part_2(file_contents: &[String]) -> u32 {
    let mut report_count: u32 = 0;
    for item in file_contents {
        let tokens: Vec<i32> = item
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if is_safe(&tokens) || can_be_made_safe(&tokens) {
            report_count += 1;
        }
    }

    report_count
}

fn is_safe(levels: &[i32]) -> bool {
    let mut is_increasing = None;

    for pair in levels.windows(2) {
        let diff = pair[1] - pair[0];

        if diff == 0 {
            return false;
        }

        if is_increasing.is_none() {
            is_increasing = Some(diff > 0);
        }

        let valid_diff = match is_increasing {
            Some(true) => (1..=3).contains(&diff),
            Some(false) => (-3..=-1).contains(&diff),
            _ => unreachable!(),
        };

        if !valid_diff {
            return false;
        }
    }

    true
}

fn can_be_made_safe(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);

        if is_safe(&modified) {
            return true;
        }
    }

    false
}

fn main() {
    let file_path = "src/assets/day_2.txt";
    let contents = read_lines(file_path);
    println!("\nPart 1: {}", part_1(&contents));
    println!("\nPart 2: {}", part_2(&contents));
}

