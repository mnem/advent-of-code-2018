extern crate regex;

use std::fs;
use regex::Regex;

fn main() {
    let input = read_input_lines();
    let result = process_lines(input);
    println!("Result: {}\n", result);
}

fn read_input_lines() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}

fn process_lines(input: String) -> i32 {
    let claims = claims_from(input);

    let mut overlap_squares = 0;
    for y in 0..1_000 {
        for x in 0..1_000 {
            let mut overlap_count = 0;
            for claim in &claims {
                if claim.contains_point(x, y) {
                    overlap_count += 1;
                    if overlap_count >= 2 {
                        overlap_squares += 1;
                        break;
                    }
                }
            }
        }
    }

    return overlap_squares;
}

fn claims_from(lines: String) -> Vec<Claim> {
    let mut claims = Vec::new();
    for line in lines.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        claims.push(Claim::from(line));
    }

    return claims;
}

#[derive(Debug)]
struct Claim {
    claim_id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    right: i32,
    bottom: i32,
}

impl Claim {
    fn from(string: &str) -> Claim {
        let re = Regex::new(r"#(?P<id>.\d*) @ (?P<x>\d*),(?P<y>\d*): (?P<width>\d*)x(?P<height>\d*)").unwrap();
        let captures = re.captures(string).unwrap();

        let claim_id = captures["id"].parse().unwrap();
        let x = captures["x"].parse().unwrap();
        let y = captures["y"].parse().unwrap();
        let width = captures["width"].parse().unwrap();
        let height = captures["height"].parse().unwrap();


        Claim {
            claim_id,
            x,
            y,
            width,
            height,
            right: x + width,
            bottom: y + height,
        }
    }
}

impl Claim {
    fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.right && y >= self.y && y < self.bottom
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_claim_from_string() {
        let input = String::from("#123 @ 4,56: 789x101112");
        let result = Claim::from(&input);

        assert_eq!(123, result.claim_id);
        assert_eq!(4, result.x);
        assert_eq!(56, result.y);
        assert_eq!(789, result.width);
        assert_eq!(101_112, result.height);
        assert_eq!(793, result.right);
        assert_eq!(101_168, result.bottom);
    }

    #[test]
    fn test_parsing_claim_from_string2() {
        let input = String::from("#1 @ 1,3: 4x4");
        let result = Claim::from(&input);

        assert_eq!(1, result.claim_id);
        assert_eq!(1, result.x);
        assert_eq!(3, result.y);
        assert_eq!(4, result.width);
        assert_eq!(4, result.height);
        assert_eq!(5, result.right);
        assert_eq!(7, result.bottom);
    }

    #[test]
    fn test_parsing() {
        let input = String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n");
        let result = claims_from(input);

        assert_eq!(3, result.len());

        assert_eq!(1, result[0].claim_id);
        assert_eq!(1, result[0].x);
        assert_eq!(3, result[0].y);
        assert_eq!(4, result[0].width);
        assert_eq!(4, result[0].height);
        assert_eq!(5, result[0].right);
        assert_eq!(7, result[0].bottom);

        assert_eq!(2, result[1].claim_id);
        assert_eq!(3, result[1].x);
        assert_eq!(1, result[1].y);
        assert_eq!(4, result[1].width);
        assert_eq!(4, result[1].height);
        assert_eq!(7, result[1].right);
        assert_eq!(5, result[1].bottom);

        assert_eq!(3, result[2].claim_id);
        assert_eq!(5, result[2].x);
        assert_eq!(5, result[2].y);
        assert_eq!(2, result[2].width);
        assert_eq!(2, result[2].height);
        assert_eq!(7, result[2].right);
        assert_eq!(7, result[2].bottom);
    }

    #[test]
    fn test_contains_point() {
        let input = String::from("#1 @ 1,1: 2x2");
        let claim = Claim::from(&input);

        assert_eq!(false, claim.contains_point(0, 0));
        assert_eq!(false, claim.contains_point(1, 0));
        assert_eq!(false, claim.contains_point(2, 0));
        assert_eq!(false, claim.contains_point(3, 0));

        assert_eq!(false, claim.contains_point(0, 1));
        assert_eq!(true,  claim.contains_point(1, 1));
        assert_eq!(true,  claim.contains_point(2, 1));
        assert_eq!(false, claim.contains_point(3, 1));

        assert_eq!(false, claim.contains_point(0, 2));
        assert_eq!(true,  claim.contains_point(1, 2));
        assert_eq!(true,  claim.contains_point(2, 2));
        assert_eq!(false, claim.contains_point(3, 2));

        assert_eq!(false, claim.contains_point(0, 3));
        assert_eq!(false, claim.contains_point(1, 3));
        assert_eq!(false, claim.contains_point(2, 3));
        assert_eq!(false, claim.contains_point(3, 3));
    }

    #[test]
    fn test_example() {
        let input = String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n");
        let result = process_lines(input);

        assert_eq!(4, result);
    }
}
