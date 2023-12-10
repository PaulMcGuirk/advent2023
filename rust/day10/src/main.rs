use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

fn find_main_loop(initial_pos: (usize, usize), grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let (mut r, mut c) = initial_pos;

    let (mut d_r, mut d_c) = match grid[r][c] {
        Tile::Vertical | Tile::SouthEast | Tile::SouthWest => (1isize, 0isize),
        Tile::NorthEast | Tile::NorthWest => (-1, 0),
        Tile::Horizontal => (0, 1),
        Tile::Ground => panic!(),
    };
    let mut main_loop = vec![];

    loop {
        (r, c) = ((r as isize + d_r) as usize, (c as isize + d_c) as usize);
        main_loop.push((r, c));

        if (r, c) == initial_pos {
            break;
        }

        (d_r, d_c) = match grid[r][c] {
            Tile::Vertical | Tile::Horizontal => (d_r, d_c),
            Tile::NorthEast | Tile::SouthWest => (d_c, d_r),
            Tile::NorthWest | Tile::SouthEast => (-d_c, -d_r),
            _ => panic!(),
        };
    }

    main_loop
}

fn measure_loop(polygon: &Vec<(usize, usize)>) -> (usize, usize) {
    let perim = polygon.len();
    let area = polygon
        .iter()
        .zip(polygon.iter().skip(1).chain(polygon.iter().take(1)))
        .map(|(p1, p2)| {
            let &(r1, c1) = p1;
            let &(r2, c2) = p2;
            (c1 * r2) as i32 - (c2 * r1) as i32 // shoelace formula
        })
        .sum::<i32>()
        / 2;
    (perim, area as usize)
}

fn parse_input(raw_input: &str) -> ((usize, usize), Vec<Vec<Tile>>) {
    let chars = raw_input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let num_rows = chars.len();
    let num_cols = chars[0].len();

    let mut grid = vec![];
    grid.push(vec![Tile::Ground; num_cols + 2]);

    let mut starting_pos = None;

    for r in 0..num_rows {
        let mut row = vec![];
        row.push(Tile::Ground);
        for c in 0..num_cols {
            let tile = match chars[r][c] {
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                '7' => Tile::SouthWest,
                'F' => Tile::SouthEast,
                '.' => Tile::Ground,
                'S' => {
                    starting_pos = Some((r + 1, c + 1));
                    Tile::Ground // we'll fix below
                }
                _ => panic!(""),
            };
            row.push(tile);
        }
        row.push(Tile::Ground);
        grid.push(row);
    }

    grid.push(vec![Tile::Ground; num_cols + 2]);

    let (r, c) = starting_pos.unwrap();
    let up = grid[r - 1][c];
    let down = grid[r + 1][c];
    let left = grid[r][c - 1];
    let right = grid[r][c + 1];

    let starting_tile = if up == Tile::Vertical || up == Tile::SouthEast || up == Tile::SouthWest {
        if left == Tile::Horizontal || left == Tile::SouthEast || left == Tile::NorthEast {
            Tile::NorthWest
        } else if right == Tile::Horizontal || right == Tile::SouthWest || right == Tile::NorthWest
        {
            Tile::NorthEast
        } else {
            Tile::Vertical
        }
    } else if down == Tile::Vertical || down == Tile::NorthEast || down == Tile::NorthWest {
        if left == Tile::Horizontal || left == Tile::SouthEast || left == Tile::NorthEast {
            Tile::SouthWest
        } else {
            Tile::SouthEast
        }
    } else {
        Tile::Horizontal
    };

    grid[r][c] = starting_tile;

    ((r, c), grid)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 10: Pipe Maze");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let (start_pos, grid) = parse_input(&raw_input);

    let main_loop = find_main_loop(start_pos, &grid);
    let (perim, area) = measure_loop(&main_loop);

    let part_one = perim / 2;
    let part_two = area + 1 - part_one; // pick's theorem

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
