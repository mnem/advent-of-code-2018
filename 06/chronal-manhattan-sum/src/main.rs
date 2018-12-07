extern crate regex;

use std::fs;
use std::cmp;
use regex::Regex;

fn main() {
    let input = read_input();
    let result = process(&input, 10_000);
    println!("Result: {}\n", result);
}

fn read_input() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}

fn process(input: &str, max_distance: i32) -> i32 {
    let points = Point::from_lines(&input);

    // Work out how large a grid we have to consider
    let (max_x, max_y) = extent(&points);

    // Score a grid for each of the points
    let mut scored_grids = Vec::new();
    for point in points {
        let scored_grid = scored_grid_from(point, max_x, max_y);
        scored_grids.push(scored_grid);
    }

    // Add the grids
    let mut total_grid = Vec::new();
    let cell_count = (max_x * max_y) as usize;
    for cell_index in 0..cell_count {
        let mut total = 0;
        for grid_index in 0..scored_grids.len() {
            let grid_cell_score = scored_grids[grid_index][cell_index];
            total += grid_cell_score;
        }
        total_grid.push(total);
    }

    let area = total_grid.iter().fold(0, |acc, value| {
        if *value < max_distance {
            return acc + 1;
        } else {
            return acc;
        }
    });

    return area;
}

fn grid_is_infinite(grid: &Vec<i32>, extent_x: usize, extent_y: usize) -> bool {
    let max = extent_x * extent_y;

    let top_range = 0..extent_x;
    let bottom_range = (extent_x * (extent_y - 1))..max;
    for (top, bottom) in top_range.zip(bottom_range) {
        if grid[top] >= 0 || grid[bottom]  >= 0 {
            return true;
        }
    }

    let left_range = (0 ..max).step_by(extent_x);
    let right_range = ((extent_x - 1)..max).step_by(extent_x);
    for (left, right) in left_range.zip(right_range) {
        if grid[left] >= 0 || grid[right]  >= 0 {
            return true;
        }
    }

    return false;
}

fn scored_grid_from(point: Point, extent_x: usize, extend_y: usize) -> Vec<i32> {
    let mut scores = Vec::new();
    for y in 0..extend_y {
        for x in 0..extent_x {
            let score = ((y as i32) - point.y).abs() + ((x as i32) - point.x).abs();
            scores.push(score);
        }
    }
    return scores;
}

fn extent(points: &Vec<Point>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    for point in points {
        max_x = cmp::max(max_x, point.x);
        max_y = cmp::max(max_y, point.y);
    }
    max_x += 1;
    max_y += 1;

    (max_x as usize, max_y as usize)
}

//fn print_scores_as_grid(scores: &Vec<i32>, extent_x: usize, extent_y: usize) {
//    println!("----[Grid]----");
//    for y in 0..extent_y {
//        for x in 0..extent_x {
//            let index = (extent_x * y + x) as usize;
//            print!("{:>4}", scores[index]);
//        }
//        println!("");
//    }
//    println!("--------------");
//}

#[derive(Debug,PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from_string(string: &str) -> Point {
        let re = Regex::new(r"(?P<x>\d*)\D*(?P<y>\d*)").unwrap();
        let captures = re.captures(string).unwrap();
        return Point { x: captures["x"].parse().unwrap(), y: captures["y"].parse().unwrap() };
    }

    fn from_lines(lines: &str) -> Vec<Point> {
        let mut points = Vec::new();
        for line in lines.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }
            points.push(Point::from_string(line));
        }
        return points;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_from_string() {
        let subject = Point::from_string("1, 2");
        assert_eq!(Point { x: 1, y: 2 }, subject);

        let subject = Point::from_string("100 , 25699");
        assert_eq!(Point { x: 100, y: 25699 }, subject);
    }

    #[test]
    fn test_point_from_lines_trailing_newline() {
        let subject = Point::from_lines("1, 2\n3, 4\n5, 6\n");
        let expected = vec![Point {x: 1, y: 2},Point {x: 3, y: 4},Point {x: 5, y: 6},];
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_point_from_lines_trailing() {
        let subject = Point::from_lines("1, 2\n3, 4\n5, 6");
        let expected = vec![Point {x: 1, y: 2},Point {x: 3, y: 4},Point {x: 5, y: 6},];
        assert_eq!(expected, subject);
    }

    #[test]
    fn test_extent() {
        let input = vec![Point {x: 0, y: 10}, Point {x: 20, y: 0}, Point {x: 19, y: 10},];
        let (max_x, max_y) = extent(&input);
        assert_eq!(21, max_x);
        assert_eq!(11, max_y);
    }

    #[test]
    fn test_scored_grid_1() {
        let in_point = Point { x: 1, y: 1};
        let extent_x = 3;
        let extent_y = 3;
        let result = scored_grid_from(in_point, extent_x, extent_y);

        let expected = vec![
            2, 1, 2,
            1, 0, 1,
            2, 1, 2,
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_scored_grid_2() {
        let in_point = Point { x: 1, y: 1};
        let extent_x = 5;
        let extent_y = 7;
        let result = scored_grid_from(in_point, extent_x, extent_y);

        let expected = vec![
            2, 1, 2, 3, 4,
            1, 0, 1, 2, 3,
            2, 1, 2, 3, 4,
            3, 2, 3, 4, 5,
            4, 3, 4, 5, 6,
            5, 4, 5, 6, 7,
            6, 5, 6, 7, 8,
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_grid_is_infinite_top() {
        let extent_x = 3;
        let extent_y = 3;
        let grid = vec![
             0, -1, -1,
            -1, -1, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);

        let grid = vec![
            -1,  0, -1,
            -1, -1, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);

        let grid = vec![
            -1, -1,  0,
            -1, -1, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);
    }

    #[test]
    fn test_grid_is_infinite_centre() {
        let extent_x = 3;
        let extent_y = 3;
        let grid = vec![
            -1, -1, -1,
            -1, -1, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(false, result);

        let grid = vec![
            -1, -1, -1,
            -1,  0, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(false, result);
    }

    #[test]
    fn test_grid_is_infinite_bottom() {
        let extent_x = 3;
        let extent_y = 3;
        let grid = vec![
            -1, -1, -1,
            -1, -1, -1,
             0, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);

        let grid = vec![
            -1, -1, -1,
            -1, -1, -1,
            -1,  0, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);

        let grid = vec![
            -1, -1, -1,
            -1, -1, -1,
            -1, -1,  0,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);
    }

    #[test]
    fn test_grid_is_infinite_left() {
        let extent_x = 3;
        let extent_y = 3;
        let grid = vec![
             0, -1, -1,
            -1, -1, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);

        let grid = vec![
            -1, -1, -1,
             0, -1, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);

        let grid = vec![
            -1, -1, -1,
             0, -1, -1,
            -1, -1, -1,
        ];
        let result = grid_is_infinite(&grid, extent_x, extent_y);
        assert_eq!(true, result);
    }

    #[test]
    fn test_example() {
        let input = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9";
        let result = process(input, 32);
        assert_eq!(16, result);
    }
}
