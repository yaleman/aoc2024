use aoc2024::*;
use regex::Regex;

pub fn main() -> Result<(), std::io::Error> {
    let cli = CliOpts::parse();
    let file_contents = load_file(3, cli.sample)?;

    let finder = Regex::new(r#"(mul\(\d+,\d+\))"#).expect("Failed to compile regex");
    let mut result = 0;

    for value in finder.find_iter(&file_contents) {
        if cli.debug {
            println!("{:?}", value);
        }
        let ints: Vec<String> = value
            .as_str()
            .replace("mul(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let left: u64 = ints[0].parse().expect("Failed to parse left");
        let right: u64 = ints[1].parse().expect("Failed to parse right");
        if cli.debug {
            println!("mul({},{}) = {}", left, right, left * right);
        }
        result += left * right;
    }

    println!("Result: {}", result);
    Ok(())
}
