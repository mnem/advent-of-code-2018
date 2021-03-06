use std::fs;

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
    let output = reduce(input);
    return output.len();
}

fn reduce(string: String) -> String {
    string.chars().fold(String::new(), |mut acc, c| {
        match acc.pop() {
            Some(last) => {
                if !is_reactive(last, c) {
                    acc.push(last);
                    acc.push(c);
                }
            },
            None => acc.push(c),
        }
        acc
    })
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
        assert_eq!(10, result);
    }
}
