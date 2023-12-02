use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

type ColorSet = HashMap<Color, u32>;

fn part1(input: &str) -> String {
    let games = input
        .lines()
        .map(|line| {
            let (game, draws) = line.split_once(':').unwrap();
            let id = game[("Game ".len())..].parse::<u32>().unwrap();
            let draws = draws
                .split(';')
                .map(|draw| {
                    draw.split(',')
                        .map(|color| {
                            let (count, color) = color.trim().split_once(' ').unwrap();
                            let count = count.parse::<u32>().unwrap();
                            let color = match color {
                                "red" => Color::Red,
                                "green" => Color::Green,
                                "blue" => Color::Blue,
                                c => panic!("unknown color '{c}'"),
                            };
                            (color, count)
                        })
                        .collect::<ColorSet>()
                })
                .collect::<Vec<_>>();
            (id, draws)
        })
        .collect::<Vec<_>>();

    let mut id_sum = 0;
    'game: for (id, draws) in games {
        for draw in draws {
            if *draw.get(&Color::Red).unwrap_or(&0) > 12
                || *draw.get(&Color::Green).unwrap_or(&0) > 13
                || *draw.get(&Color::Blue).unwrap_or(&0) > 14
            {
                continue 'game;
            }
        }
        id_sum += id;
    }
    id_sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        let input = include_str!("example1.txt");
        let result = part1(input);
        assert_eq!("8", result);
    }
}
