use crate::aoc_core;
use std::fmt::{self, Display};
use std::vec::Vec;
use std::{error::Error, fs};

#[derive(PartialEq, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

pub enum MatchResult {
    Win,
    Lose,
    Draw,
}

impl Hand {
    pub fn beats(&self) -> Self {
        match &*self {
            Hand::Rock => Self::Scissors,
            Hand::Paper => Self::Rock,
            Hand::Scissors => Self::Paper,
        }
    }

    pub fn loses(&self) -> Self {
        match &*self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn fight(&self, other: Hand) -> MatchResult {
        match (self.beats(), other.beats()) {
            _ if self.beats() == other => MatchResult::Win,
            _ if *self == other.beats() => MatchResult::Lose,
            _ => MatchResult::Draw,
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Hand::Rock => "Rock",
            Hand::Paper => "Paper",
            Hand::Scissors => "Scissors",
        };
        write!(f, "{}", name)
    }
}

impl MatchResult {
    pub fn value(&self) -> i32 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Lose => 0,
        }
    }

    pub fn interpolate(&self, other: Hand) -> Hand {
        match self {
            MatchResult::Draw => other,
            MatchResult::Lose => other.beats(),
            MatchResult::Win => other.loses(),
        }
    }
}

fn string_to_vector_line_break(input: String) -> Result<Vec<String>, Box<dyn Error>> {
    let split = input.lines();

    let mut output: Vec<String> = vec![];
    for line in split {
        output.push(line.to_string())
    }

    return Ok(output);
}

fn string_to_hands(input: String) -> Result<(Hand, Hand), Box<dyn Error>> {
    let mut hands_string = input.split_whitespace();
    let other_hand_str = hands_string.next().unwrap();
    let own_hand_str = hands_string.next().unwrap();

    let other_hand = match other_hand_str {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("Error"),
    };

    let own_hand = match own_hand_str {
        "X" => Hand::Rock,
        "Y" => Hand::Paper,
        "Z" => Hand::Scissors,
        _ => panic!("Error"),
    };

    return Ok((own_hand, other_hand));
}

fn string_to_hand_result(input: String) -> Result<(MatchResult, Hand), Box<dyn Error>> {
    let mut hands_string = input.split_whitespace();
    let other_hand_str = hands_string.next().unwrap();
    let match_result_str = hands_string.next().unwrap();

    let other_hand = match other_hand_str {
        "A" => Hand::Rock,
        "B" => Hand::Paper,
        "C" => Hand::Scissors,
        _ => panic!("Error"),
    };

    let match_result = match match_result_str {
        "X" => MatchResult::Lose,
        "Y" => MatchResult::Draw,
        "Z" => MatchResult::Win,
        _ => panic!("Error"),
    };

    return Ok((match_result, other_hand));
}

fn get_result(own_hand: Hand, other_hand: Hand) -> i32 {
    let other_hand_owned = other_hand.to_owned();
    let match_result = own_hand.fight(other_hand_owned);

    own_hand.value() + match_result.value()
}

fn get_result_p2(result: MatchResult, other_hand: Hand) -> i32 {
    let other_hand_owned = other_hand.to_owned();
    let own_hand = result.interpolate(other_hand_owned);

    own_hand.value() + result.value()
}

pub fn solve() -> Result<aoc_core::Solution, Box<dyn Error>> {
    let input = fs::read_to_string("data/day_2/2.txt")?;

    let string_vec = string_to_vector_line_break(input)?;

    let mut all_hands: Vec<(Hand, Hand)> = vec![];
    for hands in string_vec.clone() {
        all_hands.push(string_to_hands(hands)?)
    }

    let mut total_p1: i32 = 0;
    for hand in all_hands {
        total_p1 += get_result(hand.0, hand.1);
    }

    let mut all_result_hand = vec![];
    for result_hands in string_vec.clone() {
        all_result_hand.push(string_to_hand_result(result_hands)?);
    }

    let mut total_p2: i32 = 0;
    for result_hand in all_result_hand {
        total_p2 += get_result_p2(result_hand.0, result_hand.1);
    }

    Ok(aoc_core::Solution {
        part_1: total_p1,
        part_2: total_p2,
    })
}
