fn get_input_file() -> &'static str {
    if std::env::var("DEMO").is_ok() {
        "demo_input.txt"
    } else {
        "input.txt"
    }
}

pub fn get_input_lines() -> Vec<String> {
    std::fs::read_to_string(get_input_file())
        .unwrap()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}
