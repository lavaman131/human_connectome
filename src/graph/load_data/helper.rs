pub fn to_tuple(input: String) -> (usize, usize) {
    // expects input like "(0, 0)"
    // splits like ["", "0", "", "0"]
    let input = input.trim().to_string();
    let list: Vec<&str> = input.split(['(', ',', ' ', ')']).collect();
    let a: usize = list[1].parse().expect("Failed to convert to number.");
    let b: usize = list[list.len() - 2]
        .parse()
        .expect("Failed to convert to number.");
    (a, b)
}
