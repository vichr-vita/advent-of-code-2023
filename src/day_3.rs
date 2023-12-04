// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
//

// number line (len 10 probably)
// is there a number at index i
// if there is a number, which indices does it span
// get number line at row r
// get character at row r column c
//
//
//

// number in the grid gets its signature which is a vec of tuples
// the row in first part will probably will be redundant but let's go with it in case
// the second part has multi-row numbers
//
// get numbers
// go to the symbol location
// for each adjacent symbol construct a number ()
//  the number is constructed by searching contiguous sequence of digits and then reordering the
//  signature by sorting by row and column ascending

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Number {
    val: u32,
    signature: Vec<(usize, usize)>,
}

impl Number {
    fn unique_numbers(numbers: Vec<Number>) -> Vec<Number> {
        let mut unique_numbers = Vec::new();

        for number in numbers {
            if !(&unique_numbers).contains(&number) {
                unique_numbers.push(number);
            }
        }

        unique_numbers
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.signature == other.signature
    }
}

#[derive(Debug)]
pub struct SpecialSymbol {
    row: usize,
    col: usize,
}

pub enum WalkDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub struct EngineSchematic<'a> {
    filepath: &'a str,
}

impl<'a> EngineSchematic<'a> {
    pub fn new(filepath: &'a str) -> io::Result<Self> {
        let _ = File::open(filepath)?;
        Ok(Self { filepath })
    }

    pub fn get_row(&self, row: usize) -> Option<String> {
        let file = File::open(&self.filepath).unwrap(); // the check happens at struct creation
        let reader = BufReader::new(file);

        reader
            .lines()
            .nth(row) // Get the line at the specified index
            .and_then(|line_result| line_result.ok()) // Convert Result to Option
    }

    pub fn get_char(&self, row: usize, col: usize) -> Option<char> {
        self.get_row(row).and_then(|r| r.chars().nth(col))
    }

    pub fn find_special_symbols(&self) -> Vec<SpecialSymbol> {
        let mut i = 0;
        let mut special_symbols = vec![];
        while let Some(_) = self.get_row(i) {
            let mut j = 0;
            while let Some(ch) = self.get_char(i, j) {
                match ch {
                    _ if ('0'..='9').contains(&ch) => (),
                    '.' => (),
                    _ => {
                        special_symbols.push(SpecialSymbol { row: i, col: j });
                    }
                }
                j += 1;
            }
            i += 1;
        }
        special_symbols
    }

    pub fn find_gears(&self) -> Vec<SpecialSymbol> {
        let mut i = 0;
        let mut special_symbols = vec![];
        while let Some(_) = self.get_row(i) {
            let mut j = 0;
            while let Some(ch) = self.get_char(i, j) {
                match ch {
                    '*' => special_symbols.push(SpecialSymbol { row: i, col: j }),
                    _ => (),
                }
                j += 1;
            }
            i += 1;
        }
        special_symbols
    }

    pub fn number_walk(
        &self,
        row: usize,
        col: usize,
        digits: &mut VecDeque<char>,
        signature: &mut VecDeque<(usize, usize)>,
        direction: WalkDirection,
    ) {
        let maybe_digit = self.get_char(row, col);

        if maybe_digit.is_none() {
            // out of bounds
            return;
        }

        let maybe_digit = maybe_digit.unwrap();

        if !maybe_digit.is_digit(10) {
            return;
        }

        let digit = maybe_digit;

        match direction {
            WalkDirection::Left => {
                digits.push_front(digit);
                signature.push_front((row, col));
                if col == 0 {
                    // wouldn't want to overflow
                    return;
                }
                self.number_walk(row, col - 1, digits, signature, direction);
            }
            WalkDirection::Right => {
                digits.push_back(digit);
                signature.push_back((row, col));
                if col == usize::MAX {
                    return;
                }
                self.number_walk(row, col + 1, digits, signature, direction);
            }
        }
    }

    fn vecdeque_to_number(vecdeque: VecDeque<char>) -> u32 {
        vecdeque
            .iter()
            .fold(0u32, |acc, &c| acc * 10 + c.to_digit(10).unwrap() as u32)
    }

    pub fn get_number(&self, row: usize, col: usize) -> Option<Number> {
        let mut digits: VecDeque<char> = VecDeque::new();
        let mut signature_deque: VecDeque<(usize, usize)> = VecDeque::new();

        // root step
        let maybe_digit = self.get_char(row, col);

        if maybe_digit.is_none() {
            // out of bounds
            return None;
        }

        let maybe_digit = maybe_digit.unwrap();

        if !maybe_digit.is_digit(10) {
            return None;
        }

        let digit = maybe_digit;

        digits.push_front(digit);
        signature_deque.push_front((row, col));

        self.number_walk(
            row,
            col + 1,
            &mut digits,
            &mut signature_deque,
            WalkDirection::Right,
        );
        self.number_walk(
            row,
            col - 1,
            &mut digits,
            &mut signature_deque,
            WalkDirection::Left,
        );

        let val = EngineSchematic::vecdeque_to_number(digits);
        let signature = Vec::from(signature_deque);
        Some(Number { val, signature })
    }

