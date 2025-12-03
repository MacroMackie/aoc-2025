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

fn is_safe_report(row: &[i32]) -> bool {
    if row.len() < 2 {
        return true;
    }

    // get the differences between each pair of adjacent levels
    let diffs: Vec<_> = row.windows(2).map(|w| w[1] - w[0]).collect();

    // check if all the differences are either all increasing or all decreasing
    let all_increasing = diffs.iter().all(|&d| (1..=3).contains(&d));
    let all_decreasing = diffs.iter().all(|&d| (-3..=-1).contains(&d));

    all_increasing || all_decreasing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-2/input/test-a.txt";
        let result = calculate_calibration_values(filename);

        assert_eq!(result.unwrap(), 2);
    }
}
