use std::collections::HashSet;

fn parse_lines(lines: &Vec<String>) -> Vec<i32> {
    lines
        .iter()
        .map(|x| x.parse::<i32>())
        .flatten()
        .collect()
}

fn sum_lines(lines: Vec<i32>) -> i32 {
    lines.iter().sum()
}

fn find_first_repeat(lines: Vec<i32>) -> i32 {
    let mut pos = 0;
    let mut seen: HashSet<i32> = HashSet::new();
    let mut running_total = 0;

    seen.insert(0);

    loop {
        if pos == lines.len() {
            pos = 0;
        }

        running_total += lines[pos];

        if seen.contains(&running_total) {
            return running_total;
        }

        seen.insert(running_total);
        pos = pos + 1;
    }
}

pub fn part_1(lines: &Vec<String>) -> i32 {
    sum_lines(parse_lines(lines))
}

pub fn part_2(lines: &Vec<String>) -> i32 {
    find_first_repeat(parse_lines(lines))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_lines() {
        assert_eq!(sum_lines(vec![1, -2, 3, 1]), 3);
        assert_eq!(sum_lines(vec![1, 1, 1]), 3);
        assert_eq!(sum_lines(vec![1, 1, -2]), 0);
        assert_eq!(sum_lines(vec![-1, -2, -3]), -6);
    }

    #[test]
    fn should_find_first_repeat() {
        assert_eq!(find_first_repeat(vec![1, -2, 3, 1]), 2);
        assert_eq!(find_first_repeat(vec![1, -1]), 0);
        assert_eq!(find_first_repeat(vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(find_first_repeat(vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(find_first_repeat(vec![7, 7, -2, -7, -4]), 14);
    }
}
