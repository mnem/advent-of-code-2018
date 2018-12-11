use std::fs;
use regex::Regex;
use std::cmp;
use image::{Luma, GrayImage};

fn main() {
    let input = read_input();
    let result = process(&input);
    println!("Result:\n{}\n", result);
}

fn read_input() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}

fn process(input: &str) -> i32 {
    let mut starfield = starfield_from(input);
    let mut last_area = i64::max_value();
    let mut second = 0;

    loop {
        starfield_step(&mut starfield, StepDirection::Forwards);
        let (top_left, bottom_right) = starfield_extent(&starfield);
        let area = (bottom_right.x - top_left.x) * (bottom_right.y - top_left.y);
        if area > last_area {
            break;
        }
        last_area = area;
        second += 1;
    }
    starfield_step(&mut starfield, StepDirection::Backwards);
    starfield_to_bmp(&starfield, "message");

    return second;
}

fn starfield_from(string: &str) -> Vec<Star> {
    let mut starfield = Vec::new();

    for line in string.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let star = Star::from(line);
        starfield.push(star);
    }

    return starfield;
}

fn starfield_extent(starfield: &Vec<Star>) -> (Point, Point) {
    let mut top_left = Point { x: i64::max_value(), y: i64::max_value() };
    let mut bottom_right = Point { x: i64::min_value(), y: i64::min_value() };

    for star in starfield {
        top_left.x = cmp::min(top_left.x, star.position.x);
        top_left.y = cmp::min(top_left.y, star.position.y);

        bottom_right.x = cmp::max(bottom_right.x, star.position.x);
        bottom_right.y = cmp::max(bottom_right.y, star.position.y);
    }

    (top_left, bottom_right)
}

#[cfg(test)]
fn starfield_to_string(starfield: &Vec<Star>) -> String {
    let mut output = String::new();
    let (top_left, bottom_right) = starfield_extent(starfield);

    for y in top_left.y..=bottom_right.y {
        for x in top_left.x..=bottom_right.x {
            let plot = Point { x, y };
            let has_star = starfield.iter().any( |star| { star.position == plot } );
            if has_star {
                output += "*";
            } else {
                output += " ";
            }
        }
        output += "\n";
    }

    return output;
}

fn starfield_to_bmp(starfield: &Vec<Star>, name: &str) {
    let (top_left, bottom_right) = starfield_extent(starfield);

    let starfield_width = bottom_right.x - top_left.x;
    let starfield_height = bottom_right.y - top_left.y;

    let pixel = Luma([255]);
    let mut img = GrayImage::new((starfield_width + 1) as u32, (starfield_height + 1) as u32);
    for star in starfield {
        let star_pos_x = star.position.x + -top_left.x;
        let star_pos_y = star.position.y + -top_left.y;

        img.put_pixel(star_pos_x as u32, star_pos_y as u32,  pixel);
    }

    let name = format!("{}.png", name);
    let _ = img.save(name).unwrap();
}

fn starfield_step(starfield: &mut Vec<Star>, direction: StepDirection) {
    let sign_change = match direction {
        StepDirection::Forwards => 1,
        StepDirection::Backwards => -1,
    };

    for star in starfield {
        star.position.x += star.velocity.dx * sign_change;
        star.position.y += star.velocity.dy * sign_change;
    }
}

enum StepDirection {
    Forwards,
    Backwards,
}

#[derive(PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

struct Velocity {
    dx: i64,
    dy: i64,
}

struct Star {
    position: Point,
    velocity: Velocity,
}

