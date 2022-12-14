use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Sum,
    Mul,
}

#[derive(Debug, Clone)]
enum OperationValue {
    OldValue,
    Value(u64),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    operation_value: OperationValue,
    test_divisor: u64,
    test_true_target: u32,
    test_false_target: u32,
}

fn parse_monkeys() -> Vec<Monkey> {
    let input_lines = util::get_input_lines();
    let grouped_lines = input_lines.into_iter().fold(vec![], |mut acc, item| {
        if item.trim().is_empty() {
            acc.push(vec![]);
        } else if let Some(last) = acc.last_mut() {
            last.push(item);
        } else {
            acc.push(vec![item]);
        }
        acc
    });

    grouped_lines
        .into_iter()
        .map(|lines| {
            let items = &lines[1];
            let operation = &lines[2];
            let test_divisor = &lines[3];
            let test_true = &lines[4];
            let test_false = &lines[5];

            let items = items
                .trim()
                .split(&[' ', ','])
                .skip(2)
                .filter(|item| !item.is_empty())
                .map(|item| item.parse().unwrap())
                .collect();

            let (operation, operation_value) = {
                let mut iter = operation.trim().split_ascii_whitespace().skip(4);
                let op = match iter.next().unwrap() {
                    "+" => Operation::Sum,
                    "*" => Operation::Mul,
                    _ => unreachable!(),
                };
                let value = match iter.next().unwrap() {
                    "old" => OperationValue::OldValue,
                    value => OperationValue::Value(value.parse().unwrap()),
                };
                (op, value)
            };

            let test_divisor = test_divisor
                .trim()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let test_true_target = test_true
                .trim()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let test_false_target = test_false
                .trim()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            Monkey {
                items,
                operation,
                operation_value,
                test_divisor,
                test_true_target,
                test_false_target,
            }
        })
        .collect::<Vec<_>>()
}

fn main() {
    let monkeys = parse_monkeys();
    part_one(monkeys.clone());
    part_two(monkeys);
}

fn play_monkey_round<F>(monkeys: &mut [Monkey], inspections: &mut [u32], worry_reducer: F)
where
    F: Fn(u64) -> u64,
{
    let mut moved_items = vec![];
    for (monkey_index, monkey) in monkeys.iter_mut().enumerate() {
        if !moved_items.is_empty() {
            while let Some(new_item_index) = moved_items
                .iter()
                .position(|&(index, _): &(usize, u64)| monkey_index == index)
            {
                monkey.items.push_back(moved_items.remove(new_item_index).1);
            }
        }

        while let Some(mut item) = monkey.items.pop_front() {
            item = match monkey.operation {
                Operation::Sum => match &monkey.operation_value {
                    OperationValue::OldValue => item + item,
                    OperationValue::Value(value) => item + value,
                },
                Operation::Mul => match &monkey.operation_value {
                    OperationValue::OldValue => item * item,
                    OperationValue::Value(value) => item * value,
                },
            };
            item = worry_reducer(item);

            inspections[monkey_index] += 1;

            if item % monkey.test_divisor == 0 {
                moved_items.push((monkey.test_true_target as usize, item));
            } else {
                moved_items.push((monkey.test_false_target as usize, item));
            }
        }
    }
    if !moved_items.is_empty() {
        for (monkey_index, item) in moved_items {
            monkeys[monkey_index].items.push_back(item);
        }
    }
}

fn part_one(mut monkeys: Vec<Monkey>) {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 1..=20 {
        play_monkey_round(&mut monkeys, &mut inspections, |worry| worry / 3);
    }

    inspections.sort_unstable();
    inspections.reverse();

    let monkey_business = inspections[0] * inspections[1];
    println!("Part One: Level of monkey business: {}", monkey_business);
}

fn part_two(mut monkeys: Vec<Monkey>) {
    let mut inspections = vec![0; monkeys.len()];

    let divisor_lcm = monkeys
        .iter()
        .map(|m| m.test_divisor)
        .reduce(util::lcm)
        .unwrap();

    for _ in 1..=10_000 {
        play_monkey_round(&mut monkeys, &mut inspections, |worry| worry % divisor_lcm);
    }

    inspections.sort_unstable();
    inspections.reverse();

    let monkey_business = inspections[0] as u128 * inspections[1] as u128;
    println!("Part Two: Level of monkey business: {}", monkey_business);
}
