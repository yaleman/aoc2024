pub fn load_file(part_num: usize) -> Result<String, std::io::Error> {
    let full_list = std::path::PathBuf::from(format!("./inputs/{:0>2}.txt", part_num));
    let full_list_display = full_list.display();
    if full_list.clone().exists() {
        println!("Loading example input");
        return std::fs::read_to_string(&full_list)
            .inspect_err(|err| println!("Failed to load {}: {:?}", full_list_display, err));
    }

    let example_list = std::path::PathBuf::from(format!("./inputs/{:0>2}_example.txt", part_num));
    let example_list_display = example_list.display();
    if example_list.clone().exists() {
        println!("Loading full input");
        return std::fs::read_to_string(&example_list)
            .inspect_err(|err| println!("Failed to load {}: {:?}", example_list_display, err));
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Couldn't find example for {}", part_num).as_str(),
    ))
}
