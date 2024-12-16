use aoc2024::*;

// This doesn't work and I don't have the brain for it today

fn find_in_rows(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|row| {
            // find the count of the string XMAS in the row
            let mut count = 0;
            let mut i = 0;
            while i <= row.len() - 4 {
                let current_row = &row[i..i + 4];
                if current_row == "XMAS" || current_row == "SMAX" {
                    count += 1;
                    i += 4;
                } else {
                    i += 1;
                }
            }
            count
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>()
}

pub fn main() {
    // load the file
    let cli = CliOpts::parse();
    let filecontents = load_file(4, cli.sample).expect("Failed to load");

    if cli.debug {
        for line in filecontents.lines() {
            println!("{}", line);
        }
    }

    let lines = filecontents
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let row_count = find_in_rows(&lines);

    println!("row_count: {:?}", row_count);

    // we assume a sane board.
    let x_width = lines.get(0).unwrap().len();
    let y_height = lines.len();

    println!("x_width: {:?}", x_width);

    // make a rotated board
    let rotated_board = (0..x_width)
        .map(|x| {
            (0..y_height)
                .map(|y| lines.get(y).unwrap().chars().nth(x).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let col_count = find_in_rows(&rotated_board);
    println!("col_count: {:?}", col_count);

    // search the diagonals
    let mut diag_count = 0;
    for x in 0..x_width {
        for y in 0..y_height {
            // scan the board looking for diagonals starting in the top right and going down to the left
            if x + 3 < x_width && y + 3 < y_height {
                let mut diag = String::new();
                for i in 0..4 {
                    diag.push(lines.get(y + i).unwrap().chars().nth(x + i).unwrap());
                }
                if diag == "XMAS" {
                    diag_count += 1;
                }
            }
            // scan the board looking for diagonals starting in the top left and going down to the right
            if x + 3 < x_width && y >= 3 {
                let mut diag = String::new();
                for i in 0..4 {
                    diag.push(lines.get(y - i).unwrap().chars().nth(x + i).unwrap());
                }
                if diag == "XMAS" {
                    diag_count += 1;
                }
            }

            // scan the board looking for diagonals starting in the bottom right and going up to the left
            if x >= 3 && y >= 3 {
                let mut diag = String::new();
                for i in 0..4 {
                    diag.push(lines.get(y - i).unwrap().chars().nth(x - i).unwrap());
                }
                if diag == "XMAS" {
                    diag_count += 1;
                }
            }

            // scan the board looking for diagonals starting in the bottom left and going up to the right
            if x >= 3 && y + 3 < y_height {
                let mut diag = String::new();
                for i in 0..4 {
                    diag.push(lines.get(y + i).unwrap().chars().nth(x - i).unwrap());
                }
                if diag == "XMAS" {
                    diag_count += 1;
                }
            }
        }
    }
    let total = row_count + col_count + diag_count;
    println!("Total: {:?}", total);
}
