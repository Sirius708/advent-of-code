fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example() {
        let input = include_str!("example1.txt");
        let result = part1(input);
        assert_eq!("142", result);
    }
}
