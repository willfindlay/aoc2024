use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for line in input.lines() {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        right.push(nums.next().unwrap().parse().unwrap());
    }
    left.sort();
    right.sort();
    Some(
        std::iter::zip(left, right)
            .map(|(l, r)| (l - r).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut left: Vec<i32> = vec![];
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        let n = nums.next().unwrap().parse::<i32>().unwrap();
        let count = counts.entry(n).or_default();
        *count += 1;
    }
    Some(
        left.iter()
            .map(|n| n * counts.get(n).cloned().unwrap_or_default())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
