use std::collections::HashSet;
use util::Grid;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Nothing,
    Number(u32, u32),
    Symbol,
}

fn part1(input: &str) -> String {
    let mut num_id = 0;
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            let mut row: Vec<_> = line
                .chars()
                .map(|c| match c {
                    '.' => Cell::Nothing,
                    '0'..='9' => Cell::Number(0, c.to_digit(10).unwrap()),
                    _ => Cell::Symbol,
                })
                .collect();

            let mut x = 0;
            while x < row.len() {
                if let Cell::Number(_, _) = row[x] {
                    let start = x;
                    while let Cell::Number(_, _) = row[x] {
                        x += 1;
                        if x >= row.len() {
                            break;
                        }
                    }

                    let digits = &mut row[start..x];
                    let mut number = 0;
                    for cell in digits.iter() {
                        if let Cell::Number(_, n) = cell {
                            number *= 10;
                            number += *n;
                        }
                    }
                    digits.fill(Cell::Number(num_id, number));
                    num_id += 1;
                }
                x += 1;
            }
            row
        })
        .collect();

    let mut numbers = HashSet::new();
    grid.for_each_2d(|x, y, cell| {
        if let Cell::Symbol = cell {
            for surr_cell in grid.get_surrounding(x, y) {
                if let Cell::Number(id, n) = surr_cell {
                    numbers.insert((*id, *n));
                }
            }
        }
    });
    numbers.iter().map(|(_, n)| n).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        let input = include_str!("example1.txt");
        let result = part1(input);
        assert_eq!("4361", result);
    }
}
