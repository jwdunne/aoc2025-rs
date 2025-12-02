pub fn parse_input(day: u8) -> Vec<String> {
    let input = std::fs::read_to_string(format!("input/day{day:02}.txt"))
        .expect("Failed to read input file");
    input.lines().map(String::from).collect()
}
