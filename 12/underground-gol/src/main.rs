use std::fs;
use std::collections::VecDeque;
use regex::Regex;

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

fn process(input: &str) -> i64 {
    let mut game = read_game_from_text(input);

    for i in 1..=20 {
        step(&mut game);
    }

    game_sum(&game)
}

fn read_initial_input_line(line: &str) -> VecDeque<usize> {
    let re = Regex::new(r": (?P<state>.*)").expect("Broken regex");
    let captures = re.captures(line).expect("Malformed state line");
    let state_str = &captures["state"];

    let mut state = VecDeque::new();
    for c in state_str.chars() {
        if c == '#' {
            state.push_back(1);
        } else {
            state.push_back(0);
        }
    }
    return state;
}

struct Rule {
    pattern: usize,
    result: usize,
}

fn read_rule_from_line(line: &str) -> Rule {
    let re = Regex::new(r"(?P<pattern>.*) => (?P<result>.)").expect("Broken regex");
    let captures = re.captures(line).expect("Malformed state line");
    let pattern_str = &captures["pattern"];
    let result_str = &captures["result"];

    let mut pattern = 0;
    for (index, c) in pattern_str.chars().enumerate() {
        if c == '#' {
            pattern = pattern | (1 << (pattern_str.len() - 1 - index));
        }
    }

    let result = match result_str {
        "#" => 1usize,
        _ => 0usize,
    };

    Rule { pattern, result }
}

struct Game {
    state: VecDeque<usize>,
    zero_index: usize,
    rules: Vec<Rule>,
}

fn read_game_from_text(text: &str) -> Game {
    let mut state = None;
    let mut rules = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        if state.is_none() {
            state = Some(read_initial_input_line(line));
        } else {
            rules.push(read_rule_from_line(line));
        }
    }

    return Game { state: state.unwrap(), zero_index: 0, rules }
}

fn grow_state(state: &mut VecDeque<usize>) -> usize {
    if state[state.len() - 1] == 1 {
        state.push_back(0);
        state.push_back(0);
        state.push_back(0);
        state.push_back(0);
    } else if state[state.len() - 2] == 1 {
        state.push_back(0);
        state.push_back(0);
        state.push_back(0);
    } else if state[state.len() - 3] == 1 {
        state.push_back(0);
        state.push_back(0);
    } else if state[state.len() - 4] == 1 {
        state.push_back(0);
    }

    if state[0] == 1 {
        state.push_front(0);
        state.push_front(0);
        state.push_front(0);
        state.push_front(0);
        return 4;
    } else if state[1] == 1 {
        state.push_front(0);
        state.push_front(0);
        state.push_front(0);
        return 3;
    } else if state[2] == 1 {
        state.push_front(0);
        state.push_front(0);
        return 2;
    } else if state[3] == 1 {
        state.push_front(0);
        return 1;
    } else {
        return 0;
    }

}

fn step(game: &mut Game ) {
    game.zero_index += grow_state(&mut game.state);

    let in_state = &game.state;
    let mut out_state = in_state.clone();

    for i in 0..in_state.len() - 5 {
        let window = in_state[i + 0] << 4 | in_state[i + 1] << 3 | in_state[i + 2] << 2 | in_state[i + 3] << 1 | in_state[i + 4];
        if let Some(rule) = game.rules.iter().find(|r| {r.pattern == window} ) {
            out_state[i + 2] = rule.result;
        } else {
            out_state[i + 2] = 0;
        }
    }

    game.state = out_state;
}

fn game_sum(game: &Game) -> i64 {
    let mut sum = 0;

    for (index, value) in game.state.iter().enumerate() {
        if *value == 1 {
            let index = index as i64;
            let zero = game.zero_index as i64;
            sum += index - zero;
        }
    }

    return sum;
}

