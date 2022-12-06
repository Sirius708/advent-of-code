use std::collections::HashSet;

fn main() {
    let input_lines = util::get_input_lines();

    let priority_sum_one: i32 = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let one = &line[..(line.len() / 2)];
            let two = &line[(line.len() / 2)..];
            (one, two)
        })
        .filter_map(|(one, two)| {
            let chars = one.chars().collect::<HashSet<_>>();
            two.chars().find(|ch| chars.contains(ch))
        })
        .map(|item| match item {
            'a'..='z' => (item as i32) - ('a' as i32) + 1,
            'A'..='Z' => (item as i32) - ('A' as i32) + 27,
            _ => unreachable!(),
        })
        .sum();
    println!("Priority sum 1: {}", priority_sum_one);

    let priority_sum_two: i32 = input_lines
        .iter()
        .filter(|line| !line.is_empty())
        .fold(vec![vec![]], |mut acc, line| {
            if let Some(list) = acc.last_mut() {
                if list.len() < 3 {
                    list.push(line);
                } else {
                    acc.push(vec![line]);
                }
            }
            acc
        })
        .into_iter()
        .filter_map(|list| {
            list.into_iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).copied().collect())
        })
        .filter_map(|item| item.into_iter().next())
        .map(|item| match item {
            'a'..='z' => (item as i32) - ('a' as i32) + 1,
            'A'..='Z' => (item as i32) - ('A' as i32) + 27,
            _ => unreachable!(),
        })
        .sum();
    println!("Priority sum 2: {}", priority_sum_two);
}
