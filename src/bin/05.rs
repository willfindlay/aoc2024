use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    ops::DerefMut,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|s| s.split_once('|').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for pair in pairs {
        let entry = rules.entry(pair.1).or_default();
        entry.insert(pair.0);
    }

    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut valid = Vec::new();
    for update in updates {
        let mut is_valid = true;
        for i in 0..update.len() {
            let curr = update[i];
            for j in 0..i {
                let prev = update[j];
                if let Some(not_allowed) = rules.get(&prev) {
                    if not_allowed.contains(&curr) {
                        is_valid = false;
                        break;
                    }
                }
            }
            if !is_valid {
                break;
            }
        }
        if is_valid {
            valid.push(update);
        }
    }

    Some(valid.iter().map(|u| u[u.len() / 2]).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let pairs = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|s| s.split_once('|').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for pair in pairs {
        let entry = rules.entry(pair.1).or_default();
        entry.insert(pair.0);
    }

    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut invalid = Vec::new();
    for update in updates {
        let mut is_valid = true;
        for i in 0..update.len() {
            let curr = update[i];
            for j in 0..i {
                let prev = update[j];
                if let Some(not_allowed) = rules.get(&prev) {
                    if not_allowed.contains(&curr) {
                        is_valid = false;
                        break;
                    }
                }
            }
            if !is_valid {
                break;
            }
        }
        if !is_valid {
            invalid.push(update);
        }
    }

    for update in invalid.deref_mut() {
        update.sort_by(|a, b| {
            if let Some(allowed) = rules.get(&a) {
                if allowed.contains(b) {
                    return Ordering::Less;
                }
            }
            Ordering::Greater
        });
    }

    Some(invalid.iter().map(|u| u[u.len() / 2]).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
