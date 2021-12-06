mod day01 {
    // The Elves quickly load you into a spacecraft and prepare to launch.
    //
    // At the first Go / No Go poll, every Elf is Go until the Fuel
    // Counter-Upper. They haven't determined the amount of fuel required yet.
    //
    // Fuel required to launch a given module is based on its mass.
    // Specifically, to find the fuel required for a module, take its mass,
    // divide by three, round down, and subtract 2.
    //
    // For example:
    //
    // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to
    // get 2. For a mass of 14, dividing by 3 and rounding down still yields 4,
    // so the fuel required is also 2. For a mass of 1969, the fuel required is
    // 654. For a mass of 100756, the fuel required is 33583. The Fuel
    // Counter-Upper needs to know the total fuel requirement. To find it,
    // individually calculate the fuel needed for the mass of each module (your
    // puzzle input), then add together all the fuel values.
    //
    // What is the sum of the fuel requirements for all of the modules on your
    // spacecraft?

    struct Module {
        mass: u32,
    }

    impl Module {
        fn new(mass: u32) -> Self { Self { mass } }
        fn fuel(&self) -> u32 { self.mass / 3 - 2 }
    }

    pub fn part1() -> u32 {
        assert_eq!(2, Module::new(12).fuel());
        assert_eq!(2, Module::new(14).fuel());
        assert_eq!(654, Module::new(1969).fuel());
        assert_eq!(33583, Module::new(100756).fuel());

        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let handle = File::open("input/day01/part1.txt").unwrap();
        let buffer = BufReader::new(handle);

        let mut total_fuel = 0;

        for line in buffer.lines() {
            let mass = line.unwrap().parse::<u32>().unwrap();
            let module = Module::new(mass);
            total_fuel += module.fuel();
        }

        total_fuel
    }
}

fn main() {
    let result = day01::part1();
    println!("{:?}", result);
    assert_eq!(3427972, result);
}
