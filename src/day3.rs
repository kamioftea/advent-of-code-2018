use std::collections::BTreeSet;

use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct Claim {
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

pub fn parse_claims(claims: &Vec<String>) -> Vec<Claim> {
    claims
        .iter()
        .map(|c| parse_claim(c.as_str()))
        .flatten()
        .collect()
}

fn get_conflicted(claims: &Vec<Claim>) -> BTreeSet<i32> {
    let mut covered = BTreeSet::new();
    let mut contested = BTreeSet::new();

    for claim in claims.iter() {
        for x in claim.x..(claim.x + claim.w) {
            for y in claim.y..(claim.y + claim.h) {
                let pos = x + (y * 1000);

                if !covered.insert(pos) {
                    contested.insert(pos);
                }
            }
        }
    }

    contested
}

pub fn get_conflicted_area(claims: &Vec<Claim>) -> usize {
    get_conflicted(claims).len()
}

pub fn get_unique_claim_id(claims: &Vec<Claim>) -> Option<i32> {
    let conflicted = get_conflicted(claims);

    'claim_loop: for claim in claims.iter() {
        for x in claim.x..(claim.x + claim.w) {
            for y in claim.y..(claim.y + claim.h) {
                let pos = x + (y * 1000);

                if conflicted.contains(&pos) {
                    continue 'claim_loop;
                }
            }
        }

        return Some(claim.id)
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_claim() {
        assert_eq!(
            Some(Claim { id: 123, x: 3, y: 2, w: 5, h: 4 }),
            parse_claim("#123 @ 3,2: 5x4")
        );

        assert_eq!(
            Some(Claim { id: 1, x: 1, y: 3, w: 4, h: 4 }),
            parse_claim("#1 @ 1,3: 4x4")
        );

        assert_eq!(None, parse_claim(""))
    }

    #[test]
    fn can_find_conflict() {
        assert_eq!(
            4,
            get_conflicted_area(
                &vec![
                    Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
                    Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
                    Claim { id: 3, x: 5, y: 5, w: 2, h: 2 },
                ]
            )
        );

        assert_eq!(
            4,
            get_conflicted_area(
                &vec![
                    Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
                    Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
                ]
            )
        );

        assert_eq!(
            8,
            get_conflicted_area(
                &vec![
                    Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
                    Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
                    Claim { id: 3, x: 4, y: 4, w: 3, h: 3 },
                ]
            )
        )
    }

    #[test]
    fn can_find_intersect() {
        assert_eq!(
            Some(3),
            get_unique_claim_id(
                &vec![
                    Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
                    Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
                    Claim { id: 3, x: 5, y: 5, w: 2, h: 2 },
                ]
            )
        );

        assert_eq!(
            None,
            get_unique_claim_id(
                &vec![
                    Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
                    Claim { id: 2, x: 3, y: 1, w: 4, h: 4 },
                ]
            )
        );

        assert_eq!(
            Some(2),
            get_unique_claim_id(
                &vec![
                    Claim { id: 1, x: 1, y: 3, w: 4, h: 4 },
                    Claim { id: 2, x: 3, y: 1, w: 4, h: 2 },
                    Claim { id: 3, x: 4, y: 4, w: 3, h: 3 },
                ]
            )
        )
    }
}
