use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    UpSlope,
    DownSlope,
    LeftSlope,
    RightSlope,
}

fn find_longest_path(map: &Vec<Vec<Tile>>, slippery: bool) -> usize {
    // create a graph where the nodes are intersections. edges connect
    // nodes that have a path between them without containing another node.
    // the weight of the node is the length of that connection.
    let size = map.len();
    let goal = (size - 2, size - 3);
    let start = (1, 2);

    let ds = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];

    let nodes = {
        let mut nodes = vec![start, goal];
        for r in 1..(size - 1) {
            for c in 1..(size - 1) {
                if map[r][c] == Tile::Forest {
                    continue;
                }
                let n_count = ds
                    .iter()
                    .filter(|(d_r, d_c)| {
                        let (n_r, n_c) = ((r as i32 + d_r) as usize, (c as i32 + d_c) as usize);
                        map[n_r][n_c] != Tile::Forest
                    })
                    .count();
                if n_count > 2 {
                    nodes.push((r, c));
                }
            }
        }
        nodes
    };

    let node_map = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect::<HashMap<_, _>>();

    let edges = nodes
        .iter()
        .map(|node| {
            let mut node_edges = vec![];
            let mut to_visit = vec![vec![node.clone()]];
            while let Some(next) = to_visit.pop() {
                let latest = next.last().unwrap().clone();

                if &latest != node && node_map.contains_key(&latest) {
                    node_edges.push((node_map[&latest], next.len() - 1));
                    continue;
                }

                let (r, c) = latest;
                let tile = map[r][c];
                for &(d_r, d_c) in ds.iter() {
                    let (n_r, n_c) = ((r as i32 + d_r) as usize, (c as i32 + d_c) as usize);
                    let valid_dir = if slippery {
                        match tile {
                            Tile::Path => true,
                            Tile::DownSlope => d_r == 1,
                            Tile::LeftSlope => d_c == -1,
                            Tile::UpSlope => d_r == -1,
                            Tile::RightSlope => d_c == 1,
                            _ => panic!(),
                        }
                    } else {
                        true
                    };
                    if !valid_dir {
                        continue;
                    }
                    if map[n_r][n_c] == Tile::Forest {
                        continue;
                    }

                    if next.contains(&(n_r, n_c)) {
                        continue;
                    }

                    let mut path = next.clone();
                    path.push((n_r, n_c));
                    to_visit.push(path);
                }
            }

            node_edges
        })
        .collect::<Vec<_>>();

    let mut res = None;

    let mut to_visit = vec![(vec![0usize], 0)];

    while let Some(next) = to_visit.pop() {
        let (path, dist) = next;
        let &latest = path.last().unwrap();

        if latest == 1 {
            res = Some(dist.max(res.unwrap_or(0)));
            continue;
        }

        for (c, d) in edges[latest].iter() {
            if path.contains(c) {
                continue;
            }

            let mut path = path.clone();
            path.push(*c);
            to_visit.push((path, dist + d));
        }
    }

    res.unwrap()
}

fn parse_input(s: &str) -> Vec<Vec<Tile>> {
    let tiles = s
        .trim()
        .lines()
        .map(|line| {
            let mut row = vec![Tile::Forest];
            row.extend(line.trim().chars().map(|ch| match ch {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '^' => Tile::UpSlope,
                '>' => Tile::RightSlope,
                '<' => Tile::LeftSlope,
                'v' => Tile::DownSlope,
                _ => panic!(),
            }));
            row.push(Tile::Forest);
            row
        })
        .collect::<Vec<_>>();

    let size = {
        let num_rows = tiles.len();
        let num_cols = tiles[0].len();
        assert_eq!(num_rows + 2, num_cols);
        num_rows
    };

    let mut padded_tiles = vec![];
    padded_tiles.push(vec![Tile::Forest; size + 2]);
    padded_tiles.extend(tiles.into_iter());
    padded_tiles.push(vec![Tile::Forest; size + 2]);

    padded_tiles
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 23: A Long Walk");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let map = parse_input(&raw_input);

    let part_one = find_longest_path(&map, true);
    let part_two = find_longest_path(&map, false);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