fn game_state_to_string(game: &Game) -> String {
    let mut s = String::new();
    for (index, value) in game.state.iter().enumerate() {
        if index == game.zero_index {
            if *value == 1 {
                s += "O";
            } else {
                s += "|";
            }
        } else if *value == 1 {
            s += "#";
        } else {
            s += ".";
        }
    }
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_setup_from_string() {
        let input = "initial state: #..#.#..##......###...###";
        let result = read_initial_input_line(input);

        assert_eq!(25, result.len());
        assert_eq!( 1, result[ 0]);
        assert_eq!( 0, result[ 1]);
        assert_eq!( 0, result[ 2]);
        assert_eq!( 1, result[ 3]);
        assert_eq!( 0, result[ 4]);
        assert_eq!( 1, result[ 5]);
        assert_eq!( 0, result[ 6]);
        assert_eq!( 0, result[ 7]);
        assert_eq!( 1, result[ 8]);
        assert_eq!( 1, result[ 9]);
        assert_eq!( 0, result[10]);
        assert_eq!( 0, result[11]);
        assert_eq!( 0, result[12]);
        assert_eq!( 0, result[13]);
        assert_eq!( 0, result[14]);
        assert_eq!( 0, result[15]);
        assert_eq!( 1, result[16]);
        assert_eq!( 1, result[17]);
        assert_eq!( 1, result[18]);
        assert_eq!( 0, result[19]);
        assert_eq!( 0, result[20]);
        assert_eq!( 0, result[21]);
        assert_eq!( 1, result[22]);
        assert_eq!( 1, result[23]);
        assert_eq!( 1, result[24]);
    }

    #[test]
    fn test_rule_from_string() {
        let input = "...## => #";
        let result = read_rule_from_line(input);

        assert_eq!(1, result.result);
        assert_eq!(3, result.pattern);
    }

    #[test]
    fn test_game_from_string() {
        let input = "initial state: #..#.#..##......###...###\n\n...## => #\n..#.. => #\n.#... => #\n.#.#. => #\n.#.## => #\n.##.. => #\n.#### => #\n#.#.# => #\n#.### => #\n##.#. => #\n##.## => #\n###.. => #\n###.# => #\n####. => #\n";
        let result = read_game_from_text(input);

        assert_eq!(25, result.state.len());
        assert_eq!( 1, result.state[ 0]);
        assert_eq!( 0, result.state[ 1]);
        assert_eq!( 0, result.state[ 2]);
        assert_eq!( 1, result.state[ 3]);
        assert_eq!( 0, result.state[ 4]);
        assert_eq!( 1, result.state[ 5]);
        assert_eq!( 0, result.state[ 6]);
        assert_eq!( 0, result.state[ 7]);
        assert_eq!( 1, result.state[ 8]);
        assert_eq!( 1, result.state[ 9]);
        assert_eq!( 0, result.state[10]);
        assert_eq!( 0, result.state[11]);
        assert_eq!( 0, result.state[12]);
        assert_eq!( 0, result.state[13]);
        assert_eq!( 0, result.state[14]);
        assert_eq!( 0, result.state[15]);
        assert_eq!( 1, result.state[16]);
        assert_eq!( 1, result.state[17]);
        assert_eq!( 1, result.state[18]);
        assert_eq!( 0, result.state[19]);
        assert_eq!( 0, result.state[20]);
        assert_eq!( 0, result.state[21]);
        assert_eq!( 1, result.state[22]);
        assert_eq!( 1, result.state[23]);
        assert_eq!( 1, result.state[24]);

        assert_eq!(14, result.rules.len());
        assert_eq!(0, result.zero_index);
    }

    #[test]
    fn test_example() {
        let input = "initial state: #..#.#..##......###...###\n\n...## => #\n..#.. => #\n.#... => #\n.#.#. => #\n.#.## => #\n.##.. => #\n.#### => #\n#.#.# => #\n#.### => #\n##.#. => #\n##.## => #\n###.. => #\n###.# => #\n####. => #\n";
        let mut result = read_game_from_text(input);

        println!(" 0: {}", game_state_to_string(&result));
        for i in 1..=20 {
            step(&mut result);
            println!("{:>2}: {}", i, game_state_to_string(&result));
        }

        assert_eq!("....#.|..##....#####...#######....#.#..##...", game_state_to_string(&result));
        assert_eq!(325, game_sum(&result));
    }
}
