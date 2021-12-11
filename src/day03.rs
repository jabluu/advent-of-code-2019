// https://adventofcode.com/2019/day/3

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point(i32, i32);

struct Vertex {
    position: Point,
    distance: i32,
}

struct Wire {
    vertices: Vec<Vertex>,
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
                (Point(0, 0), 0),
                |(position, distance), item| {
                    *position = Point(
                        position.0 + item.0,
                        position.1 + item.1,
                    );

                    *distance += item.0.abs() + item.1.abs();

                    let vertex = Vertex {
                        position: *position,
                        distance: *distance,
                    };

                    Some(vertex)
                }
            )
            .collect();

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

fn find_crossings(wire1: &Wire, wire2: &Wire) -> Vec<(usize, usize, Point)> {
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

    for (idx1, (a, b)) in a.iter().zip(b.iter()).enumerate() {
        for (idx2, (p, q)) in p.iter().zip(q.iter()).enumerate() {
            if let Some(point) = find_intersection(
                (&a.position, &b.position),
                (&p.position, &q.position)
            ) {
                result.push((idx1, idx2, point));
            };
        }
    }

    result
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

pub fn part1() -> i32 {
    let o = Point(0, 0);

    assert_eq!(
        6,
        find_crossings(
            &Wire::from("R8,U5,L5,D3"),
            &Wire::from("U7,R6,D4,L4")
        ).into_iter()
            .map(|(_, _, p)| manhattan_distance(&o, &p))
            .min().unwrap()
    );

    assert_eq!(
        159,
        find_crossings(
            &Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &Wire::from("U62,R66,U55,R34,D71,R55,D58,R83")
        ).into_iter()
            .map(|(_, _, p)| manhattan_distance(&o, &p))
            .min().unwrap()
    );

    assert_eq!(
        135,
        find_crossings(
            &Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ).into_iter()
            .map(|(_, _, p)| manhattan_distance(&o, &p))
            .min().unwrap()
    );

    let handle = File::open("input/day03/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let wires = buffer.lines()
        .map(|line| Wire::from(line.unwrap().as_str()))
        .collect::<Vec<Wire>>();

    let (wire1, wire2) = (&wires[0], &wires[1]);

    find_crossings(
        &wire1,
        &wire2,
    ).into_iter()
        .map(|(_, _, p)| manhattan_distance(&o, &p))
        .min().unwrap()
}

pub fn part2() -> i32 {
    fn f(wire1: &Wire, wire2: &Wire) -> i32 {
        find_crossings(
            &wire1,
            &wire2,
        ).into_iter()
            .map(|(idx1, idx2, p)| {
                let vertex_a = &wire1.vertices[idx1];
                let vertex_b = &wire2.vertices[idx2];

                let dist_oa = vertex_a.distance;
                let dist_ob = vertex_b.distance;

                let dist_ap = manhattan_distance(&vertex_a.position, &p);
                let dist_bp = manhattan_distance(&vertex_b.position, &p);

                dist_oa + dist_ap + dist_ob + dist_bp
            })
            .min().unwrap()
    }

    let wire1 = Wire::from("R8,U5,L5,D3");
    let wire2 = Wire::from("U7,R6,D4,L4");
    assert_eq!(30, f(&wire1, &wire2));

    let wire1 = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(610, f(&wire1, &wire2));

    let wire1 = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(410, f(&wire1, &wire2));

    let handle = File::open("input/day03/input.txt").unwrap();
    let buffer = BufReader::new(handle);

    let wires = buffer.lines()
        .map(|line| Wire::from(line.unwrap().as_str()))
        .collect::<Vec<Wire>>();

    let (wire1, wire2) = (&wires[0], &wires[1]);

    // {
    //     use svg::Document;
    //     use svg::node::element::Path;
    //     use svg::node::element::path::Data;
    //
    //     let mut data1 = Data::new().move_to((0, 0));
    //     let mut data2 = Data::new().move_to((0, 0));
    //
    //     for vert in wire1.vertices.iter() {
    //         let pos = vert.position;
    //         data1 = data1.clone().line_to((pos.0, pos.1));
    //     }
    //
    //     for vert in wire2.vertices.iter() {
    //         let pos = vert.position;
    //         data2 = data2.clone().line_to((pos.0, pos.1));
    //     }
    //
    //     let path1 = Path::new()
    //         .set("fill", "none")
    //         .set("stroke", "black")
    //         .set("stroke-width", 5)
    //         .set("d", data1);
    //
    //     let path2 = Path::new()
    //         .set("fill", "none")
    //         .set("stroke", "black")
    //         .set("stroke-width", 5)
    //         .set("d", data2);
    //
    //     let document = Document::new()
    //         .set("viewBox", (-10000, -10000, 10000, 10000))
    //         .add(path1)
    //         .add(path2);
    //
    //     svg::save("day03part2.svg", &document).unwrap();
    // }

    f(wire1, wire2)
}