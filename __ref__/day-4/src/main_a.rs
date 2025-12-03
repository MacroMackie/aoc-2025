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
    const TARGET: &[char] = &['X', 'M', 'A', 'S'];
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, 0),  // Up
        (-1, 1),  // Up-Right
        (0, 1),   // Right
        (1, 1),   // Down-Right
        (1, 0),   // Down
        (1, -1),  // Down-Left
        (0, -1),  // Left
        (-1, -1), // Up-Left
    ];

    let rows = array.len();

    if rows == 0 {
        return 0;
    }

    let cols = array[0].len();

    let mut match_count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(delta_row, delta_col) in &DIRECTIONS {
                let mut is_sequence_matched = true;
                let mut current_row = i as isize;
                let mut current_col = j as isize;

                for &expected_char in TARGET {
                    if current_row < 0
                        || current_row >= rows as isize
                        || current_col < 0
                        || current_col >= cols as isize
                        || array[current_row as usize][current_col as usize] != expected_char
                    {
                        is_sequence_matched = false;
                        break;
                    }
                    current_row += delta_row;
                    current_col += delta_col;
                }

                if is_sequence_matched {
                    match_count += 1;
                }
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
        let filename = "./day-4/input/test-a.txt";
        let result = run(filename);

        assert_eq!(result.unwrap(), 18);
    }
}
