use utils::{load_env, load_file};

fn main() {
    load_env();

    let filename = "./day-4/input/input.txt";
    let result = run(filename).unwrap();

    println!("result: {}", result);
}

pub fn run(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let lines = load_file(filename)?;

    println!("lines: {:?}", lines);

    let array: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.split_whitespace().flat_map(|s| s.chars()).collect())
        .collect();

    println!("array: {:?}", array);

    let xmas_count = calculate_xmas_counts(array);

    println!("xmas_count: {}", xmas_count);

    Ok(xmas_count)
}

fn calculate_xmas_counts(array: Vec<Vec<char>>) -> i32 {
    let rows = array.len();
    if rows == 0 {
        return 0;
    }
    let cols = array[0].len();
    let mut match_count = 0;

    let diagonals = [
        [(-1, -1), (1, 1)], // Top-left to bottom-right
        [(-1, 1), (1, -1)], // Top-right to bottom-left
    ];

    for i in 0..rows {
        for j in 0..cols {
            if array[i][j] != 'A' {
                continue;
            }

            let mut matches = true;

            for diagonal in &diagonals {
                let mut chars = Vec::new();

                for &(dr, dc) in diagonal {
                    let ni = i as isize + dr;
                    let nj = j as isize + dc;

                    if ni < 0 || ni >= rows as isize || nj < 0 || nj >= cols as isize {
                        matches = false;
                        break;
                    }

                    chars.push(array[ni as usize][nj as usize]);
                }

                if !matches {
                    break;
                }

                let mut sorted_chars = chars.clone();
                sorted_chars.sort();

                if sorted_chars != vec!['M', 'S'] {
                    matches = false;
                    break;
                }
            }

            if matches {
                match_count += 1;
            }
        }
    }

    match_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-4/input/test-b.txt";
        let result = run(filename);

        assert_eq!(result.unwrap(), 9);
    }
}
