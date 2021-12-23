// https://adventofcode.com/2019/day/6

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Node<T> {
    data: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, parent: None, children: Vec::new() }
    }
}

fn load() -> (Vec<Node<String>>, HashMap<String, usize>) {
    let handle = File::open("input/day06/input.txt").unwrap();
    let line_buffer = BufReader::new(handle);

    let mut node_buffer = Vec::new();
    let mut node_mapping = HashMap::new();

    for line in line_buffer.lines() {
        let line = line.unwrap();
        let mut parts = line.split(')');

        let pkey = parts.next().unwrap().to_string();
        let ckey = parts.next().unwrap().to_string();

        let pindex = *node_mapping
            .entry(pkey)
            .or_insert_with_key(|key| {
                let node = Node::new(key.clone());
                let index = node_buffer.len();
                node_buffer.push(node);
                index
            });

        let cindex = *node_mapping
            .entry(ckey)
            .or_insert_with_key(|key| {
                let node = Node::new(key.clone());
                let index = node_buffer.len();
                node_buffer.push(node);
                index
            });

        node_buffer[cindex].parent = Some(pindex);
        node_buffer[pindex].children.push(cindex);
    }

    (node_buffer, node_mapping)
}

//             /-----G(2)----H(3)      /-----J(5)----K(6)----L(7)
//            /        |       |      /        |       |       |
// COM(0)----B(1)----C(2)----D(3)----E(4)----F(5)      |       |
//     |       |       |      \|       |       |       |       |
//     |       |       |       \-----I(4)      |       |       |
//     |       |       |       |       |       |       |       |
//   0(1)   +1(1)   +2(2)   +3(2)   +4(2)   +5(2)   +6(1)   +7(1)
//     0       1       4       6       8      10       6       7
//---------------------------------------------------------------
//     0       1       5      11      19      29      35      42

pub fn part1() -> i32 {
    let (node_buffer, node_mapping) = load();

    let root_node = node_buffer.iter()
        .find(|node| node.parent.is_none())
        .unwrap();

    let mut queue = VecDeque::from([(0, root_node)]);
    let mut total = 0;

    while let Some((level, node)) = queue.pop_front() {
        total += level;
        let items = node.children.iter()
            .map(|index| (level+1, &node_buffer[*index]));
        queue.extend(items);
    }

    total
}

pub fn part2() -> i32 {
    let (node_buffer, node_mapping) = load();

    let node_you = &node_buffer[node_mapping["YOU"]];
    let node_san = &node_buffer[node_mapping["SAN"]];

    let mut path_you = vec![node_you];
    let mut path_san = vec![node_san];

    while let Some(index) = path_you.last().unwrap().parent {
        path_you.push(&node_buffer[index]);
    }

    while let Some(index) = path_san.last().unwrap().parent {
        path_san.push(&node_buffer[index]);
    }

    let mut iter_you = path_you.iter().rev();
    let mut iter_san = path_san.iter().rev();

    loop {
        let node_you = iter_you.next().unwrap();
        let node_san = iter_san.next().unwrap();

        if node_you.data != node_san.data {
            break;
        }
    }

    let num_transfers = iter_you.count() + iter_san.count();

    num_transfers as i32
}