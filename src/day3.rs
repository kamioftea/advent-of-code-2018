use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn parse_claim(claim: &str) -> Option<Claim> {
    lazy_static! {
        static ref CLAIM_MATCHER: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    CLAIM_MATCHER
        .captures(claim)
        .map(|matches|
            Claim {
                id: matches.get(1).unwrap().as_str().parse().unwrap(),
                x: matches.get(2).unwrap().as_str().parse().unwrap(),
                y: matches.get(3).unwrap().as_str().parse().unwrap(),
                w: matches.get(4).unwrap().as_str().parse().unwrap(),
                h: matches.get(5).unwrap().as_str().parse().unwrap(),
            }
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_claim() {
        assert_eq!(
            Some(Claim {id: 1, x: 1, y: 3, w: 4, h: 4}),
            parse_claim("#1 @ 1,3: 4x4")
        );

        assert_eq!(None, parse_claim(""))
    }

}
