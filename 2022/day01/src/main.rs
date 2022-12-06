fn main() {
    let lines = util::get_input_lines();
    let mut last_index = 0;
    let mut calories = vec![];
    while last_index < lines.len() {
        let mut sum = 0;
        for i in last_index..lines.len() {
            let line = &lines[i];
            if line.is_empty() {
                last_index = i + 1;
                break;
            }
            sum += line.parse::<i32>().unwrap();
            if i == (lines.len() - 1) {
                last_index = lines.len()
            }
        }
        calories.push(sum);
    }
    calories.sort_unstable();

    let single_total = calories.last().unwrap();
    println!("Total calories of top Elf: {}", single_total);

    let three_total: i32 = calories[(calories.len() - 3)..].iter().sum();
    println!("Total calories of top 3 Elves: {}", three_total);
}
