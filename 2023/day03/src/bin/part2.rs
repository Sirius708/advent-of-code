use std::collections::HashSet;
use util::Grid;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Nothing,
    Number(u32, u32),
    Gear,
}

fn part2(input: &str) -> String {
    let mut num_id = 0;
    let grid: Vec<_> = input
        .lines()
        .map(|line| {
            let mut row: Vec<_> = line
                .chars()
                .map(|c| match c {
                    '*' => Cell::Gear,
                    '0'..='9' => Cell::Number(0, c.to_digit(10).unwrap()),
                    _ => Cell::Nothing,
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

    let mut sum = 0;
    grid.for_each_2d(|x, y, cell| {
        if let Cell::Gear = cell {
            let cells: HashSet<_> = grid
                .get_surrounding(x, y)
                .iter()
                .filter_map(|c| {
                    if let Cell::Number(id, n) = c {
                        Some((*id, *n))
                    } else {
                        None
                    }
                })
                .collect();
            if cells.len() == 2 {
                sum += cells.iter().map(|(_, n)| n).product::<u32>();
            }
        }
    });
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        let input = include_str!("example2.txt");
        let result = part2(input);
        assert_eq!("467835", result);
    }
}
