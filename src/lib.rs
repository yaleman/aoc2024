pub use clap::Parser;

pub fn load_file(day_num: usize, sample: bool) -> Result<String, std::io::Error> {
    if !sample {
        let full_list = std::path::PathBuf::from(format!("./inputs/{:0>2}.txt", day_num));
        let full_list_display = full_list.display();
        if full_list.clone().exists() {
            println!("Loading full input");
            return std::fs::read_to_string(&full_list)
                .inspect_err(|err| println!("Failed to load {}: {:?}", full_list_display, err));
        }
    }

    let example_list = std::path::PathBuf::from(format!("./inputs/{:0>2}_example.txt", day_num));
    let example_list_display = example_list.display();
    if example_list.clone().exists() {
        println!("Loading example input");
        return std::fs::read_to_string(&example_list)
            .inspect_err(|err| println!("Failed to load {}: {:?}", example_list_display, err));
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Couldn't find example for {}", day_num).as_str(),
    ))
}

#[derive(Parser)]
pub struct CliOpts {
    #[clap(short, long)]
    pub debug: bool,

    #[clap(short, long)]
    pub sample: bool,
}
