use aoc2024::*;

pub fn main() {
    // load the file
    let cli = CliOpts::parse();
    let filecontents = load_file(1, cli.sample).expect("Failed to load");

    // println!("File:\n{}", filecontents);

    // read the list into two sets of numbers

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in filecontents.lines() {
        let mut nums = line.split_whitespace();
        if let Some(left_num) = nums.next().map(|ln| {
            ln.parse::<usize>()
                .inspect_err(|err| println!("Failed to parse left_num {}: {:?}", ln, err))
                .expect("Failed to parse")
        }) {
            left_list.push(left_num)
        };
        if let Some(right_num) = nums.next().map(|rn| {
            rn.parse::<usize>()
                .inspect_err(|err| println!("Failed to parse right_num {}: {:?}", rn, err))
                .expect("Failed to parse")
        }) {
            right_list.push(right_num)
        };
    }

    left_list.sort();
    right_list.sort();

    // println!("left set: {:?}", left_list);
    // println!("right set: {:?}", right_list);

    if left_list.len() != right_list.len() {
        panic!(
            "Mismatched list lengths: {} != {}",
            left_list.len(),
            right_list.len()
        );
    }

    let mut final_list = Vec::new();

    for (left_index, left) in left_list.into_iter().enumerate() {
        final_list.push(left.abs_diff(right_list[left_index]))
    }

    // println!("final list: {:?}", final_list);

    let final_sum: usize = final_list.into_iter().sum();

    println!("Final sum: {}", final_sum);
}
