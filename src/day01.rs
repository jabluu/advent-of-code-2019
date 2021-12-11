// https://adventofcode.com/2019/day/1

pub fn part1() -> i32 {
    struct Module {
        mass: i32,
    }

    impl Module {
        fn new(mass: i32) -> Self { Self { mass } }
        fn fuel_required(&self) -> i32 { self.mass / 3 - 2 }
    }

    assert_eq!(2, Module::new(12).fuel_required());
    assert_eq!(2, Module::new(14).fuel_required());
    assert_eq!(654, Module::new(1969).fuel_required());
    assert_eq!(33583, Module::new(100756).fuel_required());

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let handle = File::open("input/day01/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let mut total_fuel = 0;

    for line in buffer.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        let module = Module::new(mass);
        total_fuel += module.fuel_required();
    }

    total_fuel
}

pub fn part2() -> i32 {
    struct Module {
        mass: i32,
    }

    impl Module {
        fn new(mass: i32) -> Self { Self { mass } }

        fn fuel_required_for_mass(mass: i32) -> i32 {
            std::cmp::max(mass / 3 - 2, 0)
        }

        fn fuel_required(&self) -> i32 {
            let mut total_fuel = 0;
            let mut unaccounted_mass = self.mass;

            while unaccounted_mass > 0 {
                let additional_fuel =
                    Self::fuel_required_for_mass(unaccounted_mass);

                total_fuel += additional_fuel;
                unaccounted_mass = additional_fuel;
            }

            total_fuel
        }
    }

    assert_eq!(2, Module::new(12).fuel_required());
    assert_eq!(2, Module::new(14).fuel_required());
    assert_eq!(966, Module::new(1969).fuel_required());
    assert_eq!(50346, Module::new(100756).fuel_required());

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let handle = File::open("input/day01/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let mut total_fuel = 0;

    for line in buffer.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        let module = Module::new(mass);
        total_fuel += module.fuel_required();
    }

    total_fuel
}