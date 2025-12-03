use utils::{load_env, load_file};

fn main() {
    load_env();

    let filename = "./day-2/input/input.txt";
    let result = calculate_calibration_values(filename).unwrap();

    println!("result: {}", result);
}

pub fn calculate_calibration_values(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let lines = load_file(filename)?;

    println!("lines: {:?}", lines);

    let array: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect()
        })
        .collect();

    println!("array: {:?}", array);

    let number_of_safe_reports = array.iter().filter(|row| is_safe_report(row)).count() as i32;

    println!("number_of_safe_reports: {}", number_of_safe_reports);

    Ok(number_of_safe_reports)
}

// Might be a better way to determine the exact index....
// This is n+1...
fn is_safe_report(row: &[i32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    fn check_sequence(sequence: &[i32]) -> bool {
        let mut increasing = true;
        let mut decreasing = true;

        for window in sequence.windows(2) {
            let diff = window[1] - window[0];

            if !(1..=3).contains(&diff) {
                increasing = false;
            }

            if !(-3..=-1).contains(&diff) {
                decreasing = false;
            }

            if !increasing && !decreasing {
                return false;
            }
        }

        true
    }

    // Check the full sequence first
    if check_sequence(row) {
        return true;
    }

    // If the full sequence fails, try removing one number
    for skip_pos in 0..row.len() {
        let filtered: Vec<i32> = row
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != skip_pos)
            .map(|(_, &x)| x)
            .collect();

        if check_sequence(&filtered) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-2/input/test-a.txt";
        let result = calculate_calibration_values(filename);

        assert_eq!(result.unwrap(), 4);
    }
}
