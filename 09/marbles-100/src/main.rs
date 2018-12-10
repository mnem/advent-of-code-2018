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

fn process(input: &str) -> usize {
    let setup = GameSetup::from(input);
    let mut game = GameState::new(setup);

    game.play();

    let (_, high_score) = game.highest_score();

    return high_score;
}

struct GameSetup {
    num_players: usize,
    last_marble_score: usize,
}

impl GameSetup {
    fn from(string: &str) -> GameSetup {
        let re = Regex::new(r"(?P<players>\d*) players; last marble is worth (?P<last_marble_score>\d*)").unwrap();
        let captures = re.captures(string).expect("Malformed game setup string.");

        let num_players = captures["players"].parse().expect("Malformed players number.");
        let last_marble_score: usize = captures["last_marble_score"].parse().expect("Malformed last marble score.");

        GameSetup { num_players, last_marble_score: last_marble_score * 100 }
    }
}

struct GameState {
    setup: GameSetup,
    player_scores: Vec<usize>,
    placed_marbles: VecDeque<usize>,
}

impl GameState {
    fn new(setup: GameSetup) -> GameState {
        let player_scores = vec![0usize; setup.num_players as usize];
        let mut placed_marbles = VecDeque::new();
        placed_marbles.push_front(0);

        GameState { setup, player_scores, placed_marbles }
    }

    fn play(&mut self) {
        for marble in 1..= self.setup.last_marble_score {
            self.place_next_marble(marble);
        }
    }

    fn place_next_marble(&mut self, next_marble_score: usize) {
        if next_marble_score != 0 && next_marble_score % 23 == 0 {
            self.place_23_marble(next_marble_score);
        } else {
            self.place_normal_marble(next_marble_score);
        }
    }

    fn place_normal_marble(&mut self, marble: usize) {
        // Rotate the buffer CW
        for _ in 0..2 {
            let v = self.placed_marbles.pop_front().expect("Can't rotate CW");
            self.placed_marbles.push_back(v);
        }

        self.placed_marbles.push_front(marble);
    }

    fn place_23_marble(&mut self, marble: usize) {
        // Rotate the buffer CCW
        for _ in 0..7 {
            let v = self.placed_marbles.pop_back().expect("Can't rotate CCW");
            self.placed_marbles.push_front(v);
        }

        let taken_marble = self.placed_marbles.pop_front().expect("Could not take marble");

        let player = marble % self.player_scores.len();
        self.player_scores[player] += marble + taken_marble;
    }

    fn highest_score(&self) -> (usize, usize) {
        let mut high_player = 0usize;
        let mut high_score = 0usize;
        for (player, score) in self.player_scores.iter().enumerate() {
            if *score > high_score {
                high_player = player;
                high_score = *score;
            }
        }
        assert_ne!(0, high_score);
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
        assert_eq!(25 * 100, result.last_marble_score);
    }

    #[test]
    fn test_game_state_from_setup() {
        let setup = GameSetup { num_players: 9, last_marble_score: 25 };
        let result = GameState::new(setup);
        assert_eq!(9, result.setup.num_players);
        assert_eq!(25, result.setup.last_marble_score);
        assert_eq!(vec![0; 9], result.player_scores);
        assert_eq!(1, result.placed_marbles.len());
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
