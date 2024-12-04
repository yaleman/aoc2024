use aoc2024::load_file;

pub fn main() -> Result<(), std::io::Error> {
    // load the file
    let filecontents = load_file(2)?;

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

    let sim_scores: Vec<usize> = left_list
        .into_iter()
        .map(|left| {
            let diff_score = right_list
                .iter()
                .filter(|right| left == **right)
                .cloned()
                .collect::<Vec<usize>>()
                .len();
            diff_score * left
        })
        .collect();

    let final_sum: usize = sim_scores.into_iter().sum();

    println!("Final sum: {}", final_sum);
    Ok(())
}
