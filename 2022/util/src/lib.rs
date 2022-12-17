use priority_queue::PriorityQueue;
use std::cmp::max;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn get_input_file() -> &'static str {
    if std::env::var("DEMO").is_ok() {
        "demo_input.txt"
    } else {
        "input.txt"
    }
}

pub fn get_input_lines() -> Vec<String> {
    std::fs::read_to_string(get_input_file())
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

/// [Chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance)
///
/// Distance on a square grid with horizontal, vertical and diagonal movement.
pub fn chebyshev_distance_2d(p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32) -> i32 {
    max((p1_x - p2_x).abs(), (p1_y - p2_y).abs())
}

/// [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
///
/// Distance on a square grid with horizontal and vertical movement.
pub fn manhatten_distance_2d(p1_x: i32, p1_y: i32, p2_x: i32, p2_y: i32) -> u32 {
    (p2_x - p1_x).unsigned_abs() + (p2_y - p1_y).unsigned_abs()
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a * (b / gcd(a, b))
}

/// [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm)
pub fn find_shortest_distance<T, F1, F2>(
    start: T,
    goal: T,
    get_successors: F1,
    goal_distance_estimate: F2,
) -> Option<u32>
where
    T: Eq + Hash + Copy,
    F1: Fn(T) -> Vec<(T, u32)>,
    F2: Fn(T) -> u32,
{
    let mut visited_nodes = HashSet::new();
    let mut open_nodes = PriorityQueue::new();
    open_nodes.push(start, Reverse(0));
    let mut start_distance = HashMap::new();
    start_distance.insert(start, 0);

    while let Some((current_node, _)) = open_nodes.pop() {
        if current_node == goal {
            return Some(start_distance[&current_node]);
        }

        visited_nodes.insert(current_node);

        for (successor, successor_distance) in get_successors(current_node) {
            if visited_nodes.contains(&successor) {
                continue;
            }

            let current_start_distance = start_distance[&current_node];
            let successor_start_distance = current_start_distance + successor_distance;

            if let Some(distance) = start_distance.get(&successor) {
                if *distance < successor_start_distance {
                    continue;
                }
            }

            start_distance.insert(successor, successor_start_distance);

            let successor_cost = successor_start_distance + goal_distance_estimate(successor);
            open_nodes.push(successor, Reverse(successor_cost));
        }
    }
    None
}
