advent_of_code::solution!(4);

fn wordsearch(search: &Vec<Vec<char>>, x: isize, y: isize, word: &str) -> u32 {
    let n = search.len() as isize;
    let m = search[0].len() as isize;

    let word = word.chars().collect::<Vec<_>>();

    let dir_x = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dir_y = [-1, 0, 1, -1, 1, -1, 0, 1];

    if search[x as usize][y as usize] != word[0] {
        return 0;
    }

    let mut count = 0;
    for d in 0..8 {
        let mut curr_x = x + dir_x[d];
        let mut curr_y = y + dir_y[d];
        let mut k = 1;

        while k < word.len() {
            if curr_x >= n || curr_x < 0 || curr_y >= m || curr_y < 0 {
                break;
            }

            if search[curr_x as usize][curr_y as usize] != word[k] {
                break;
            }

            curr_x += dir_x[d];
            curr_y += dir_y[d];
            k += 1;

            if k == word.len() {
                count += 1;
            }
        }
    }
    return count;
}

fn is_x_mas(search: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let n = search.len();
    let m = search[0].len();

    if search[x][y] != 'A' {
        return false;
    }

    if x < 1 || x >= n - 1 || y < 1 || y >= m - 1 {
        return false;
    }

    let target = ['M', 'A', 'S'];
    let target_rev = ['S', 'A', 'M'];
    let word1 = [search[x - 1][y - 1], search[x][y], search[x + 1][y + 1]];
    let word2 = [search[x - 1][y + 1], search[x][y], search[x + 1][y - 1]];
    let mut word1_match = false;
    let mut word2_match = false;

    for i in 0..3 {
        if word1[i] != target[i] {
            break;
        }
        if i == 2 {
            word1_match = true;
        }
    }

    for i in 0..3 {
        if word1[i] != target_rev[i] {
            break;
        }
        if i == 2 {
            word1_match = true;
        }
    }

    for i in 0..3 {
        if word2[i] != target[i] {
            break;
        }
        if i == 2 {
            word2_match = true;
        }
    }

    for i in 0..3 {
        if word2[i] != target_rev[i] {
            break;
        }
        if i == 2 {
            word2_match = true;
        }
    }

    return word1_match && word2_match;
}

pub fn part_one(input: &str) -> Option<u32> {
    let search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for x in 0..search.len() {
        for y in 0..search[0].len() {
            count += wordsearch(&search, x as isize, y as isize, "XMAS".into());
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let search = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for x in 0..search.len() {
        for y in 0..search[0].len() {
            count += is_x_mas(&search, x, y).then_some(1).unwrap_or(0);
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
