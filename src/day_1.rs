use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// solution 2
// construct a trie of digits
// go from start and end and send substrings
// once both searches yield a valid result, convert to digits

#[derive(Debug)]
pub enum DayOneError {
    ParsingError,
}

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn new_digits() -> Self {
        let mut trie = Trie::new();

        for word in [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ]
        .iter()
        {
            trie.insert(word);
        }
        trie
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default);
        }
        node.is_end_of_word = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return false,
            }
        }
        node.is_end_of_word
    }

    pub fn contains_word(&self, s: &str) -> bool {
        for (start_index, _) in s.char_indices() {
            if self.search_from(&s[start_index..]) {
                return true;
            }
        }
        false
    }

    pub fn search_from(&self, s: &str) -> bool {
        let mut node = &self.root;
        for ch in s.chars() {
            match node.children.get(&ch) {
                Some(next_node) => node = next_node,
                None => return false,
            }
            if node.is_end_of_word {
                return true;
            }
        }
        false
    }
}

fn get_first_last_digit_as_u32(s: &[char]) -> u32 {
    // guaranteed to contain two digits
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    let mut start: usize = 0;
    let mut end: usize = s.len() - 1;
    while start <= end {
        let curr_first = s[start];
        let curr_last = s[end];

        if curr_first.is_digit(10) {
            first = Some(curr_first);
        }

        if curr_last.is_digit(10) {
            last = Some(curr_last);
        }

        if first.is_none() {
            start += 1;
        }
        if last.is_none() {
            end -= 1;
        }

        if first.is_some() && last.is_some() {
            break;
        }
    }
    let first = first.expect("first should have been found");
    let last = last.expect("last should have been found");
    let concatenated = first.to_string() + &last.to_string();
    concatenated.parse::<u32>().expect("parsing failed")
}

fn str_to_char_digit(s: &str) -> Option<char> {
    match s {
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => s.chars().next(),
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn get_first_and_last_digit_include_words(s: &str) -> u32 {
    let mut left_start: usize = 0;
    let mut left_end: usize = left_start;

    let mut right_end: usize = s.len(); // exclusive take so no minus one
    let mut right_start: usize = right_end;

    let mut left_contained_found = false;
    let mut right_contained_found = false;

    let trie = Trie::new_digits();

    while !left_contained_found || !right_contained_found {
        // the string won't be empty so we start by incrementing to make
        // it len 1
        if !left_contained_found {
            left_end += 1;
        }
        if !right_contained_found {
            right_start -= 1;
        }

        let left_subslice = &s[left_start..left_end];
        let right_subslice = &s[right_start..right_end];

        left_contained_found = trie.contains_word(&left_subslice);
        right_contained_found = trie.contains_word(&right_subslice);
    }

    let mut left_found = false;
    let mut right_found = false;

    while !left_found || !right_found {
        let left_subslice = &s[left_start..left_end];
        let right_subslice = &s[right_start..right_end];

        left_found = trie.search(left_subslice);
        right_found = trie.search(right_subslice);

        if !left_found {
            left_start += 1;
        }
        if !right_found {
            right_end -= 1;
        }
    }

    let left_digit_text = &s[left_start..left_end];
    let right_digit_text = &s[right_start..right_end];

    let left_digit = str_to_char_digit(left_digit_text).expect("failed to parse left digit");
    let rigth_digit = str_to_char_digit(right_digit_text).expect("failed to parse right digit");

    let concatenated = left_digit.to_string() + &rigth_digit.to_string();
    concatenated
        .parse::<u32>()
        .expect("parsing of digit failed")
}

pub fn solution_part_1(filename: &str) -> Result<u32, DayOneError> {
    let maybe_file = File::open(filename);

    match maybe_file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();

            let mut total: u32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    let chars_ip: Vec<char> = ip.chars().collect();
                    let num = get_first_last_digit_as_u32(&chars_ip);
                    total += num;
                }
            }

            Ok(total)
        }
        Err(_) => Err(DayOneError::ParsingError),
    }
}

pub fn solution_part_2(filename: &str) -> Result<u32, DayOneError> {
    let maybe_file = File::open(filename);

    match maybe_file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();

            let mut total: u32 = 0;
            for line in lines {
                if let Ok(ip) = line {
                    let num = get_first_and_last_digit_include_words(&ip);
                    total += num;
                }
            }

            Ok(total)
        }
        Err(_) => Err(DayOneError::ParsingError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_last_digit_success() {
        let s = "sd3fklsdj2";
        let chars: Vec<char> = s.chars().collect();
        let num = get_first_last_digit_as_u32(&chars);
        assert_eq!(num, 32);

        let s = "sd3fslkdfjsdkljf239klsdj2";
        let chars: Vec<char> = s.chars().collect();
        let num = get_first_last_digit_as_u32(&chars);
        assert_eq!(num, 32);

        let s = "treb7uchet";
        let chars: Vec<char> = s.chars().collect();
        let num = get_first_last_digit_as_u32(&chars);
        assert_eq!(num, 77);
    }

    #[test]
    fn day_1_part_1_success() {
        let file_path = "data/day_1.txt";
        let result = solution_part_1(file_path);
        assert!(result.is_ok());
        println!("result part 1: {:?}", result.unwrap());
    }

    #[test]
    fn trie_search_works() {
        let mut trie = Trie::new();

        // Insert words
        for word in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        {
            trie.insert(word);
        }

        let is_number = trie.search("two");
        assert!(is_number);

        let not_number = trie.search("xdfplk5sdf");
        assert!(!not_number);
    }

    #[test]
    fn trie_contains_word_works() {
        let mut trie = Trie::new();

        // Insert words
        for word in [
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ]
        .iter()
        {
            trie.insert(word);
        }

        let is_number = trie.contains_word("sdfsdtwosdfs");
        assert!(is_number);

        let is_number = trie.contains_word("sdfsdt7wsdfs");
        assert!(is_number);

        let not_number = trie.contains_word("xdfplksdf");
        assert!(!not_number);
    }

    #[test]
    fn get_first_and_last_digit_inlude_words_success() {
        let s = "eightwothree";
        let num = get_first_and_last_digit_include_words(s);
        assert_eq!(num, 83);

        let s = "eightwothree5schmee";
        let num = get_first_and_last_digit_include_words(s);
        assert_eq!(num, 85);

        let s = "ei7ghtwothree5schmee";
        let num = get_first_and_last_digit_include_words(s);
        assert_eq!(num, 75);

        let s = "xdtwoxd";
        let num = get_first_and_last_digit_include_words(s);
        assert_eq!(num, 22);

        let s = "six9mnfjmtsf2kfmznkxntninesevenrpmfjfpgsk";
        let num = get_first_and_last_digit_include_words(s);
        assert_eq!(num, 67);

        let calibration_strs = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let calibration_nums = vec![29, 83, 13, 24, 42, 14, 76];

        for i in 0..calibration_strs.len() {
            assert_eq!(
                get_first_and_last_digit_include_words(calibration_strs[i]),
                calibration_nums[i]
            );
        }
    }

    #[test]
    fn day_1_part_2_success() {
        let file_path = "data/day_1.txt";
        let result = solution_part_2(file_path);
        assert!(result.is_ok());
        println!("result part 2: {:?}", result.unwrap());
    }
}
