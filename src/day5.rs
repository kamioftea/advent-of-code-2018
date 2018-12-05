use std::collections::HashSet;
use std::iter::FromIterator;

pub fn collapse_polymer(polymer: &String) -> String {
    polymer.chars().fold(
        "".to_string(),
        |acc, c| {
            let (rest, p) = if acc == "" { ("", "") } else { acc.split_at(acc.len() - 1) };
            if c.to_string() != p && c.to_string().to_lowercase() == p.to_lowercase() {
                rest.to_string()
            } else {
                format!("{}{}", acc, c).trim().to_string()
            }
        },
    )
}

pub fn remove_best_unit_and_collapse(polymer: &String) -> String {
    let chars: HashSet<char> = HashSet::from_iter(polymer.to_lowercase().chars());

    chars.iter()
        .map(|to_remove| {
            let removed: String = polymer.chars().filter(|c| c.to_lowercase().to_string() != to_remove.to_string()).collect();
            collapse_polymer(&removed)
        })
        .min_by(|a, b| a.len().cmp(&b.len()))
        .unwrap_or("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_collapse_polymer() {
        assert_eq!("", collapse_polymer(&"aA".to_string()));
        assert_eq!("", collapse_polymer(&"abBA".to_string()));
        assert_eq!("abAB", collapse_polymer(&"abAB".to_string()));
        assert_eq!("aabAAB", collapse_polymer(&"aabAAB".to_string()));
        assert_eq!("aabAAB", collapse_polymer(&"aabCcAAB".to_string()));
        assert_eq!("dabCBAcaDA", collapse_polymer(&"dabAcCaCBAcCcaDA".to_string()));
    }

    #[test]
    fn should_remove_best_unit() {
        assert_eq!("", remove_best_unit_and_collapse(&"aA".to_string()));
        assert_eq!("", remove_best_unit_and_collapse(&"abBA".to_string()));
        assert_eq!("", remove_best_unit_and_collapse(&"abAB".to_string()));
        assert_eq!("", remove_best_unit_and_collapse(&"aabAAB".to_string()));
        assert_eq!("", remove_best_unit_and_collapse(&"aabCcAAB".to_string()));
        assert_eq!("daDA", remove_best_unit_and_collapse(&"dabAcCaCBAcCcaDA".to_string()));
    }
}
