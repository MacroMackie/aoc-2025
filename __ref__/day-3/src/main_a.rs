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

    // Sum all the lines
    let sum = lines
        .iter()
        .map(|line| calculate_sum_for_program_string(line))
        .sum::<i32>();

    Ok(sum)
}

fn calculate_sum_for_program_string(str: &str) -> i32 {
    let mut sum = 0;
    // Use regex to find all mul(x,y) patterns
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Find all matches and calculate their products
    for cap in re.captures_iter(str) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        sum += x * y;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-3/input/test-a.txt";
        let result = run(filename);

        assert_eq!(result.unwrap(), 161);
    }
}