impl Star {
    fn from(string: &str) -> Star {
        let re = Regex::new(r"n=<\s*(?P<x>.*?),\s*(?P<y>.*?)>.*y=<\s*(?P<dx>.*?),\s*(?P<dy>.*?)>").expect("Malformed regex");
        let captures = re.captures(string).expect("Malformed game setup string.");

        let x = captures["x"].parse().expect("Malformed x.");
        let y = captures["y"].parse().expect("Malformed y.");
        let position = Point { x, y };

        let dx = captures["dx"].parse().expect("Malformed dx.");
        let dy = captures["dy"].parse().expect("Malformed dy.");
        let velocity = Velocity { dx, dy };

        Star { position, velocity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_from_string() {
        let input = "position=< 9,  1> velocity=< 0,  2>";
        let result = Star::from(input);
        assert_eq!(9, result.position.x);
        assert_eq!(1, result.position.y);
        assert_eq!(0, result.velocity.dx);
        assert_eq!(2, result.velocity.dy);
    }

    #[test]
    fn test_starfield_from_string() {
        let input = "position=< 9,  1> velocity=< 0,  2>\nposition=< 7,  0> velocity=<-1,  0>\nposition=< 3, -2> velocity=<-1,  1>\nposition=< 6, 10> velocity=<-2, -1>\nposition=< 2, -4> velocity=< 2,  2>\nposition=<-6, 10> velocity=< 2, -2>\nposition=< 1,  8> velocity=< 1, -1>\nposition=< 1,  7> velocity=< 1,  0>\nposition=<-3, 11> velocity=< 1, -2>\nposition=< 7,  6> velocity=<-1, -1>\nposition=<-2,  3> velocity=< 1,  0>\nposition=<-4,  3> velocity=< 2,  0>\nposition=<10, -3> velocity=<-1,  1>\nposition=< 5, 11> velocity=< 1, -2>\nposition=< 4,  7> velocity=< 0, -1>\nposition=< 8, -2> velocity=< 0,  1>\nposition=<15,  0> velocity=<-2,  0>\nposition=< 1,  6> velocity=< 1,  0>\nposition=< 8,  9> velocity=< 0, -1>\nposition=< 3,  3> velocity=<-1,  1>\nposition=< 0,  5> velocity=< 0, -1>\nposition=<-2,  2> velocity=< 2,  0>\nposition=< 5, -2> velocity=< 1,  2>\nposition=< 1,  4> velocity=< 2,  1>\nposition=<-2,  7> velocity=< 2, -2>\nposition=< 3,  6> velocity=<-1, -1>\nposition=< 5,  0> velocity=< 1,  0>\nposition=<-6,  0> velocity=< 2,  0>\nposition=< 5,  9> velocity=< 1, -2>\nposition=<14,  7> velocity=<-2,  0>\nposition=<-3,  6> velocity=< 2, -1>\n";
        let result = starfield_from(input);
        assert_eq!(31, result.len());
    }

    #[test]
    fn test_starfield_extent() {
        let input = "position=< 9,  1> velocity=< 0,  2>\nposition=< 7,  0> velocity=<-1,  0>\nposition=< 3, -2> velocity=<-1,  1>\nposition=< 6, 10> velocity=<-2, -1>\nposition=< 2, -4> velocity=< 2,  2>\nposition=<-6, 10> velocity=< 2, -2>\nposition=< 1,  8> velocity=< 1, -1>\nposition=< 1,  7> velocity=< 1,  0>\nposition=<-3, 11> velocity=< 1, -2>\nposition=< 7,  6> velocity=<-1, -1>\nposition=<-2,  3> velocity=< 1,  0>\nposition=<-4,  3> velocity=< 2,  0>\nposition=<10, -3> velocity=<-1,  1>\nposition=< 5, 11> velocity=< 1, -2>\nposition=< 4,  7> velocity=< 0, -1>\nposition=< 8, -2> velocity=< 0,  1>\nposition=<15,  0> velocity=<-2,  0>\nposition=< 1,  6> velocity=< 1,  0>\nposition=< 8,  9> velocity=< 0, -1>\nposition=< 3,  3> velocity=<-1,  1>\nposition=< 0,  5> velocity=< 0, -1>\nposition=<-2,  2> velocity=< 2,  0>\nposition=< 5, -2> velocity=< 1,  2>\nposition=< 1,  4> velocity=< 2,  1>\nposition=<-2,  7> velocity=< 2, -2>\nposition=< 3,  6> velocity=<-1, -1>\nposition=< 5,  0> velocity=< 1,  0>\nposition=<-6,  0> velocity=< 2,  0>\nposition=< 5,  9> velocity=< 1, -2>\nposition=<14,  7> velocity=<-2,  0>\nposition=<-3,  6> velocity=< 2, -1>\n";
        let starfield = starfield_from(input);
        let (top_left, bottom_right) = starfield_extent(&starfield);
        assert_eq!(-6, top_left.x);
        assert_eq!(-4, top_left.y);
        assert_eq!(15, bottom_right.x);
        assert_eq!(11, bottom_right.y);
    }

    #[test]
    fn test_starfield_step() {
        let input = "position=< 9,  1> velocity=< 0,  2>\nposition=< 7,  0> velocity=<-1,  0>\nposition=< 3, -2> velocity=<-1,  1>\nposition=< 6, 10> velocity=<-2, -1>\nposition=< 2, -4> velocity=< 2,  2>\nposition=<-6, 10> velocity=< 2, -2>\nposition=< 1,  8> velocity=< 1, -1>\nposition=< 1,  7> velocity=< 1,  0>\nposition=<-3, 11> velocity=< 1, -2>\nposition=< 7,  6> velocity=<-1, -1>\nposition=<-2,  3> velocity=< 1,  0>\nposition=<-4,  3> velocity=< 2,  0>\nposition=<10, -3> velocity=<-1,  1>\nposition=< 5, 11> velocity=< 1, -2>\nposition=< 4,  7> velocity=< 0, -1>\nposition=< 8, -2> velocity=< 0,  1>\nposition=<15,  0> velocity=<-2,  0>\nposition=< 1,  6> velocity=< 1,  0>\nposition=< 8,  9> velocity=< 0, -1>\nposition=< 3,  3> velocity=<-1,  1>\nposition=< 0,  5> velocity=< 0, -1>\nposition=<-2,  2> velocity=< 2,  0>\nposition=< 5, -2> velocity=< 1,  2>\nposition=< 1,  4> velocity=< 2,  1>\nposition=<-2,  7> velocity=< 2, -2>\nposition=< 3,  6> velocity=<-1, -1>\nposition=< 5,  0> velocity=< 1,  0>\nposition=<-6,  0> velocity=< 2,  0>\nposition=< 5,  9> velocity=< 1, -2>\nposition=<14,  7> velocity=<-2,  0>\nposition=<-3,  6> velocity=< 2, -1>\n";
        let mut starfield = starfield_from(input);
        starfield_step(&mut starfield, StepDirection::Forwards);
        let result = starfield_to_string(&starfield);

        let expected = "        *    *    \n      *     *     \n*         *      *\n                  \n    *             \n  **         *    \n    * *           \n   ** **  *       \n      * *         \n      *   *     * \n*           *     \n  *     * *       \n";
        assert_eq!(expected, result);
    }

    #[test]
    fn test_example() {
        let input = "position=< 9,  1> velocity=< 0,  2>\nposition=< 7,  0> velocity=<-1,  0>\nposition=< 3, -2> velocity=<-1,  1>\nposition=< 6, 10> velocity=<-2, -1>\nposition=< 2, -4> velocity=< 2,  2>\nposition=<-6, 10> velocity=< 2, -2>\nposition=< 1,  8> velocity=< 1, -1>\nposition=< 1,  7> velocity=< 1,  0>\nposition=<-3, 11> velocity=< 1, -2>\nposition=< 7,  6> velocity=<-1, -1>\nposition=<-2,  3> velocity=< 1,  0>\nposition=<-4,  3> velocity=< 2,  0>\nposition=<10, -3> velocity=<-1,  1>\nposition=< 5, 11> velocity=< 1, -2>\nposition=< 4,  7> velocity=< 0, -1>\nposition=< 8, -2> velocity=< 0,  1>\nposition=<15,  0> velocity=<-2,  0>\nposition=< 1,  6> velocity=< 1,  0>\nposition=< 8,  9> velocity=< 0, -1>\nposition=< 3,  3> velocity=<-1,  1>\nposition=< 0,  5> velocity=< 0, -1>\nposition=<-2,  2> velocity=< 2,  0>\nposition=< 5, -2> velocity=< 1,  2>\nposition=< 1,  4> velocity=< 2,  1>\nposition=<-2,  7> velocity=< 2, -2>\nposition=< 3,  6> velocity=<-1, -1>\nposition=< 5,  0> velocity=< 1,  0>\nposition=<-6,  0> velocity=< 2,  0>\nposition=< 5,  9> velocity=< 1, -2>\nposition=<14,  7> velocity=<-2,  0>\nposition=<-3,  6> velocity=< 2, -1>\n";
        let mut starfield = starfield_from(input);
        starfield_step(&mut starfield, StepDirection::Forwards);
        starfield_step(&mut starfield, StepDirection::Forwards);
        starfield_step(&mut starfield, StepDirection::Forwards);
        let result = starfield_to_string(&starfield);

        let expected = "*   *  ***\n*   *   * \n*   *   * \n*****   * \n*   *   * \n*   *   * \n*   *   * \n*   *  ***\n";
        assert_eq!(expected, result);
    }
}
