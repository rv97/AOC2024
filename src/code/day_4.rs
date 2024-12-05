use utils::utils::read_lines;

fn get_columns_as_strings(matrix: &Vec<Vec<char>>) -> Vec<String> {
    if matrix.is_empty() {
        return vec![];
    }

    let num_columns = matrix[0].len();
    let mut columns = vec![String::new(); num_columns];

    for row in matrix.iter() {
        for (col_idx, value) in row.iter().enumerate() {
            columns[col_idx].push_str(&value.to_string());
        }
    }

    columns
}
fn check_line (line: &str) -> u32 {
    let mut count = 0;
    let mut i: usize = 0;
    while i < line.len() {
        let curr_char = line.chars().nth(i).unwrap();
        if curr_char == 'X' {
            if i+3 < line.len() {
                let sub_str = &line[i..i+4];
                if sub_str == "XMAS" {
                    count += 1;
                }
            }
        }
        if curr_char == 'S' {
            if i+3 < line.len() {
                let sub_str = &line[i..i+4];
                if sub_str == "SAMX" {
                    count += 1;
                }
            }
        }
        i += 1;
    }
    count
}

fn check_diagonal (mat: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let m = mat.len();
    let n = mat[0].len();
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 'X' {
                if i+3 < m && j +3 < n {
                    let mut sub = String::from("");
                    sub.push(mat[i+1][j+1]);
                    sub.push(mat[i+2][j+2]);
                    sub.push(mat[i+3][j+3]);
                    if sub == "MAS" {
                        count += 1;
                    }
                }
                if i+3 < m && j >= 3 {
                    let mut sub = String::from("");
                    sub.push(mat[i+1][j-1]);
                    sub.push(mat[i+2][j-2]);
                    sub.push(mat[i+3][j-3]);
                    if sub == "MAS" {
                        count += 1;
                    }
                }
                if i >= 3 && j+3 < n {
                    let mut sub = String::from("");
                    sub.push(mat[i-1][j+1]);
                    sub.push(mat[i-2][j+2]);
                    sub.push(mat[i-3][j+3]);
                    if sub == "MAS" {
                        count += 1;
                    }
                }
                if i >= 3 && j >= 3 {
                    let mut sub = String::from("");
                    sub.push(mat[i-1][j-1]);
                    sub.push(mat[i-2][j-2]);
                    sub.push(mat[i-3][j-3]);
                    if sub == "MAS" {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn part_1 (mat: &Vec<Vec<char>>, contents: &Vec<String>) -> u32 {
    let mut total_count = 0;
    for line in contents {
        total_count += check_line(&line);
    }
    let columns = get_columns_as_strings(&mat);
    for column in columns {
        total_count += check_line(&column);
    }
    total_count += check_diagonal(&mat);
    total_count
}

fn part_2 (mat: &Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let m = mat.len();
    let n = mat[0].len();
    for i in 0..m {
        for j in 0..n {
            if mat[i][j] == 'A' {
                if i+1 < m && j+1 < n && i >= 1 && j >= 1 {
                    if mat[i+1][j+1] == 'S' && mat[i+1][j-1] == 'S' && mat[i-1][j-1] == 'M' && mat[i-1][j+1] == 'M' {
                        count += 1;
                    }
                    if mat[i+1][j+1] == 'M' && mat[i+1][j-1] == 'M' && mat[i-1][j-1] == 'S' && mat[i-1][j+1] == 'S' {
                        count += 1;
                    }
                    if mat[i+1][j+1] == 'M' && mat[i+1][j-1] == 'S' && mat[i-1][j-1] == 'S' && mat[i-1][j+1] == 'M' {
                        count += 1;
                    }
                    if mat[i+1][j+1] == 'S' && mat[i+1][j-1] == 'M' && mat[i-1][j-1] == 'M' && mat[i-1][j+1] == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
fn main () {
    let file_path = "src/assets/day_4.txt";
    let contents = read_lines(file_path);
    let mut mat: Vec<Vec<char>> = Vec::new();
    for line in &contents {
        mat.push(line.chars().collect());
    }
    println!("Part 1: {}", part_1(&mat, &contents));
    println!("Part 2: {}", part_2(&mat));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let sample_contents = "\
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX";
        let lines: Vec<String> = sample_contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        let mut mat: Vec<Vec<char>> = Vec::new();
        for line in &lines {
            mat.push(line.chars().collect());
        }
        let result = part_1(&mat, &lines);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_2() {
        let sample_contents = "\
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX";
        let lines: Vec<String> = sample_contents
            .lines()
            .map(|line| line.trim().to_string())
            .collect();
        let mut mat: Vec<Vec<char>> = Vec::new();
        for line in &lines {
            mat.push(line.chars().collect());
        }
        let result = part_2(&mat);
        assert_eq!(result, 9);
    }
}