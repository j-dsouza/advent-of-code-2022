use core::panic;
use std::fs;
use std::{error::Error, collections::HashSet};
use std::string::ParseError;

use crate::aoc_core::Solution;

struct Rucksack {
    section_1: String,
    section_2: String
}

impl Rucksack {
    pub fn find_common_char(&self) -> Result<char, ParseError> {
        let s2_set: HashSet<char> = self.section_2.chars().collect();

        for char in self.section_1.chars() {
            if s2_set.contains(&char) {
                return Ok(char)
            }
        }

        panic!("No match")
    }

    pub fn priority(l: char) -> Result<i32, Box<dyn Error>> {
        let ascii_val = l as i32;
        // println!("{}", ascii_val);
        if ascii_val < 91 {
            return Ok((ascii_val % 65) + 27)
        } else {
            return Ok((ascii_val % 97) + 1)
        }
    }
}

struct ElfGroup {
    rucksack_1: String,
    rucksack_2: String,
    rucksack_3: String
}

impl ElfGroup {
    pub fn find_common_char(&self) -> Result<char, ParseError> {
        let r2_set: HashSet<char> = self.rucksack_2.chars().collect();
        let r3_set: HashSet<char> = self.rucksack_3.chars().collect();

        for char in self.rucksack_1.chars() {
            if r2_set.contains(&char) {
                if r3_set.contains(&char) {
                    return Ok(char)
                }
            }
        }
        panic!("No match")
    }
}


fn parse_pt1(input: String) -> Result<Vec<Rucksack>, Box<dyn Error>> {
    let input_lines = input.lines();

    let mut input_vec: Vec<Rucksack> = vec![];

    for line in input_lines {
        let length = line.len();
        let (s1, s2) = line.split_at(length/2);
        let s1_string = s1.to_string();
        let s2_string = s2.to_string();
        input_vec.push(Rucksack{section_1: s1_string, section_2: s2_string})
    }

    Ok(input_vec)
}

fn parse_pt2(input: String) -> Result<Vec<ElfGroup>, Box<dyn Error>> {
    let input_lines = input.lines().collect::<Vec<&str>>().to_owned();
    // let lines_vec: Vec<String> = input_lines;
    let mut input_vec: Vec<ElfGroup> = vec![];

    for t in input_lines.chunks(3) {
        if let [line_1, line_2, line_3] = t{
        input_vec.push(ElfGroup {
            rucksack_1: line_1.to_string(),
            rucksack_2: line_2.to_string(),
            rucksack_3: line_3.to_string()
        })}
    }

    Ok(input_vec)
}

pub fn solve() -> Result<Solution, Box<dyn Error>> {
    let input = fs::read_to_string("data/day_3/3.txt")?;

    let input_pt1_parsed = parse_pt1(input.clone())?;

    let mut sum = 0;
    for rucksack in input_pt1_parsed {
        let common_char = rucksack.find_common_char()?;
        sum += Rucksack::priority(common_char)?;
    }

    let input_pt2_parsed = parse_pt2(input.clone())?;

    let mut sum_2 = 0;
    for elf_group in input_pt2_parsed {
        let common_char = elf_group.find_common_char()?;
        sum_2 += Rucksack::priority(common_char)?;
    }

    Ok(Solution {part_1: sum, part_2: sum_2})
}
