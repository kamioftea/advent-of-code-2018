use std::collections::HashMap;
use itertools::Itertools;

fn contains_duplicates(s: &str, n: i32) -> bool {
    let mut chars: HashMap<char, i32> = HashMap::new();

    s.chars().for_each(
        |c| *chars.entry(c).or_insert(0) += 1
    );

    chars.values()
        .find(|v| **v == n)
        .is_some()
}

fn count_with_duplicates(strings: &Vec<String>, n: i32) -> usize {
    strings.iter().filter(|s| contains_duplicates(s, n)).count()
}

pub fn check_sum(strings: &Vec<String>) -> usize {
    count_with_duplicates(strings, 2) * count_with_duplicates(strings, 3)
}

fn compare_strings(a: &str, b: &str) -> String {
    let a_s: Vec<char> = a.chars().collect();
    let b_s: Vec<char> = b.chars().collect();

    let mut out: String = "".to_string();

    a_s.iter().enumerate().for_each(
        |(i, c)| {
            if b_s[i] == *c { out.push(*c) }
        }
    );

    out.to_string()
}

pub fn find_matching(strings: &Vec<String>) -> Option<String> {
    for (a, b) in strings.iter().tuple_combinations() {
        let intersect = compare_strings(a, b);
        if intersect.len() == a.len() - 1 {
            return Some(intersect)
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_check_for_duplicates() {
        assert_eq!(contains_duplicates("abcdef", 2), false);
        assert_eq!(contains_duplicates("abcdef", 3), false);

        assert_eq!(contains_duplicates("bababc", 2), true);
        assert_eq!(contains_duplicates("bababc", 3), true);

        assert_eq!(contains_duplicates("abbcde", 2), true);
        assert_eq!(contains_duplicates("abbcde", 3), false);

        assert_eq!(contains_duplicates("abcccd", 2), false);
        assert_eq!(contains_duplicates("abcccd", 3), true);

        assert_eq!(contains_duplicates("aabcdd", 2), true);
        assert_eq!(contains_duplicates("aabcdd", 3), false);

        assert_eq!(contains_duplicates("abcdee", 2), true);
        assert_eq!(contains_duplicates("abcdee", 3), false);

        assert_eq!(contains_duplicates("ababab", 2), false);
        assert_eq!(contains_duplicates("ababab", 3), true);
    }

    #[test]
    fn should_get_correct_checksum() {
        let strings = vec!["abcdef",
                           "bababc",
                           "abbcde",
                           "abcccd",
                           "aabcdd",
                           "abcdee",
                           "ababab"].iter().map(|x| x.to_string()).collect();

        assert_eq!(count_with_duplicates(&strings, 2), 4);
        assert_eq!(count_with_duplicates(&strings, 3), 3);
        assert_eq!(check_sum(&strings), 12);
    }

    #[test]
    fn can_compare_strings() {
        assert_eq!("", compare_strings("abcde", "fghij"));
        assert_eq!("fgij", compare_strings("fguij", "fghij"));
        assert_eq!("ace", compare_strings("abcde", "axcye"));
        assert_eq!("abcde", compare_strings("abcde", "abcde"));
        assert_eq!("a", compare_strings("accbb", "abbcc"));
    }

    #[test]
    fn finds_matching_string() {
        let strings = vec!["abcde",
                           "fghij",
                           "klmno",
                           "pqrst",
                           "fguij",
                           "axcye",
                           "wvxyz"].iter().map(|x| x.to_string()).collect();

        assert_eq!(Some("fgij".to_string()), find_matching(&strings))
    }
}
