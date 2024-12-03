advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum: u32 = 0;
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            sum += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mul_re = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"^do\(\)").unwrap();
    let dont_re = Regex::new(r"^don't\(\)").unwrap();
    let mut sum: u32 = 0;
    let mut enabled = true;
    for line in input.lines() {
        for i in 0..line.len() {
            if enabled {
                if dont_re.is_match(&line[i..]) {
                    enabled = false;
                    continue;
                }
                if let Some(cap) = mul_re.captures(&line[i..]) {
                    sum += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
                }
            } else {
                if do_re.is_match(&line[i..]) {
                    enabled = true;
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
