advent_of_code::solution!(2);

fn is_safe(report: &[u32]) -> bool {
    let is_increasing = report.windows(2).all(|w| w[0] < w[1]);
    let is_decreasing = report.windows(2).all(|w| w[0] > w[1]);
    let differ_ok = report.windows(2).all(|w| {
        let diff = w[0].abs_diff(w[1]);
        (1..=3).contains(&diff)
    });
    (is_decreasing || is_increasing) && differ_ok
}

fn is_safe_part2(report: &[u32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for n in 0..report.len() {
        let removed = report
            .iter()
            .enumerate()
            .filter_map(|(i, el)| (i != n).then_some(*el))
            .collect::<Vec<_>>();
        if is_safe(&removed) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut reports = Vec::new();
    for line in input.lines() {
        reports.push(
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    let mut num_safe = 0;
    for report in reports {
        if is_safe(&report) {
            num_safe += 1;
        }
    }
    Some(num_safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut reports = Vec::new();
    for line in input.lines() {
        reports.push(
            line.split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }
    let mut num_safe = 0;
    for report in reports {
        if is_safe_part2(&report) {
            num_safe += 1;
        }
    }
    Some(num_safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
