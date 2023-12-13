use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn summarize_grids(grids: &Vec<Vec<Vec<bool>>>, smudge: bool) -> usize {
    grids.iter().map(|grid| summarize_grid(grid, smudge)).sum()
}

fn summarize_grid(grid: &Vec<Vec<bool>>, smudge: bool) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    if let Some(val) = find_mirror(grid, smudge) {
        100 * (val + 1)
    } else {
        let tranpose = (0..num_cols)
            .map(|c| (0..num_rows).map(|r| grid[r][c]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let val = find_mirror(&tranpose, smudge).unwrap();
        val + 1
    }
}

fn find_mirror(matrix: &Vec<Vec<bool>>, smudge: bool) -> Option<usize> {
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    (0..(num_rows) - 1).find(|&i| {
        let max = (i + 1).min(num_rows - i - 1);
        let (single_error_count, multiple_error_count) =
            (0..max).fold((0, 0), |(sing, mult), j| {
                let v1 = &matrix[i - j];
                let v2 = &matrix[i + j + 1];
                let errors = (0..num_cols).filter(|&k| v1[k] != v2[k]).count();
                if errors == 0 {
                    (sing, mult)
                } else if errors == 1 {
                    (sing + 1, mult)
                } else {
                    (sing, mult + 1)
                }
            });
        if smudge {
            multiple_error_count == 0 && single_error_count == 1
        } else {
            multiple_error_count == 0 && single_error_count == 0
        }
    })
}

fn parse_input(input: &str) -> Vec<Vec<Vec<bool>>> {
    input
        .trim()
        .split("\n\n")
        .map(|chk| {
            chk.trim()
                .lines()
                .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 13: Point of Incidence");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let grids = parse_input(&raw_input);

    let part_one = summarize_grids(&grids, false);
    let part_two = summarize_grids(&grids, true);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
