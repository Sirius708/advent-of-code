fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let line = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");
            let digits: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example() {
        let input = include_str!("example2.txt");
        let result = part2(input);
        assert_eq!("281", result);
    }
}
