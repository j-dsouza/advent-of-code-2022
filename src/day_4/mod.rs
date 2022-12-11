use std::{error::Error, fs, ops::RangeInclusive};

use crate::aoc_core::Solution;

fn parse_pt1(input: String) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let input_lines = input.lines();

    let mut input_vec: Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> = vec![];
    for line in input_lines {
        let range_strs: Vec<&str> = line.split(",").collect();
        let range_1: &str = range_strs[0];
        let range_2: &str = range_strs[1];
        let range_1_split: Vec<&str> = range_1.split("-").collect();
        let range_2_split: Vec<&str> = range_2.split("-").collect();
        input_vec.push((
            RangeInclusive::new(
                range_1_split[0].parse::<i32>().unwrap(),
                range_1_split[1].parse::<i32>().unwrap(),
            ),
            RangeInclusive::new(
                range_2_split[0].parse::<i32>().unwrap(),
                range_2_split[1].parse::<i32>().unwrap(),
            ),
        ))
    }

    input_vec
}

pub fn solve() -> Result<Solution, Box<dyn Error>> {
    let input = fs::read_to_string("data/day_4/4.txt")?;
    let input_pt1_parsed = parse_pt1(input.clone());

    let mut sum_pt_1 = 0;
    for (input_range_1, input_range_2) in input_pt1_parsed {
        if (input_range_1.contains(&input_range_2.start())
            && input_range_1.contains(&input_range_2.end()))
            || (input_range_2.contains(&input_range_1.start())
                && input_range_2.contains(&input_range_1.end()))
        {
            sum_pt_1 += 1
        }
    }

    let input_pt1_parsed = parse_pt1(input.clone());
    let mut sum_pt_2 = 0;
    for (input_range_1, input_range_2) in input_pt1_parsed {
        if (input_range_1.contains(&input_range_2.start())
            || input_range_1.contains(&input_range_2.end()))
            || (input_range_2.contains(&input_range_1.start())
                || input_range_2.contains(&input_range_1.end()))
        {
            sum_pt_2 += 1
        }
    }

    Ok(Solution {
        part_1: sum_pt_1,
        part_2: sum_pt_2,
    })
}
