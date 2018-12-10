use std::fs;
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

fn process(input: &str) -> i32 {
    let setup = GameSetup::from(input);
    let mut game = GameState::new(setup);

    game.play();

    let (high_player, high_score) = game.highest_score();

    return high_score;
}

fn modulus(a: i32, b: i32) -> i32 {
    if a == 0 {
        return 0;
    } else if a > 0 {
        return a % b;
    } else {
        return (a % b) + b;
    }
}

struct GameSetup {
    num_players: i32,
    last_marble_score: i32,
}

impl GameSetup {
    fn from(string: &str) -> GameSetup {
        let re = Regex::new(r"(?P<players>\d*) players; last marble is worth (?P<last_marble_score>\d*)").unwrap();
        let captures = re.captures(string).expect("Malformed game setup string.");

        let num_players = captures["players"].parse().expect("Malformed players number.");
        let last_marble_score: i32 = captures["last_marble_score"].parse().expect("Malformed last marble score.");

        GameSetup { num_players, last_marble_score, }
    }
}

struct GameState {
    setup: GameSetup,
    player_scores: Vec<i32>,
    placed_marbles: Vec<i32>,
    current_marble_position: i32,
    next_marble_score: i32,
}

impl GameState {
    fn new(setup: GameSetup) -> GameState {
        let player_scores = vec![0; setup.num_players as usize];
        let placed_marbles = vec![0];
        let current_marble_position = 0;
        let next_marble_score = 1;

        GameState { setup, player_scores, placed_marbles, current_marble_position, next_marble_score }
    }

    fn play(&mut self) {
        while self.next_marble_score <= self.setup.last_marble_score {
            self.place_next_marble(self.next_marble_score);
            self.next_marble_score += 1;
        }
    }

    fn place_next_marble(&mut self, next_marble_score: i32) {
        if next_marble_score != 0 && next_marble_score % 23 == 0 {
            self.place_23_marble(next_marble_score);
        } else {
            self.place_normal_marble(next_marble_score);
        }
    }

    fn place_normal_marble(&mut self, marble: i32) {
        let position = modulus(self.current_marble_position + 2, self.placed_marbles.len() as i32);
        assert!(position >= 0);

        self.placed_marbles.insert(position as usize, marble);
        self.current_marble_position = position;
    }

    fn place_23_marble(&mut self, marble: i32) {
        let player = self.next_marble_score % self.player_scores.len() as i32;
        let position = modulus(self.current_marble_position - 7, self.placed_marbles.len() as i32);
        assert!(position >= 0);

        let taken_marble = self.placed_marbles.remove(position as usize);
        let score = marble + taken_marble;
        self.player_scores[player as usize] += score;

        self.current_marble_position = position;
    }

    fn board_string(&self) -> String {
        let mut board = String::new();
        for (index, marble) in self.placed_marbles.iter().enumerate() {
            if index == self.current_marble_position as usize {
                board += &format!(" ({})", marble);
            } else {
                board += &format!(" {}", marble);
            }
        }
        return board;
    }

    fn highest_score(&self) -> (usize, i32) {
        let mut high_player = 0usize;
        let mut high_score = -1;
        for (player, score) in self.player_scores.iter().enumerate() {
            if *score > high_score {
                high_player = player;
                high_score = *score;
            }
        }
        assert_ne!(-1, high_score);
        return (high_player, high_score);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_setup_from_string() {
        let input = "9 players; last marble is worth 25 points";
        let result = GameSetup::from(input);
        assert_eq!(9, result.num_players);
        assert_eq!(25, result.last_marble_score);
    }

    #[test]
    fn test_game_state_from_setup() {
        let setup = GameSetup { num_players: 9, last_marble_score: 25 };
        let result = GameState::new(setup);
        assert_eq!(9, result.setup.num_players);
        assert_eq!(25, result.setup.last_marble_score);
        assert_eq!(vec![0; 9], result.player_scores);
        assert_eq!(vec![0; 1], result.placed_marbles);
        assert_eq!(0, result.current_marble_position);
        assert_eq!(1, result.next_marble_score);
    }

    #[test]
    fn test_example_state_from_setup() {
        let setup = GameSetup { num_players: 9, last_marble_score: 25 };
        let mut game = GameState::new(setup);

        game.play();

        let (high_player, high_score) = game.highest_score();
        assert_eq!(5, high_player);
        assert_eq!(32, high_score);

    }
}
