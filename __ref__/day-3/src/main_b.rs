use regex::Regex;
use utils::{load_env, load_file};

fn main() {
    load_env();

    let filename = "./day-3/input/input.txt";
    let result = run(filename).unwrap();

    println!("result: {}", result);
}

pub fn run(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let lines = load_file(filename)?;

    println!("lines: {:?}", lines);

    // Combine all the lines into a single string
    let combined_lines = lines.join("");

    // Sum all the lines
    let sum = calculate_sum_for_program_string(&combined_lines);

    Ok(sum)
}

// Start of Selection
fn calculate_sum_for_program_string(str: &str) -> i32 {
    let mut sum = 0;
    let mut mul_enabled = true;

    // Use regex to find do(), don't(), and mul(x,y) patterns
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    // Iterate through all matches and process instructions
    for cap in re.captures_iter(str) {
        if &cap[0] == "do()" {
            mul_enabled = true;
        } else if &cap[0] == "don't()" {
            mul_enabled = false;
        } else if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
            if mul_enabled {
                let x: i32 = x.as_str().parse().unwrap();
                let y: i32 = y.as_str().parse().unwrap();
                sum += x * y;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-3/input/test-b.txt";
        let result = run(filename);

        assert_eq!(result.unwrap(), 48);
    }
}
