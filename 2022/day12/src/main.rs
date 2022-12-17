use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;

use util::{find_shortest_distance, manhatten_distance_2d};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Edge<T> {
    pub value: T,
    pub cost: u32,
}

impl<T> PartialOrd for Edge<T>
where
    T: Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Edge<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn main() {
    let input_lines = util::get_input_lines();
    let grid = input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => b'z' - b'a',
                    _ => c as u8 - b'a',
                } as i8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = grid.len();
    let width = grid[0].len();

    let start = input_lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .position(|c| c == 'S')
                .map(|x| (x as i32, y as i32))
        })
        .next()
        .unwrap()
        .into();
    let end = input_lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .position(|c| c == 'E')
                .map(|x| (x as i32, y as i32))
        })
        .next()
        .unwrap()
        .into();

    let distance =
        find_shortest_distance(start, end, get_successors(&grid, width, height), |point| {
            manhatten_distance_2d(point.x, point.y, end.x, end.y)
        })
        .unwrap();
    println!("Distance to target: {}", distance);

    let (closest_start, closest_distance) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(move |(x, _)| Point {
                    x: x as i32,
                    y: y as i32,
                })
        })
        .filter_map(|start| {
            find_shortest_distance(start, end, get_successors(&grid, width, height), |point| {
                manhatten_distance_2d(point.x, point.y, end.x, end.y)
            })
            .map(|distance| (start, distance))
        })
        .min_by_key(|(_, distance)| *distance)
        .unwrap();
    println!(
        "Shortest start to target: {:?} = {}",
        closest_start, closest_distance
    );
}

fn get_neighbor_points(grid: &[Vec<i8>], point: Point, width: usize, height: usize) -> Vec<Point> {
    let mut neighbours = Vec::with_capacity(4);
    if point.x > 0 {
        let neighbor = Point {
            x: point.x - 1,
            y: point.y,
        };
        if get_height_difference(grid, point, neighbor) <= 1 {
            neighbours.push(neighbor);
        }
    }
    if point.x < (width - 1) as i32 {
        let neighbor = Point {
            x: point.x + 1,
            y: point.y,
        };
        if get_height_difference(grid, point, neighbor) <= 1 {
            neighbours.push(neighbor);
        }
    }
    if point.y > 0 {
        let neighbor = Point {
            x: point.x,
            y: point.y - 1,
        };
        if get_height_difference(grid, point, neighbor) <= 1 {
            neighbours.push(neighbor);
        }
    }
    if point.y < (height - 1) as i32 {
        let neighbor = Point {
            x: point.x,
            y: point.y + 1,
        };
        if get_height_difference(grid, point, neighbor) <= 1 {
            neighbours.push(neighbor);
        }
    }
    neighbours
}

fn get_height_difference(grid: &[Vec<i8>], p1: Point, p2: Point) -> i8 {
    grid[p2.y as usize][p2.x as usize] - grid[p1.y as usize][p1.x as usize]
}

fn get_successors(
    grid: &[Vec<i8>],
    width: usize,
    height: usize,
) -> impl Fn(Point) -> Vec<(Point, u32)> + '_ {
    move |point: Point| {
        get_neighbor_points(grid, point, width, height)
            .into_iter()
            .map(|s| (s, 1))
            .collect()
    }
}
