use std::fs;
use std::collections::HashSet;

fn main() {
    let input = read_input_lines();

    let total_calibration = calibrate_from_lines(input);

    println!("Final calibration: {}\n", total_calibration);
}

fn read_input_lines() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}


fn calibrate_from_lines(input: String) -> i32 {
    let mut seen = HashSet::new();
    let mut total_calibration = 0;
    seen.insert(total_calibration);

    let mut loop_count = 0;
    let max_loops = 10_000;
    loop {
        loop_count += 1;
        if loop_count >= max_loops {
            panic!("No repeat found after looping over all input {} times!", max_loops);
        }

        for line in input.lines() {
            let line = line.trim();
            if line.len() == 0 {
                continue;
            }

            let calibration: i32 = line.parse().unwrap();
            total_calibration += calibration;

            if seen.contains(&total_calibration) {
                // Found the first repeated value, finish
                return total_calibration;
            }

            seen.insert(total_calibration);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_no_trailing_newline() {
        let input_lines = String::from("+1\n-1");
        let result = calibrate_from_lines(input_lines);

        assert_eq!(0, result);
    }

    #[test]
    fn test_with_trailing_newlines() {
        let input_lines = String::from("+1\n-1\n\n\n");
        let result = calibrate_from_lines(input_lines);

        assert_eq!(0, result);
    }

    #[test]
    fn test_with_trailing_newlines_and_whitespace() {
        let input_lines = String::from("+1\n-1\n  \n\n");
        let result = calibrate_from_lines(input_lines);

        assert_eq!(0, result);
    }

    #[test]
    fn test_looping_1() {
        let input_lines = String::from("+1\n-1\n");
        let result = calibrate_from_lines(input_lines);

        assert_eq!(0, result);
    }

    #[test]
    fn test_looping_2() {
        let input_lines = String::from("+3\n+3\n+4\n-2\n-4\n");
        let result = calibrate_from_lines(input_lines);

        assert_eq!(10, result);
    }

    #[test]
    fn test_looping_3() {
        let input_lines = String::from("-6\n+3\n+8\n+5\n-6\n");
        let result = calibrate_from_lines(input_lines);

        assert_eq!(5, result);
    }

    #[test]
    fn test_looping_4() {
        let input_lines = String::from("+7\n+7\n-2\n-7\n-4\n");
        let result = calibrate_from_lines(input_lines);

        assert_eq!(14, result);
    }
}
