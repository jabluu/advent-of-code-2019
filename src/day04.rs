// https://adventofcode.com/2019/day/4

use std::fs::File;
use std::io::{BufRead, BufReader};

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

    fn rule3(&self, password: &str) -> bool {
        let n = password.len();
        let a = &password[0..n-1];
        let b = &password[1..n];

        let result = a.chars().zip(b.chars()).find(|(x, y)| x == y).is_some();
        result
    }

    fn rule4(&self, password: &str) -> bool {
        let n = password.len();
        let a = &password[0..n-1];
        let b = &password[1..n];

        let result = a.chars().zip(b.chars()).find(|(x, y)| x > y).is_none();
        result
    }

    fn check(&self, password: i32) -> bool {
        let password_as_str =format!("{}", password);

        let result = self.rule1(&password_as_str)
            && self.rule2(&password_as_str)
            && self.rule3(&password_as_str)
            && self.rule4(&password_as_str);

        result
    }
}

pub fn part1() -> i32 {
    assert_eq!(
        true,
        PasswordCriteria::new(0, 999999).check(111111)
    );

    assert_eq!(
        false,
        PasswordCriteria::new(0, 999999).check(223450)
    );

    assert_eq!(
        false,
        PasswordCriteria::new(0, 999999).check(123789)
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
        .filter(|&password| criteria.check(password))
        .count();

    result as i32
}

pub fn part2() -> i32 {
    assert_eq!(
        true,
        PasswordCriteria::new(0, 999999).check(112233)
    );

    assert_eq!(
        false,
        PasswordCriteria::new(0, 999999).check(123444)
    );

    assert_eq!(
        false,
        PasswordCriteria::new(0, 999999).check(111122)
    );

    let handle = File::open("input/day04/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    0
}