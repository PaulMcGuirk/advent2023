use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn part_one(grids: &Vec<Vec<Vec<bool>>>) -> usize {
    grids.iter().map(|grid| summarize(grid)).sum()
}

fn part_two(grids: &Vec<Vec<Vec<bool>>>) -> usize {
    grids.iter().map(|grid| summarize_with_smudge(grid)).sum()
}

fn summarize(grid: &Vec<Vec<bool>>) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let col_nums = (0..num_cols)
        .map(|c| (0..num_rows).fold(0u32, |acc, r| (acc << 1) + if grid[r][c] { 1 } else { 0 }))
        .collect::<Vec<_>>();
    let row_nums = (0..num_rows)
        .map(|r| (0..num_cols).fold(0u32, |acc, c| (acc << 1) + if grid[r][c] { 1 } else { 0 }))
        .collect::<Vec<_>>();

    let col_rel = find_mirror(&col_nums);

    if let Some(col_rel) = col_rel {
        col_rel + 1
    } else {
        let row_rel = find_mirror(&row_nums).unwrap();
        100 * (row_rel + 1)
    }
}

fn summarize_with_smudge(grid: &Vec<Vec<bool>>) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let tranpose = (0..num_cols)
        .map(|c| (0..num_rows).map(|r| grid[r][c]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    if let Some(val) = find_mirror_with_smudge(&tranpose) {
        return val + 1;
    }

    let val = find_mirror_with_smudge(&grid).unwrap();
    100 * (val + 1)
}

fn find_mirror(nums: &Vec<u32>) -> Option<usize> {
    let max = nums.len();
    (0..(max - 1)).find(|&i| {
        (0..=i).all(|j| {
            let k = i + j + 1;
            k >= max || nums[k] == nums[i - j]
        })
    })
}

fn find_mirror_with_smudge(matrix: &Vec<Vec<bool>>) -> Option<usize> {
    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    for i in 0..(num_rows - 1) {
        let mut repl = false;
        for j in 0..=i {
            let v1 = &matrix[i - j];
            if i + j + 1 >= num_rows {
                break;
            }
            let v2 = &matrix[i + j + 1];
            let diffs = (0..num_cols)
                .filter(|&k| v1[k] != v2[k])
                .collect::<Vec<_>>();
            if diffs.is_empty() {
                continue;
            }
            if diffs.len() > 1 {
                repl = false;
                break;
            }
            if repl {
                repl = false;
                break; // more than 1 diff
            }

            repl = true;
        }

        if repl {
            return Some(i);
        }
    }

    None
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

    let raw_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let grids = parse_input(&raw_input);

    let part_one = part_one(&grids);
    let part_two = part_two(&grids);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
