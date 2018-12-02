use std::fs;

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

fn process_lines(input: String) -> String {
    let mut box_ids: Vec<&str> = input.lines()
        .map(|id| {id.trim()})
        .filter(|id| {id.len() > 0})
        .collect();

    while let Some(id) = box_ids.pop() {
        for other_id in box_ids.iter().cloned() {
            let matching = matching_chars(id, other_id);
            if matching.len() == (id.len() - 1) {
                return matching;
            }
        }
    }
    return String::new();
}

fn matching_chars(id: &str, other_id: &str) -> String {
    id.chars().zip(other_id.chars())
        .filter_map(|(a, b)| {
            if a == b {
                Some(a)
            } else {
                None
            }
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = String::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n");
        let result = process_lines(input);
        assert_eq!("fgij", result);
    }
}
