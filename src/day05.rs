// https://adventofcode.com/2019/day/4

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::intcode::Computer;

pub fn part1() -> i32 {
    let handle = File::open("input/day05/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let computers = buffer.lines()
        .map(|line| {
            line.unwrap().split(',').map(|item| {
                item.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
        })
        .map(|program| {
            Computer::new(&program)
        })
        .collect::<Vec<Computer>>();

    let mut computer = computers[0].clone();

    computer.execute_program();

    computer.output
}

pub fn part2() -> i32 {
    0
}