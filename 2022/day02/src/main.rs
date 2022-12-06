use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl Shape {
    fn beats(self, other: Self) -> bool {
        match (self, other) {
            (Shape::Rock, Shape::Paper) => false,
            (Shape::Rock, Shape::Scissor) => true,
            (Shape::Paper, Shape::Rock) => true,
            (Shape::Paper, Shape::Scissor) => false,
            (Shape::Scissor, Shape::Rock) => false,
            (Shape::Scissor, Shape::Paper) => true,
            _ => false,
        }
    }
}

fn main() {
    let input_lines = util::get_input_lines();

    let mut lookup = HashMap::new();
    lookup.insert('A', Shape::Rock);
    lookup.insert('B', Shape::Paper);
    lookup.insert('C', Shape::Scissor);
    lookup.insert('X', Shape::Rock);
    lookup.insert('Y', Shape::Paper);
    lookup.insert('Z', Shape::Scissor);

    let score_one: u32 = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let left = left.chars().next().unwrap();
            let right = right.chars().next().unwrap();
            (lookup[&left], lookup[&right])
        })
        .map(|(left, right)| {
            if left == right {
                3 + right as u32
            } else if right.beats(left) {
                6 + right as u32
            } else {
                right as u32
            }
        })
        .sum();
    println!("Total score 1: {score_one}");

    let score_two: u32 = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let left = left.chars().next().unwrap();
            let left = lookup[&left];
            let right = right.chars().next().unwrap();
            (
                left,
                match right {
                    'X' => match left {
                        Shape::Rock => Shape::Scissor,
                        Shape::Paper => Shape::Rock,
                        Shape::Scissor => Shape::Paper,
                    },
                    'Y' => left,
                    'Z' => match left {
                        Shape::Rock => Shape::Paper,
                        Shape::Paper => Shape::Scissor,
                        Shape::Scissor => Shape::Rock,
                    },
                    _ => unreachable!(),
                },
            )
        })
        .map(|(left, right)| {
            if left == right {
                3 + right as u32
            } else if right.beats(left) {
                6 + right as u32
            } else {
                right as u32
            }
        })
        .sum();
    println!("Total score 2: {score_two}");
}
