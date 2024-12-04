use aoc2024::*;
use regex::Regex;

pub fn main() -> Result<(), std::io::Error> {
    let cli = CliOpts::parse();
    let file_contents = if cli.sample {
        std::fs::read_to_string("./inputs/03_02_example.txt")?
    } else {
        load_file(3, cli.sample)?
    };

    let mut do_multi = true;

    let finder =
        Regex::new(r#"(mul\(\d+,\d+\)|do\(\)|don't\(\))"#).expect("Failed to compile regex");

    let mut result: u64 = 0;

    for value in finder.find_iter(&file_contents) {
        if cli.debug {
            println!("{:?}", value);
        }
        match value.as_str() {
            "do()" => do_multi = true,
            "don't()" => do_multi = false,
            _ => {
                if do_multi {
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
                } else if cli.debug {
                    println!("Skipping {} as do_multi is off", value.as_str())
                }
            }
        }
    }

    println!("Result: {}", result);
    Ok(())
}
