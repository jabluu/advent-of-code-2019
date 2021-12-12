// https://adventofcode.com/2019/day/2

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::intcode::Computer;

pub fn part1() -> i32 {
    assert_eq!(
        [2,0,0,0,99].to_vec(),
        Computer::new(&[1,0,0,0,99]).execute_program().memory
    );

    assert_eq!(
        [2,3,0,6,99].to_vec(),
        Computer::new(&[2,3,0,3,99]).execute_program().memory
    );

    assert_eq!(
        [2,4,4,5,99,9801].to_vec(),
        Computer::new(&[2,4,4,5,99,0]).execute_program().memory
    );

    assert_eq!(
        [30,1,1,4,2,5,6,0,99].to_vec(),
        Computer::new(&[1,1,1,4,99,5,6,0,99]).execute_program().memory
    );

    let handle = File::open("input/day02/input.txt").unwrap();
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

    computer.with_inputs(12, 2).execute_program().memory[0]
}

pub fn part2() -> i32 {
    assert_eq!(
        [2,0,0,0,99].to_vec(),
        Computer::new(&[1,0,0,0,99]).execute_program().memory
    );

    assert_eq!(
        [2,3,0,6,99].to_vec(),
        Computer::new(&[2,3,0,3,99]).execute_program().memory
    );

    assert_eq!(
        [2,4,4,5,99,9801].to_vec(),
        Computer::new(&[2,4,4,5,99,0]).execute_program().memory
    );

    assert_eq!(
        [30,1,1,4,2,5,6,0,99].to_vec(),
        Computer::new(&[1,1,1,4,99,5,6,0,99]).execute_program().memory
    );

    let handle = File::open("input/day02/input.txt").unwrap();
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

    let computer = computers[0].clone();

    for noun in 0..99 {
        for verb in 0..99 {
            let result = computer.clone()
                .with_inputs(noun, verb)
                .execute_program()
                .memory[0];

            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}