fn main() {
    let lines = util::get_input_lines();
    let sections = lines.splitn(2, |line| line.is_empty()).collect::<Vec<_>>();
    let (stacks, moves) = (&sections[0][..(sections[0].len() - 1)], sections[1]);

    let rows = stacks
        .iter()
        .map(|line| {
            let mut columns = vec![];
            let mut index = 0;
            while index < line.len() {
                columns.push(&line[index..(index + 3)]);
                index += 4;
            }
            columns
        })
        .collect::<Vec<_>>();
    let mut stacks = vec![vec![]; rows.iter().map(|columns| columns.len()).max().unwrap()];
    for row in rows {
        for (i, column) in row.iter().enumerate() {
            let letter = column.chars().nth(1).unwrap();
            if ('A'..='Z').contains(&letter) {
                stacks[i].insert(0, letter);
            }
        }
    }

    let moves = moves
        .iter()
        .map(|line| {
            let words = line.split_ascii_whitespace().collect::<Vec<_>>();
            let count = words[1].parse::<usize>().unwrap();
            let from = words[3].parse::<usize>().unwrap() - 1;
            let to = words[5].parse::<usize>().unwrap() - 1;
            (count, from, to)
        })
        .collect::<Vec<_>>();

    // Part One
    // for (count, from, to) in moves {
    //     for _ in 0..count {
    //         let value = stacks[from].pop().unwrap();
    //         stacks[to].push(value);
    //     }
    // }

    // Part Two
    for (count, from, to) in moves {
        for i in (1..=count).rev() {
            let value = stacks[from][stacks[from].len() - i];
            stacks[to].push(value);
        }
        let new_len = stacks[from].len() - count;
        stacks[from].resize(new_len, 'x');
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
