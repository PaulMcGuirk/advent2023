use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn count_safe(bricks: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut bricks = bricks.clone();
    bricks.sort_by_key(|b| b.iter().map(|c| c[2]).min().unwrap());

    let maxes = (0..3)
        .map(|i| {
            bricks
                .iter()
                .map(|b| b.iter().map(|c| c[i]).max().unwrap())
                .max()
                .unwrap()
                + 1
        })
        .collect::<Vec<_>>();

    let mut tiles = vec![vec![vec![Option::<usize>::None; maxes[2]]; maxes[1]]; maxes[0]];
    let mut supports = vec![vec![]; bricks.len()];

    for (i, brick) in bricks.iter_mut().enumerate() {
        let x_min = brick.iter().map(|c| c[0]).min().unwrap();
        let x_max = brick.iter().map(|c| c[0]).max().unwrap();
        let y_min = brick.iter().map(|c| c[1]).min().unwrap();
        let y_max = brick.iter().map(|c| c[1]).max().unwrap();
        let z_min = brick.iter().map(|c| c[2]).min().unwrap();

        let z = (1usize..z_min)
            .rev()
            .find(|&z| (x_min..=x_max).any(|x| (y_min..=y_max).any(|y| tiles[x][y][z].is_some())))
            .unwrap_or(0)
            + 1;

        let below = (x_min..=x_max)
            .flat_map(|x| {
                (y_min..=y_max)
                    .filter_map(|y| tiles[x][y][z - 1])
                    .collect::<HashSet<_>>()
            })
            .collect::<HashSet<_>>();
        supports[i].extend(below);

        for c in brick.iter_mut() {
            c[2] = z + c[2] - z_min;
            tiles[c[0]][c[1]][c[2]] = Some(i);
        }
    }

    (0..bricks.len())
        .filter(|&i| !supports.iter().any(|s| s.len() == 1 && s[0] == i))
        .count()
}

fn chain_reaction(bricks: &Vec<Vec<Vec<usize>>>) -> usize {
    let mut bricks = bricks.clone();
    bricks.sort_by_key(|b| b.iter().map(|c| c[2]).min().unwrap());

    let baseline = {
        let mut bricks = bricks.clone();

        let maxes = (0..3)
            .map(|i| {
                bricks
                    .iter()
                    .map(|b| b.iter().map(|c| c[i]).max().unwrap())
                    .max()
                    .unwrap()
                    + 1
            })
            .collect::<Vec<_>>();

        let mut tiles = vec![vec![vec![Option::<usize>::None; maxes[2]]; maxes[1]]; maxes[0]];

        for (i, brick) in bricks.iter_mut().enumerate() {
            let x_min = brick.iter().map(|c| c[0]).min().unwrap();
            let x_max = brick.iter().map(|c| c[0]).max().unwrap();
            let y_min = brick.iter().map(|c| c[1]).min().unwrap();
            let y_max = brick.iter().map(|c| c[1]).max().unwrap();
            let z_min = brick.iter().map(|c| c[2]).min().unwrap();

            let z = (1usize..z_min)
                .rev()
                .find(|&z| {
                    (x_min..=x_max).any(|x| (y_min..=y_max).any(|y| tiles[x][y][z].is_some()))
                })
                .unwrap_or(0)
                + 1;

            for c in brick.iter_mut() {
                c[2] = z + c[2] - z_min;
                tiles[c[0]][c[1]][c[2]] = Some(i);
            }
        }

        bricks
    };

    let mut res = 0;
    for dis in 0..bricks.len() {
        let mut bricks = bricks.clone();

        bricks.sort_by_key(|b| b.iter().map(|c| c[2]).min().unwrap());

        let maxes = (0..3)
            .map(|i| {
                bricks
                    .iter()
                    .map(|b| b.iter().map(|c| c[i]).max().unwrap())
                    .max()
                    .unwrap()
                    + 1
            })
            .collect::<Vec<_>>();

        let mut tiles = vec![vec![vec![Option::<usize>::None; maxes[2]]; maxes[1]]; maxes[0]];

        for (i, brick) in bricks.iter_mut().enumerate() {
            if i == dis {
                continue;
            }
            let x_min = brick.iter().map(|c| c[0]).min().unwrap();
            let x_max = brick.iter().map(|c| c[0]).max().unwrap();
            let y_min = brick.iter().map(|c| c[1]).min().unwrap();
            let y_max = brick.iter().map(|c| c[1]).max().unwrap();
            let z_min = brick.iter().map(|c| c[2]).min().unwrap();

            let z = (1usize..z_min)
                .rev()
                .find(|&z| {
                    (x_min..=x_max).any(|x| (y_min..=y_max).any(|y| tiles[x][y][z].is_some()))
                })
                .unwrap_or(0)
                + 1;

            for c in brick.iter_mut() {
                c[2] = z + c[2] - z_min;
                tiles[c[0]][c[1]][c[2]] = Some(i);
            }
        }

        let fell = (0..bricks.len())
            .filter(|&i| {
                i != dis
                    && baseline[i].iter().map(|c| c[2]).min() > bricks[i].iter().map(|c| c[2]).min()
            })
            .count();
        res += fell;
    }

    res
}

fn parse_input(input: &str) -> Vec<Vec<Vec<usize>>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let ends = line
                .split("~")
                .map(|side| {
                    side.trim()
                        .split(",")
                        .map(|v| v.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let diff = (0..3).find(|&i| ends[0][i] != ends[1][i]).unwrap_or(0);
            let start = ends[0][diff].min(ends[1][diff]);
            let end = ends[0][diff].max(ends[1][diff]);
            (start..=end)
                .map(|d| {
                    (0..3)
                        .map(|i| if i == diff { d } else { ends[0][i] })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 22: Sand Slabs");

    let now = Instant::now();

    let raw_input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let bricks = parse_input(&raw_input);

    let part_one = count_safe(&bricks);
    let part_two = chain_reaction(&bricks);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
