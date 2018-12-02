use std::io::Error;
use std::fs;

pub fn read_lines<'a>(filename: &str) -> Result<Vec<String>, Error> {
    let lines = fs::read_to_string(filename)?;
    return Ok(lines.lines().map(|x| x.to_string()).collect());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_lines() {
        let res = read_lines("resources/test-helpers.txt");
        assert!(res.is_ok());

        let lines = res.expect("Failed to load file");

        assert_eq!(vec!["+1", "-2", "+3", "+1"], lines)
    }

    #[test]
    fn should_error_on_missing_file() {
        let res = read_lines("resources/not-a-file.nope");
        assert!(res.is_err());
    }
}