    pub fn get_adjacent_numbers_for_symbol(&self, symbol: &SpecialSymbol) -> Vec<Number> {
        let mut numbers: Vec<Number> = vec![];
        let values: [i32; 3] = [-1, 0, 1];
        for &i in &values {
            for &j in &values {
                if let (0, 0) = (i, j) {
                    continue;
                }

                // Check for potential negative indices and skip if found
                if (i < 0 && symbol.row < i.abs() as usize)
                    || (j < 0 && symbol.col < j.abs() as usize)
                {
                    continue;
                }

                // Perform addition with type conversion
                let row_index = (symbol.row as i32 + i) as usize;
                let col_index = (symbol.col as i32 + j) as usize;

                let maybe_number = self.get_number(row_index, col_index);
                if let Some(number) = maybe_number {
                    numbers.push(number);
                }
            }
        }
        Number::unique_numbers(numbers)
    }

    /// return the numbers if there are only two adjacent
    pub fn get_two_part_number(&self, symbol: &SpecialSymbol) -> Option<Vec<Number>> {
        let nums = self.get_adjacent_numbers_for_symbol(symbol);
        if nums.len() == 2 {
            Some(nums)
        } else {
            None
        }
    }
}

pub fn solution_1(filepath: &str) -> u32 {
    let engine_schematic = EngineSchematic::new(filepath).expect("file not found");
    let symbols = engine_schematic.find_special_symbols();
    let mut total_nums: Vec<Number> = vec![];
    for symbol in symbols {
        let nums = engine_schematic.get_adjacent_numbers_for_symbol(&symbol);
        for num in nums {
            total_nums.push(num);
        }
    }
    let total_nums = Number::unique_numbers(total_nums);
    let mut total: u32 = 0;
    for num in total_nums {
        total += num.val;
    }
    total
}

pub fn solution_2(filepath: &str) -> u32 {
    let engine_schematic = EngineSchematic::new(filepath).expect("file not found");
    let symbols = engine_schematic.find_gears();
    let mut total_nums_two_part: Vec<Vec<Number>> = vec![];
    for symbol in symbols {
        let maybe_nums = engine_schematic.get_two_part_number(&symbol);
        if let Some(nums) = maybe_nums {
            total_nums_two_part.push(nums);
        }
    }
    let mut total: u32 = 0;
    for nums_two_part in total_nums_two_part {
        let mut mult_total: u32 = 1;
        for num in nums_two_part {
            mult_total *= num.val;
        }
        total += mult_total;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    const FILE_PATH: &str = "data/day_3.txt";

    #[test]
    fn engine_get_row() {
        let expected = String::from(".......427............................................683&.726...303........*.......905.......................&..115.....412.479.....491....");
        let engine_schematic = EngineSchematic::new(FILE_PATH);
        let maybe_row = engine_schematic.expect("not found").get_row(7);

        assert!(maybe_row.is_some());

        assert_eq!(maybe_row.unwrap(), expected);
    }

    #[test]
    fn engine_get_symbols() {
        let engine_schematic = EngineSchematic::new(FILE_PATH).unwrap();
        let symbols = engine_schematic.find_special_symbols();
        assert!(symbols.len() > 0);
    }

    #[test]
    fn engine_get_number() {
        let engine_schematic = EngineSchematic::new(FILE_PATH).unwrap();
        let number = engine_schematic.get_number(0, 10);
        assert!(number.is_some());
        assert_eq!(number.unwrap().val, 798);

        let num = engine_schematic.get_number(99, 89);
        assert!(num.is_some());
        println!("{:?}", num.as_ref().unwrap());
        assert_eq!(num.unwrap().val, 924);
    }

    #[test]
    fn engine_numbers_adjacent() {
        let engine_schematic = EngineSchematic::new(FILE_PATH).unwrap();
        let symbol = SpecialSymbol { row: 6, col: 96 };
        let nums = engine_schematic.get_adjacent_numbers_for_symbol(&symbol);
        assert_eq!(nums.len(), 2);
        let symbol = SpecialSymbol { row: 15, col: 36 };
        let nums = engine_schematic.get_adjacent_numbers_for_symbol(&symbol);
        assert_eq!(nums.len(), 2);
    }

    #[test]
    fn day_3_part_1() {
        let sol = solution_1(FILE_PATH);
        println!("Day 3 part 1: {}", sol);
    }

    #[test]
    fn day_3_part_2() {
        let sol = solution_2(FILE_PATH);
        println!("Day 3 part 2: {}", sol);
    }
}
