// https://adventofcode.com/2019/day/7

use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use crate::intcode::Computer;

pub fn part1() -> i32 {
    let handle = File::open("input/day07/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let program = buffer.lines()
        .map(|line| {
            line.unwrap().split(',').map(|item| {
                item.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
        }).next().unwrap();

    let num_phase_settings = 5;
    let phase_settings = 0..num_phase_settings;

    let mut max_signal = 0;

    for phases in phase_settings.permutations(num_phase_settings) {
        let mut opamps = [
            Computer::new(&program.clone()),
            Computer::new(&program.clone()),
            Computer::new(&program.clone()),
            Computer::new(&program.clone()),
            Computer::new(&program.clone()),
        ];

        let mut signal = 0;

        for (phase, mut opamp) in phases.iter().zip(opamps.iter_mut()) {
            opamp.input_buffer.push_back(*phase as i32);
            opamp.input_buffer.push_back(signal);

            opamp.execute_program();

            signal = opamp.output_buffer.pop_back().unwrap();
        }

        max_signal = std::cmp::max(signal, max_signal);
    }

    max_signal
}

pub fn part2() -> i32 {
    0
}