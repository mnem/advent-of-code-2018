use std::fs;
use std::cmp::min;
use std::collections::HashSet;

fn main() {
    let input = read_input();
    let result = process(&input);
    println!("Result: {}\n", result);
}

fn read_input() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}

fn process(input: &str) -> usize {
    let input = input.trim().to_string();
    let units = unique_units(&input);

    let mut shortest = usize::max_value();
    for unit in units {
        let filtered = input.chars().filter(|u| { u.to_ascii_lowercase() != unit }).collect();
        let output = reduce(filtered);
        shortest = min(shortest, output.len());
    }

    return shortest;
}

fn unique_units(string: &str) -> Vec<char> {
    let mut uniques = HashSet::new();
    for unit in string.chars() {
        uniques.insert(unit.to_ascii_lowercase());
    }
    uniques.into_iter().collect()
}

fn reduce(string: String) -> String {
    let mut output = string;
    loop {
        let input = output;
        output = String::from("");

        let mut last_unit = None;
        for unit in input.chars() {
            match last_unit {
                Some(previous) => {
                    if is_reactive(previous, unit) {
                        last_unit = None;
                    } else {
                        output.push(previous);
                        last_unit = Some(unit);
                    }
                },
                None => last_unit = Some(unit),
            }
        }

        if let Some(last_unit) = last_unit {
            output.push(last_unit);
        }

        if output.len() == input.len() {
            return output;
        }
    }
}

fn is_reactive(a: char, b: char) -> bool {
    assert_eq!(true, a.is_ascii());
    assert_eq!(true, b.is_ascii());

    if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
        return false;
    }

    if (a.is_lowercase() && b.is_uppercase()) || (a.is_uppercase() && b.is_lowercase()) {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_reactive() {
        assert_eq!(true, is_reactive('a', 'A'));
        assert_eq!(true, is_reactive('A', 'a'));
        assert_eq!(false, is_reactive('A', 'A'));
        assert_eq!(false, is_reactive('A', 'b'));
    }

    #[test]
    fn test_reduce_to_empty() {
        let input = String::from("aA");
        let result = reduce(input);
        assert_eq!("", result);
    }

    #[test]
    fn test_reduce_to_one_from_end() {
        let input = String::from("aAb");
        let result = reduce(input);
        assert_eq!("b", result);
    }

    #[test]
    fn test_reduce_to_one_from_start() {
        let input = String::from("baA");
        let result = reduce(input);
        assert_eq!("b", result);
    }

    #[test]
    fn test_reduce_to_empty_2_steps() {
        let input = String::from("baAB");
        let result = reduce(input);
        assert_eq!("", result);
    }

    #[test]
    fn test_reduce_to_one_3_steps() {
        let input = String::from("baABB");
        let result = reduce(input);
        assert_eq!("B", result);
    }

    #[test]
    fn test_example() {
        let input = "dabAcCaCBAcCcaDA";
        let result = process(input);
        assert_eq!(4, result);
    }

    #[test]
    fn test_unique_units() {
        let input = "dabAcCaCBAcCcaDA";
        let mut result = unique_units(input);
        assert_eq!(4, result.len());
        result.sort();
        assert_eq!(vec!['a','b','c','d'], result);
    }
}
