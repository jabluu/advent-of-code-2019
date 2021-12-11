// https://adventofcode.com/2019/day/3

pub fn part1() -> i32 {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct Point(i32, i32);

    struct Wire {
        vertices: Vec<Point>,
    }

    impl From<&str> for Wire {
        fn from(line: &str) -> Self {
            let vertices = line.split(',').map(|item| {
                    let direction = &item[..1];

                    let magnitude = &item[1..];
                    let magnitude = magnitude.parse::<i32>().unwrap();

                    let segment = match direction {
                        "L" => (-magnitude, 0),
                        "R" => (magnitude, 0),
                        "D" => (0, -magnitude),
                        "U" => (0, magnitude),
                        _ => panic!("unrecognized direction {}", direction),
                    };

                    segment
                })
                .scan(
                    Point(0, 0),
                    |state, item| {
                        *state = Point(
                            state.0 + item.0,
                            state.1 + item.1,
                        );

                        Some(*state)
                    }
                )
                .collect::<Vec<Point>>();

            Wire { vertices }
        }
    }

    fn find_intersection(
        (a, b): (&Point, &Point),
        (p, q): (&Point, &Point)
    ) -> Option<Point> {
        #[derive(Clone, Copy, PartialEq)]
        enum Ori { HORI, VERT }

        let ab_ori = match (a.0 == b.0, a.1 == b.1) {
            (true, false) => Ori::VERT,
            (false, true) => Ori::HORI,
            _ => panic!("unrecognized orientation {:?}-{:?}", a, b)
        };

        let pq_ori = match (p.0 == q.0, p.1 == q.1) {
            (true, false) => Ori::VERT,
            (false, true) => Ori::HORI,
            _ => panic!("unrecognized orientation {:?}-{:?}", p, q)
        };

        // wires cannot cross if they are parallel
        if ab_ori == pq_ori {
            return None
        }

        // ensure a-b is the horizontal wire and p-q is the vertical wire
        let (a, b, p, q) = match (ab_ori, pq_ori) {
            (Ori::HORI, Ori::VERT) =>  (a, b, p, q),
            (Ori::VERT, Ori::HORI) =>  (p, q, a, b),
            _ => unreachable!(),
        };

        // ensure a is left of b and p is below q
        let (a, b) = if a.0 > b.0 { (b, a) } else { (a, b) };
        let (p, q) = if p.1 > q.1 { (q, p) } else { (p, q) };

        // check that a-b and p-q intersect
        if (a.0 <= p.0 && p.0 <= b.0) && (p.1 <= a.1 && a.1 <= q.1) {
            Some(Point(p.0, a.1))
        } else {
            None
        }
    }

    fn find_crossings(wire1: &Wire, wire2: &Wire) -> Vec<Point> {
        let (a, b) = {
            let n = wire1.vertices.len();
            let a = &wire1.vertices[0..n-1];
            let b = &wire1.vertices[1..n];
            (a, b)
        };

        let (p, q) = {
            let n = wire2.vertices.len();
            let p = &wire2.vertices[0..n-1];
            let q = &wire2.vertices[1..n];
            (p, q)
        };

        let mut result = Vec::new();

        for (a, b) in a.iter().zip(b.iter()) {
            for (p, q) in p.iter().zip(q.iter()) {
                if let Some(point) = find_intersection((a, b), (p, q)) {
                    result.push(point);
                };
            }
        }

        result
    }

    fn manhattan_distance_between(a: &Point, b: &Point) -> i32 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }

    assert_eq!(
        6,
        find_crossings(
            &Wire::from("R8,U5,L5,D3"),
            &Wire::from("U7,R6,D4,L4")
        ).into_iter()
            .map(|point| manhattan_distance_between(&Point(0, 0), &point))
            .min().unwrap()
    );

    assert_eq!(
        159,
        find_crossings(
            &Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &Wire::from("U62,R66,U55,R34,D71,R55,D58,R83")
        ).into_iter()
            .map(|point| manhattan_distance_between(&Point(0, 0), &point))
            .min().unwrap()
    );

    assert_eq!(
        135,
        find_crossings(
            &Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ).into_iter()
            .map(|point| manhattan_distance_between(&Point(0, 0), &point))
            .min().unwrap()
    );

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let handle = File::open("input/day03/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let wires = buffer.lines()
        .map(|line| Wire::from(line.unwrap().as_str()))
        .collect::<Vec<Wire>>();

    find_crossings(
        &wires[0],
        &wires[1],
    ).into_iter()
        .map(|point| manhattan_distance_between(&Point(0, 0), &point))
        .min().unwrap()
}

pub fn part2() -> i32 {
    0
}