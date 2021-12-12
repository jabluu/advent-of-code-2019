// https://adventofcode.com/2019/day/4

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct PasswordCriteria {
    min_val: i32,
    max_val: i32,
}

impl PasswordCriteria {
    fn new(min_val: i32, max_val: i32,) -> Self {
        PasswordCriteria { min_val, max_val }
    }

    fn rule1(&self, password: &str) -> bool {
        let result = password.len() == 6;
        result
    }

    fn rule2(&self, password: &str) -> bool {
        let value = password.parse::<i32>().unwrap();
        let result = self.min_val < value && value < self.max_val;
        result
    }

    fn rule3_part1(&self, password: &str) -> bool {
        let n = password.len();
        let a = &password[0..n-1];
        let b = &password[1..n];

        let result = a.chars().zip(b.chars())
            .find(|(x, y)| {
                let x = x.to_digit(10).unwrap();
                let y = y.to_digit(10).unwrap();
                x == y
            })
            .is_some();

        result
    }

    fn rule3_part2(&self, password: &str) -> bool {
        let n = password.len();
        let a = &password[0..n-1];
        let b = &password[1..n];

        #[derive(Debug)]
        struct State {
            streak: i32,
            total_streaks: i32,
            valid_streaks: i32,
        }

        let state = a.chars().zip(b.chars())
            .scan(
                State {
                    streak: 0,
                    total_streaks: 0,
                    valid_streaks: 0,
                },
                |state, (x, y)| {
                    let State {
                        streak,
                        total_streaks,
                        valid_streaks,
                    } = state;

                    let x = x.to_digit(10).unwrap();
                    let y = y.to_digit(10).unwrap();

                    if x == y {
                        *streak += 1;

                        if *streak == 1 {
                            *total_streaks += 1;
                            *valid_streaks += 1;
                        } else if *streak == 2 {
                            *valid_streaks -= 1;
                        }
                    } else {
                        *streak = 0;
                    }

                    Some(
                        State {
                            streak: *streak,
                            total_streaks: *total_streaks,
                            valid_streaks: *valid_streaks,
                        }
                    )
                }
            )
            .last().unwrap();

        let result = state.valid_streaks > 0;

        result
    }

    fn rule4(&self, password: &str) -> bool {
        let n = password.len();
        let a = &password[0..n-1];
        let b = &password[1..n];

        let result = a.chars().zip(b.chars())
            .find(|(x, y)| {
                let x = x.to_digit(10).unwrap();
                let y = y.to_digit(10).unwrap();
                x > y
            })
            .is_none();

        result
    }

    fn check_part1(&self, password: i32) -> bool {
        let password_as_str = format!("{}", password);

        let result = self.rule1(&password_as_str)
            && self.rule2(&password_as_str)
            && self.rule4(&password_as_str)
            && self.rule3_part1(&password_as_str);

        result
    }

    fn check_part2(&self, password: i32) -> bool {
        let password_as_str = format!("{}", password);

        let result = self.rule1(&password_as_str)
            && self.rule2(&password_as_str)
            && self.rule4(&password_as_str)
            && self.rule3_part2(&password_as_str);

        result
    }
}

pub fn part1() -> i32 {
    assert_eq!(
        true,
        PasswordCriteria::new(0, 999999).check_part1(111111)
    );

    assert_eq!(
        false,
        PasswordCriteria::new(0, 999999).check_part1(223450)
    );

    assert_eq!(
        false,
        PasswordCriteria::new(0, 999999).check_part1(123789)
    );

    let handle = File::open("input/day04/input.txt").unwrap();
    let mut buffer = BufReader::new(handle);

    let mut line = String::new();
    buffer.read_line(&mut line).unwrap();

    let items = line.split("-")
        .map(|item| item.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let (min_val, max_val) = (items[0], items[1]);

    let criteria = PasswordCriteria::new(min_val, max_val);

    let result = (min_val..max_val)
        .filter(|&password| criteria.check_part1(password))
        .count();

    result as i32
}

pub fn part2() -> i32 {
    assert_eq!(
        true,
        PasswordCriteria::new(0, 999999).check_part2(112233)
    );

    assert_eq!(
        false,
        PasswordCriteria::new(0, 999999).check_part2(123444)
    );

    assert_eq!(
        true,
        PasswordCriteria::new(0, 999999).check_part2(111122)
    );

    let handle = File::open("input/day04/input.txt").unwrap();
    let mut buffer = BufReader::new(handle);

    let mut line = String::new();
    buffer.read_line(&mut line).unwrap();

    let items = line.split("-")
        .map(|item| item.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let (min_val, max_val) = (items[0], items[1]);

    let criteria = PasswordCriteria::new(min_val, max_val);

    let result = (min_val..max_val)
        .filter(|&password| criteria.check_part2(password))
        .count();

    result as i32
}