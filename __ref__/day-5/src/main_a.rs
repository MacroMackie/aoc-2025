// Start of Selection
use std::collections::HashMap;

use utils::{load_env, load_file};

fn main() {
    load_env();

    let filename = "./day-5/input/input.txt";
    let result = run(filename).unwrap();

    println!("result: {}", result);
}

pub fn run(filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = load_file(filename)?;
    let joined = content.join("\n");
    let sections: Vec<&str> = joined.split("\n\n").collect();

    println!("sections: {:?}", sections);

    if sections.len() < 2 {
        return Err("Input file must contain rules and updates separated by a blank line".into());
    }

    // Parse rules
    let rules = parse_rules(sections[0]);

    // Parse updates
    let updates = parse_updates(sections[1]);

    // Calculate the sum of middle pages from correctly ordered updates
    let middle_sum = calculate_middle_sum(&rules, &updates);

    Ok(middle_sum)
}

fn parse_rules(rules_section: &str) -> Vec<(i32, i32)> {
    rules_section
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.trim().split('|').collect();
            if parts.len() == 2 {
                if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    return Some((x, y));
                }
            }
            None
        })
        .collect()
}

fn parse_updates(updates_section: &str) -> Vec<Vec<i32>> {
    updates_section
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() {
                return None;
            }
            let pages = line
                .split(',')
                .map(|s| s.trim().parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
                .ok()?;
            Some(pages)
        })
        .collect()
}

fn calculate_middle_sum(rules: &[(i32, i32)], updates: &[Vec<i32>]) -> i32 {
    let mut sum = 0;

    for update in updates {
        if is_correctly_ordered(rules, update) {
            if let Some(middle) = get_middle_page(update) {
                sum += middle;
            }
        }
    }

    sum
}

fn is_correctly_ordered(rules: &[(i32, i32)], update: &[i32]) -> bool {
    let page_positions: HashMap<i32, usize> =
        update.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (page_positions.get(&x), page_positions.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }

    true
}

fn get_middle_page(update: &[i32]) -> Option<i32> {
    if update.is_empty() {
        return None;
    }
    let middle_index = update.len() / 2;
    Some(update[middle_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_calculates_middle_sum() {
        let rules_section = "\
            47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13";

        let updates_section = "\
            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            ";

        let rules = parse_rules(rules_section);
        let updates = parse_updates(updates_section);
        let result = calculate_middle_sum(&rules, &updates);

        assert_eq!(result, 143);
    }

    #[test]
    fn it_handles_empty_updates() {
        let rules_section = "1|2";
        let updates_section = "";

        let rules = parse_rules(rules_section);
        let updates = parse_updates(updates_section);
        let result = calculate_middle_sum(&rules, &updates);

        assert_eq!(result, 0);
    }

    #[test]
    fn it_handles_no_correct_updates() {
        let rules_section = "\
            1|2
            2|3";

        let updates_section = "\
            2,1,3
            3,2,1
            ";

        let rules = parse_rules(rules_section);
        let updates = parse_updates(updates_section);
        let result = calculate_middle_sum(&rules, &updates);

        assert_eq!(result, 0);
    }

    #[test]
    fn it_handles_all_correct_updates() {
        let rules_section = "\
            1|2
            2|3
            1|3";

        let updates_section = "\
            1,2,3
            1,3
            2,3
            ";

        let rules = parse_rules(rules_section);
        let updates = parse_updates(updates_section);
        let result = calculate_middle_sum(&rules, &updates);

        // Middle pages: 2, 3, 3 => sum = 8
        assert_eq!(result, 8);
    }
}
