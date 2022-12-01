use std::collections::BinaryHeap;
use std::{fs, error::Error};
use std::vec::Vec;
use crate::aoc_core;

fn string_to_vector_line_break(input: String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let split = input.lines();
    let mut output: Vec<Vec<i32>> = vec![];
    let mut temp: Vec<i32> = vec![];
    for line in split {
        if line == "" {
            let group = temp.clone();
            output.push(group);
            temp = vec![]
        } else {
            let int_line: i32 = line.parse::<i32>()?;
            temp.push(int_line)
        }
    }
    return Ok(output)
}


pub fn solve() -> Result<aoc_core::Solution, Box<dyn Error>> {
    let input = fs::read_to_string("data/day_1/1.txt")?;
    let input_vec = string_to_vector_line_break(input)?;

    let mut max_sum = 0;
    for elf in input_vec.iter() {
        let elf_sum = elf.iter().sum();
        if elf_sum > max_sum {
            max_sum = elf_sum
        }
    }

    let mut heap:BinaryHeap<i32> = BinaryHeap::new();
    for elf in input_vec.iter() {
        let elf_sum = elf.iter().sum();
        heap.push(elf_sum)
    }
    let mut sum = 0;
    sum += heap.pop().unwrap();
    sum += heap.pop().unwrap();
    sum += heap.pop().unwrap();

    Ok(aoc_core::Solution{part_1:max_sum, part_2: sum})

}
