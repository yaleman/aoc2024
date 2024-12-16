use aoc2024::*;

pub fn main() {
    // load the file
    let cli = CliOpts::parse();
    let filecontents = load_file(4, cli.sample).expect("Failed to load");

    if cli.debug {
        println!("filecontents: {:?}", filecontents);
    }
}
