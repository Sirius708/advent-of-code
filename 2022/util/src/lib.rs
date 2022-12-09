use std::cmp::max;

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
