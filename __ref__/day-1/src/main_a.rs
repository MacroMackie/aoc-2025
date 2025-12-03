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

    let (mut list_a, mut list_b): (Vec<i32>, Vec<i32>) = lines
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

    list_a.sort();
    list_b.sort();

    let total_distance: i32 = list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Sorted list_a: {:?}", list_a);
    println!("Sorted list_b: {:?}", list_b);
    println!("Total distance: {}", total_distance);

    Ok(total_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-1/input/test-a.txt";
        let result = calculate_calibration_values(filename);

        assert_eq!(result.unwrap(), 11);
    }
}
