use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct Card {
    id: u32,
    winning_nums: Vec<u32>,
    your_nums: Vec<u32>,
}

impl Card {
    pub fn from_str(s: &str) -> Self {
        let colon_idx = s.find(":").expect("Colon found");
        let card_id_unparsed = &s[..colon_idx];
        let card_id_str = card_id_unparsed
            .split(" ")
            .last()
            .expect(format!("card id invalid {}", card_id_unparsed).as_str());
        let id = card_id_str
            .parse::<u32>()
            .expect(format!("Invalid card id format to parse to u32 {}", card_id_str).as_str());

        // winning | your_nums
        let card_sides_unparsed = &s[colon_idx + 1..];
        let mut card_sides_split = card_sides_unparsed.split("|");

        let winning_side_unparsed = card_sides_split.nth(0).expect("winning side parse fail");
        let your_side_unparsed = card_sides_split.last().expect("your side parse fail");

        let winning_nums: Vec<u32> = winning_side_unparsed
            .split_whitespace()
            .into_iter()
            .map(|num_unparsed| {
                num_unparsed.parse::<u32>().expect(
                    format!("Winning side - Invalid num to parse: {}", num_unparsed).as_str(),
                )
            })
            .collect();

        let your_nums: Vec<u32> = your_side_unparsed
            .split_whitespace()
            .into_iter()
            .map(|num_unparsed| {
                num_unparsed.parse::<u32>().expect(
                    format!("Winning side - Invalid num to parse: {}", num_unparsed).as_str(),
                )
            })
            .collect();

        Self {
            id,
            winning_nums,
            your_nums,
        }
    }

    pub fn get_winning_numbers(&self) -> Vec<u32> {
        let winning_set: HashSet<u32> = self.winning_nums.iter().cloned().collect();
        let your_set: HashSet<u32> = self.your_nums.iter().cloned().collect();

        winning_set
            .intersection(&your_set)
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn get_winning_points(&self) -> u32 {
        let winning_numbers_count = self.get_winning_numbers().len();
        if winning_numbers_count == 0 {
            0
        } else {
            2u32.pow((winning_numbers_count as u32) - 1)
        }
    }
}

pub fn solution_1(filename: &str) -> u32 {
    let file = File::open(filename).expect("failed to open file in solution 1");

    let lines = io::BufReader::new(file).lines();

    let mut total: u32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let card = Card::from_str(&ip);
            let pts = card.get_winning_points();

            total += pts;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn card_parse() {
        let line = "Card 159: 13 42 26 92 53 33 44 45 19 90 | 59 95  2 51 25 81 17 30  3 71 36 22 58 90 33 52  8 92 37  6 11 19 45 96 88";
        let card = Card::from_str(line);
        assert_eq!(card.winning_nums[0], 13);
        assert_eq!(card.your_nums[0], 59);
    }

    #[test]
    fn card_my_winning_nums() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(line);
        assert_eq!(card.get_winning_numbers().len(), 4);
    }

    #[test]
    fn card_my_winning_points() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(line);
        assert_eq!(card.get_winning_points(), 8);
    }

    #[test]
    fn day_4_part_1() {
        let filename = "data/day_4.txt";
        let sol = solution_1(&filename);
        println!("Day 4 Solution 1: {}", sol);
    }
}
