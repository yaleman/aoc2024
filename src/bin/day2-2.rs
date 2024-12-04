use aoc2024::*;

fn is_safe(input: &[u64], cli: &CliOpts) -> bool {
    let mut sorted = input.to_vec();
    sorted.sort();
    let mut rev_sort = input.to_vec();
    rev_sort.sort();
    rev_sort.reverse();
    let in_order = (sorted == input) || (rev_sort == input);

    let diffs: Vec<_> = input
        .iter()
        .enumerate()
        .filter_map(|(index, left)| input.get(index + 1).map(|right| left.abs_diff(*right)))
        .collect();
    let diffs_ok = diffs.into_iter().all(|v| 0 < v && v <= 3);
    if cli.debug {
        println!(
            "in_order: {}\t\tdiffs_ok: {}\t\tres: {}",
            in_order,
            diffs_ok,
            in_order && diffs_ok
        );
    }
    in_order && diffs_ok
}

fn safe_without_one(input: &[u64], cli: &CliOpts) -> bool {
    (0..(input.len())).any(|index| {
        let mut test_case = input.to_vec();
        test_case.remove(index);
        is_safe(&test_case, cli)
    })
}

pub fn main() -> Result<(), std::io::Error> {
    let cli = CliOpts::parse();

    let file_contents = load_file(2, cli.sample)?;

    let reports: Vec<_> = file_contents
        .lines()
        .map(|line| {
            let nums = line
                .split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            println!("line: {} nums: {:?}", line, nums);
            nums
        })
        .collect();

    let safe_reports: usize = reports
        .into_iter()
        .filter(|report| safe_without_one(report, &cli))
        .collect::<Vec<Vec<u64>>>()
        .len();

    println!("safe_reports: {:?}", safe_reports);

    Ok(())
}
