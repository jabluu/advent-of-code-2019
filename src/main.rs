mod intcode;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let result = day01::part1();
    println!("Day 1 Part 1: {:?}", result);
    assert_eq!(3427972, result);

    let result = day01::part2();
    println!("      Part 2: {:?}", result);
    assert_eq!(5139078, result);

    let result = day02::part1();
    println!("Day 2 Part 1: {:?}", result);
    assert_eq!(7594646, result);

    let result = day02::part2();
    println!("      Part 2: {:?}", result);
    assert_eq!(3376, result);

    let result = day03::part1();
    println!("Day 3 Part 1: {:?}", result);
    assert_eq!(529, result);

    let result = day03::part2();
    println!("      Part 2: {:?}", result);
    assert_eq!(20386, result);

    let result = day04::part1();
    println!("Day 4 Part 1: {:?}", result);
    assert_eq!(945, result);

    let result = day04::part2();
    println!("      Part 2: {:?}", result);
    assert_eq!(617, result);
}
