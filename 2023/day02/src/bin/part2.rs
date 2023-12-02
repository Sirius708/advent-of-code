use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

type ColorSet = HashMap<Color, u32>;

fn part2(input: &str) -> String {
    let games = input
        .lines()
        .map(|line| {
            let (_, draws) = line.split_once(':').unwrap();
            draws
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
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut power_sum = 0;
    for draws in games {
        let mut max_colors = ColorSet::new();
        max_colors.insert(Color::Red, 0);
        max_colors.insert(Color::Green, 0);
        max_colors.insert(Color::Blue, 0);

        for draw in draws {
            if let Some(red) = draw.get(&Color::Red) {
                max_colors.insert(Color::Red, max(max_colors[&Color::Red], *red));
            }
            if let Some(green) = draw.get(&Color::Green) {
                max_colors.insert(Color::Green, max(max_colors[&Color::Green], *green));
            }
            if let Some(blue) = draw.get(&Color::Blue) {
                max_colors.insert(Color::Blue, max(max_colors[&Color::Blue], *blue));
            }
        }

        power_sum += max_colors[&Color::Red] * max_colors[&Color::Green] * max_colors[&Color::Blue];
    }
    power_sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        let input = include_str!("example2.txt");
        let result = part2(input);
        assert_eq!("2286", result);
    }
}
