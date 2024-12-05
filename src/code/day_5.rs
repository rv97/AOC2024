use utils::utils::read_lines;
use std::collections::HashMap;

struct Information {
    ordering_rules: HashMap<usize, Vec<usize>>,
    page_updates: Vec<Vec<usize>>,
}

struct Updates<'a> {
    valid_updates: Vec<&'a Vec<usize>>,
    invalid_updates: Vec<&'a Vec<usize>>,
}

fn form_ordering_and_updates (contents: Vec<String>) -> Information {
    let mut ordering_rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut page_updates: Vec<Vec<usize>> = Vec::new();
    let mut has_updates_started:bool = false;
    for line in contents {
        if line == "" {
            has_updates_started = true;
            continue;
        }
        if !has_updates_started {
            let tokens = line.split("|").collect::<Vec<&str>>();
            ordering_rules.entry(tokens[0].parse::<usize>().unwrap_or(0)).or_default().push( tokens[1].parse::<usize>().unwrap_or(0));
        } else {
            let tokens = line.split(",").collect::<Vec<&str>>();
            let mut tokens_i64: Vec<usize> = Vec::new();
            for token in tokens {
                tokens_i64.push(token.parse::<usize>().unwrap_or(0));
            }
            page_updates.push(tokens_i64);
        }
    }
    Information {
        ordering_rules,
        page_updates,
    }
}

fn find_valid_and_invalid_updates (info: &Information) -> Updates {
    let mut valid_updates: Vec<&Vec<usize>> = Vec::new();
    let mut invalid_updates: Vec<&Vec<usize>> = Vec::new();
    for update in &info.page_updates {
        let mut valid_update = true;
        for i in 0..update.len() {
            let empty_vec: Vec<usize> = Vec::new();
            let vec = info.ordering_rules.get(&update[i]).unwrap_or(&empty_vec);
            for j in i+1..update.len() {
                if vec.contains(&update[j]) {
                    valid_update = true;
                } else {
                    valid_update = false;
                    break;
                }
            }
            if !valid_update {
                break;
            }
        }
        if valid_update {
            valid_updates.push(update);
        } else {
            invalid_updates.push(update);
        }
    }
    Updates {
        valid_updates,
        invalid_updates
    }
}
fn part_1 (valid_updates: Vec<&Vec<usize>>) -> usize {
    let mut total_sum: usize = 0;
    for valid in valid_updates {
        total_sum += valid[valid.len()/2];
    }
    total_sum
}

fn part_2 (invalid_updates: Vec<&Vec<usize>>, info: &Information) -> usize {
    let mut total_sum: usize = 0;
    for invalid in invalid_updates {
        let mut temp_vec = vec![0; invalid.len()];
        for i in 0..invalid.len() {
            let empty_vec: Vec<usize> = Vec::new();
            let vec = info.ordering_rules.get(&invalid[i]).unwrap_or(&empty_vec);
            let mut count = 0;
            for j in 0..invalid.len() {
                if vec.contains(&invalid[j]) {
                    count += 1;
                }
            }
            let len = temp_vec.len();
            temp_vec[len - count - 1] = invalid[i];
        }
        total_sum += temp_vec[temp_vec.len()/2];
    }
    total_sum
}

fn main () {
    let file_path = "src/assets/day_5.txt";
    let contents = read_lines(file_path);
    let info = form_ordering_and_updates(contents);
    let update = find_valid_and_invalid_updates(&info);

    println!("Part 1: {}", part_1(update.valid_updates));
    println!("Part 2: {}",part_2(update.invalid_updates, &info));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let sample_contents = "\
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";
        let lines: Vec<String> = sample_contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        let info = form_ordering_and_updates(lines);
        let update = find_valid_and_invalid_updates(&info);
        let result = part_1(update.valid_updates);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_2() {
        let sample_contents = "\
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47";
        let lines: Vec<String> = sample_contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        let info = form_ordering_and_updates(lines);
        let update = find_valid_and_invalid_updates(&info);
        let result = part_2(update.invalid_updates, &info);
        assert_eq!(result, 123);
    }
}