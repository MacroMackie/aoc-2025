use std::collections::HashMap;

use utils::{load_env, load_file};

fn main() {
    load_env();

    let filename = "./day-1/input/input.txt";
    let result = calculate_calibration_values(filename).unwrap();

    println!("result: {}", result);
}

pub fn calculate_calibration_values(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let lines = load_file(filename)?;

    println!("lines: {:?}", lines);

    let (list_a, list_b): (Vec<i32>, Vec<i32>) = lines
        .iter()
        .filter_map(|line| {
            line.split_once(' ').and_then(|(first, second)| {
                Some((
                    first.trim().parse::<i32>().ok()?,
                    second.trim().parse::<i32>().ok()?,
                ))
            })
        })
        .unzip();

    println!("list_a: {:?}", list_a);
    println!("list_b: {:?}", list_b);

    let freq_map: HashMap<i32, usize> = list_b.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    println!("Frequency Map for List B: {:?}", freq_map);

    let similarity_score: i32 = list_a
        .iter()
        .map(|&num| num * (*freq_map.get(&num).unwrap_or(&0) as i32))
        .sum();

    println!("Similarity Score: {}", similarity_score);

    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-1/input/test-b.txt";
        let result = calculate_calibration_values(filename);

        assert_eq!(result.unwrap(), 31);
    }
}
