use std::cmp::{max, min};
use std::iter::once;

#[derive(Debug, Copy, Clone)]
struct Bounds {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl Bounds {
    pub fn get_width(&self) -> usize {
        (self.max_x - self.min_x + 1) as usize
    }

    pub fn get_height(&self) -> usize {
        (self.max_y - self.min_y + 1) as usize
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        (self.min_x..=self.max_x).contains(&x) && (self.min_y..=self.max_y).contains(&y)
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<bool>>,
    bounds: Bounds,
}

impl Grid {
    pub fn new(bounds: Bounds) -> Self {
        Self {
            data: vec![vec![false; bounds.get_width()]; bounds.get_height() + 2],
            bounds,
        }
    }

    pub fn place_block(&mut self, x: i32, y: i32) {
        self.data[(y - self.bounds.min_y) as usize][(x - self.bounds.min_x) as usize] = true;
    }

    pub fn is_block(&self, x: i32, y: i32) -> bool {
        self.contains(x, y)
            && self.data[(y - self.bounds.min_y) as usize][(x - self.bounds.min_x) as usize]
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.bounds.contains(x, y)
    }

    pub fn is_bedrock(&self, y: i32) -> bool {
        y >= (self.bounds.max_y + 2)
    }

    pub fn is_block_or_bedrock(&self, x: i32, y: i32) -> bool {
        self.is_bedrock(y)
            || self.data[(y - self.bounds.min_y) as usize][(x - self.bounds.min_x) as usize]
    }
}

fn main() {
    let input_lines = util::get_input_lines();
    let rock_paths = input_lines
        .into_iter()
        .map(|line| {
            line.split(" -> ")
                .map(|coordinates| {
                    let (x, y) = coordinates.split_once(',').unwrap();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (min_x, min_y, max_x, max_y) = rock_paths.iter().chain(once(&vec![(500, 0)])).fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(mut min_x, mut min_y, mut max_x, mut max_y), path| {
            for (x, y) in path {
                min_x = min(min_x, *x);
                min_y = min(min_y, *y);
                max_x = max(max_x, *x);
                max_y = max(max_y, *y);
            }
            (min_x, min_y, max_x, max_y)
        },
    );
    let bounds = Bounds {
        min_x: min_x - (max_y - min_y + 1),
        min_y,
        max_x: max_x + (max_y - min_y + 1),
        max_y,
    };

    let mut grid = Grid::new(bounds);
    draw_rock_paths(&mut grid, &rock_paths);
    let mut sand_grains = 0;
    while !part_1_let_sand_fall(&mut grid, 500, 0) {
        sand_grains += 1;
    }
    println!("{sand_grains} sand grains at the edge of the abyss.");

    let mut grid = Grid::new(bounds);
    draw_rock_paths(&mut grid, &rock_paths);
    let mut sand_grains = 0;
    while !part_2_pyramid_schemes(&mut grid, 500, 0) {
        sand_grains += 1;
    }
    println!("{sand_grains} sand grains to bedrock.");
}

fn draw_rock_paths(grid: &mut Grid, rock_paths: &[Vec<(i32, i32)>]) {
    for path in rock_paths {
        if path.is_empty() {
            continue;
        }
        if path.len() == 1 {
            let (x, y) = path[0];
            grid.place_block(x, y);
            continue;
        }

        let mut previous = path[0];
        for &current in path.iter().skip(1) {
            let (previous_x, previous_y) = previous;
            let (current_x, current_y) = current;
            for x in min(previous_x, current_x)..=max(previous_x, current_x) {
                grid.place_block(x, current_y);
            }
            for y in min(previous_y, current_y)..=max(previous_y, current_y) {
                grid.place_block(current_x, y);
            }
            previous = current;
        }
    }
}

fn part_1_let_sand_fall(grid: &mut Grid, start_x: i32, start_y: i32) -> bool {
    if !grid.contains(start_x, start_y) {
        return true;
    }

    let mut current_y = start_y;
    while grid.contains(start_x, current_y + 1) && !grid.is_block(start_x, current_y + 1) {
        current_y += 1;
    }

    if !grid.is_block(start_x - 1, current_y + 1) {
        part_1_let_sand_fall(grid, start_x - 1, current_y + 1)
    } else if !grid.is_block(start_x + 1, current_y + 1) {
        part_1_let_sand_fall(grid, start_x + 1, current_y + 1)
    } else {
        grid.place_block(start_x, current_y);
        false
    }
}

fn part_2_pyramid_schemes(grid: &mut Grid, start_x: i32, start_y: i32) -> bool {
    if grid.is_block(start_x, start_y) {
        return true;
    }

    let mut current_x = start_x;
    let mut current_y = start_y;
    loop {
        if !grid.is_block_or_bedrock(current_x, current_y + 1) {
            current_y += 1;
        } else if !grid.is_block_or_bedrock(current_x - 1, current_y + 1) {
            current_x -= 1;
            current_y += 1;
        } else if !grid.is_block_or_bedrock(current_x + 1, current_y + 1) {
            current_x += 1;
            current_y += 1;
        } else {
            break;
        }
    }
    grid.place_block(current_x, current_y);
    false
}
