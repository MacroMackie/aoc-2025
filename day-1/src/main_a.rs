use utils::{load_env, load_file};

fn main() {
    load_env();

    let filename = "./day-1/input/input.txt";
    let result = calculate_dial_rotations(filename).unwrap();

    println!("result: {}", result);
}

#[derive(Debug, PartialEq)]
enum DialDirection {
    Left,
    Right,
}

impl DialDirection {
    fn from_str(s: &str) -> Self {
        if s == "L" {
            DialDirection::Left
        } else {
            DialDirection::Right
        }
    }
}

#[derive(Debug, PartialEq)]
struct DialRotation {
    direction: DialDirection,
    distance: i32,
}

pub fn calculate_dial_rotations(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let lines = load_file(filename)?;

    println!("lines: {:?}", lines);

    let array: Vec<DialRotation> = lines
        .iter()
        .map(|line| {
            let (direction_str, distance_str) = line.split_at(1);

            let direction = DialDirection::from_str(direction_str);
            let distance = distance_str
                .trim()
                .parse::<i32>()
                .expect("Failed to parse distance");

            DialRotation {
                direction,
                distance,
            }
        })
        .collect();

    // We start rotating the dial from 50
    // If we hit greater than 99 or less than 0, it wraps around to the other side
    // (eg modulus 100)
    let mut current_position = 50;
    // We increment this counter whenever we turn it back to 0
    let mut times_rotated_back_to_0 = 0;

    for rotation in array {
        if rotation.direction == DialDirection::Left {
            current_position -= rotation.distance;
        } else {
            current_position += rotation.distance;
        }

        if current_position > 99 {
            current_position = current_position % 100;
        }

        if current_position < 0 {
            current_position = (current_position + 100) % 100;
        }

        if current_position == 0 {
            times_rotated_back_to_0 += 1;
        }

        println!("rotation: {:?}", rotation);
        println!("current_position: {}", current_position);
    }

    println!("current_position: {}", current_position);
    println!("times_rotated_back_to_0: {}", times_rotated_back_to_0);

    Ok(times_rotated_back_to_0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        load_env();
        let filename = "./day-1/input/test-a.txt";
        let result = calculate_dial_rotations(filename);

        assert_eq!(result.unwrap(), 3);
    }
}
