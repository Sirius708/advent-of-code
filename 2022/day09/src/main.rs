use std::cmp::{max, min};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Grid {
    tail_positions: HashSet<Position>,
    head: Position,
    knots: Vec<Position>,
}

impl Grid {
    pub fn new(knots: usize) -> Self {
        let mut result = Self {
            tail_positions: HashSet::new(),
            head: Default::default(),
            knots: vec![Default::default(); knots],
        };
        result.tail_positions.insert(Default::default());
        result
    }

    pub fn apply_move(&mut self, direction: Direction, steps: u32) {
        if steps == 0 {
            return;
        }
        match direction {
            Direction::Left => {
                for _ in 0..steps {
                    self.head.x -= 1;
                    self.adjust_tails();
                }
            }
            Direction::Right => {
                for _ in 0..steps {
                    self.head.x += 1;
                    self.adjust_tails();
                }
            }
            Direction::Up => {
                for _ in 0..steps {
                    self.head.y += 1;
                    self.adjust_tails();
                }
            }
            Direction::Down => {
                for _ in 0..steps {
                    self.head.y -= 1;
                    self.adjust_tails();
                }
            }
        }
    }

    fn adjust_tails(&mut self) {
        let mut target = self.head;
        for knot in self.knots.iter_mut() {
            *knot = Self::adjust_tail(target, *knot);
            target = *knot;
        }
        self.tail_positions.insert(target);
    }

    fn adjust_tail(target: Position, mut knot: Position) -> Position {
        if util::chebyshev_distance_2d(target.x as _, target.y as _, knot.x as _, knot.y as _) <= 1
        {
            return knot;
        }
        if target.x == knot.x {
            knot.y += (target.y - knot.y).signum();
        } else if target.y == knot.y {
            knot.x += (target.x - knot.x).signum();
        } else {
            knot.y += (target.y - knot.y).signum();
            knot.x += (target.x - knot.x).signum();
        }
        knot
    }

    pub fn print_positions(&self) {
        let (min_x, min_y, max_x, max_y) = self
            .tail_positions
            .iter()
            .chain(self.knots.iter())
            .fold((0, 0, 0, 0), |(min_x, min_y, max_x, max_y), point| {
                (
                    min(min_x, point.x),
                    min(min_y, point.y),
                    max(max_x, point.x),
                    max(max_y, point.y),
                )
            });
        let min_x = min(min_x, self.head.x) - 1;
        let min_y = min(min_y, self.head.y) - 1;
        let max_x = max(max_x, self.head.x) + 1;
        let max_y = max(max_y, self.head.y) + 1;

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if self.head.y == y && self.head.x == x {
                    print!("H");
                } else if self.knots.contains(&Position { x, y }) {
                    let index = self
                        .knots
                        .iter()
                        .position(|p| p.x == x && p.y == y)
                        .unwrap();
                    print!("{}", index + 1);
                } else if x == 0 && y == 0 {
                    print!("s");
                } else if self.tail_positions.contains(&Position { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let input_lines = util::get_input_lines();
    let moves: Vec<(Direction, u32)> = input_lines
        .into_iter()
        .map(|line| {
            let (direction, step_count) = line.split_once(' ').unwrap();
            (direction.parse().unwrap(), step_count.parse().unwrap())
        })
        .collect();
    let mut grid = Grid::new(9);
    for (direction, steps) in moves {
        grid.apply_move(direction, steps);
        // grid.print_positions();
        // println!();
    }
    println!("Positions touched by tail: {}", grid.tail_positions.len());
}
