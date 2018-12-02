use std::fs;
use std::collections::HashMap;

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
    let mut num_pairs = 0;
    let mut num_triplets = 0;

    for line in input.lines() {
        // Cleanup the input line
        let line = line.trim();
        if line.len() == 0 {
            continue
        }

        let letter_counts = count_letters(line);
        let (pairs_adjustment, triplets_adjustment) = score_pairs_and_triplets(letter_counts);

        // Update the counts
        num_pairs += pairs_adjustment;
        num_triplets += triplets_adjustment;
    }

    checksum(num_pairs, num_triplets)
}

fn score_pairs_and_triplets(letter_counts: HashMap<char, i32>) -> (i32, i32) {
    let mut pairs_adjustment = 0;
    let mut triplets_adjustment = 0;
    for (_, count) in letter_counts {
        match count {
            2 => pairs_adjustment = 1,
            3 => triplets_adjustment = 1,
            _ => {}
        }
    }
    (pairs_adjustment, triplets_adjustment)
}

fn count_letters(line: &str) -> HashMap<char, i32> {
    let mut letter_counts = HashMap::new();
    for letter in line.chars() {
        letter_counts.entry(letter)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);
    }
    letter_counts
}

fn checksum(num_pairs: i32, num_triplets: i32) -> i32 {
    num_pairs * num_triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = String::from("abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n");
        let result = process_lines(input);
        assert_eq!(12, result);
    }
}
