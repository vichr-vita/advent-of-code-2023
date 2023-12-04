use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Turn {
    cube_counts: HashMap<String, u32>,
}

impl Turn {
    pub fn from_tuples(turn_tuples: Vec<(&str, &str)>) -> Self {
        let mut cube_counts = HashMap::with_capacity(5);
        for t in turn_tuples {
            let k = String::from(t.1);
            let v =
                t.0.parse::<u32>()
                    .expect(format!("parsing failed {}", t.0).as_str());
            cube_counts.insert(k, v);
        }
        Self { cube_counts }
    }

    pub fn power_set(&self) -> u32 {
        let mut total = 1;
        for val in self.cube_counts.values() {
            total *= val;
        }
        total
    }
}

pub struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl Game {
    pub fn from_line(line: &str) -> Self {
        let colon_idx = line.find(":").expect("Colon found");
        let game_id_unparsed = &line[..colon_idx];

        let game_id_unparsed = game_id_unparsed
            .split(" ")
            .last()
            .expect(format!("Invalid game id format {}", game_id_unparsed).as_str());
        let id = game_id_unparsed.parse::<u32>().expect(
            format!(
                "Invalid game id format to parse to u32 {}",
                game_id_unparsed
            )
            .as_str(),
        );
        let turns_unparsed = &line[colon_idx + 2..];
        let turns_parsed: Vec<Vec<(&str, &str)>> = turns_unparsed
            .split("; ")
            .map(|cubes_str| {
                cubes_str
                    .split(", ")
                    .filter_map(|cube_turn| {
                        cube_turn
                            .split_once(' ')
                            .or_else(|| panic!("cube_turn: {} is invalid", cube_turn))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let turns: Vec<Turn> = turns_parsed
            .into_iter()
            .map(|turn| Turn::from_tuples(turn))
            .collect();

        Self { id, turns }
    }

    pub fn is_valid(&self, validator_turn: &Turn) -> bool {
        for turn in self.turns.iter() {
            for color in turn.cube_counts.keys() {
                let validator_count = validator_turn.cube_counts.get(color);
                if validator_count.is_none() {
                    panic!("color {} does not exist in validator", color);
                }
                let validator_count = validator_count.unwrap();
                let cube_count = turn.cube_counts.get(color).unwrap(); // assume exists
                if cube_count <= validator_count {
                    continue;
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn take_most_from_turns(&self) -> Turn {
        let mut most_turn_hm: HashMap<String, u32> = HashMap::new();

        for turn in self.turns.iter() {
            for color in turn.cube_counts.keys() {
                let cube_count = turn.cube_counts.get(color).unwrap(); // exists
                let maybe_curr_most = most_turn_hm.get_mut(color);
                if let Some(curr_most) = maybe_curr_most {
                    if *curr_most < *cube_count {
                        *curr_most = *cube_count;
                    }
                } else {
                    most_turn_hm.insert(color.to_string(), *cube_count);
                }
            }
        }

        Turn {
            cube_counts: most_turn_hm,
        }
    }

    pub fn power_minimum(&self) -> u32 {
        let min_turn = self.take_most_from_turns();
        min_turn.power_set()
    }
}

pub fn solution_part_1(filename: &str) -> u32 {
    let file = File::open(filename).expect("failed to parse file.");
    let validator_counts = vec![("12", "red"), ("13", "green"), ("14", "blue")];
    let validator_turn = Turn::from_tuples(validator_counts);

    let lines = io::BufReader::new(file).lines();

    let mut total: u32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let game = Game::from_line(ip.as_str());
            if game.is_valid(&validator_turn) {
                total += game.id
            }
        }
    }
    total
}

pub fn solution_part_2(filename: &str) -> u32 {
    let file = File::open(filename).expect("failed to parse file.");

    let lines = io::BufReader::new(file).lines();
    let mut total: u32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let game = Game::from_line(ip.as_str());
            total += game.power_minimum();
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_line(line);
        assert_eq!(game.id, 1);
        assert_eq!(game.turns.len(), 3);
    }

    #[test]
    fn game_check_validity() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_line(line);
        let validator_counts = vec![("12", "red"), ("13", "green"), ("14", "blue")];
        let validator_turn = Turn::from_tuples(validator_counts);
        let is_valid = game.is_valid(&validator_turn);
        assert!(is_valid)
    }

    #[test]
    fn day_2_part_1() {
        let filename = "data/day_2.txt";
        let solution = solution_part_1(filename);
        println!("Solution part 1: {}", solution);
    }

    #[test]
    fn game_power() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_line(line);
        assert_eq!(game.power_minimum(), 48);
    }

    #[test]
    fn day_2_part_2() {
        let filename = "data/day_2.txt";
        let solution = solution_part_2(filename);
        println!("Solution part 2: {}", solution);
    }
}
