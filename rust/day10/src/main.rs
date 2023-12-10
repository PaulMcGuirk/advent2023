use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn move_(&self, dir: &Direction) -> Self {
        let (row, col) = match dir {
            Direction::North => (self.row - 1, self.col),
            Direction::South => (self.row + 1, self.col),
            Direction::East => (self.row, self.col + 1),
            Direction::West => (self.row, self.col - 1),
        };
        Self { row, col }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

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

fn find_loop(initial_pos: &Position, grid: &Vec<Vec<Tile>>) -> Vec<Position> {
    let mut pos = Position {
        row: initial_pos.row,
        col: initial_pos.col,
    };

    let mut dist = 0;
    let mut dir = match grid[pos.row][pos.col] {
        Tile::Vertical | Tile::SouthEast | Tile::SouthWest => Direction::South,
        Tile::NorthEast | Tile::NorthWest => Direction::North,
        Tile::Horizontal => Direction::East,
        Tile::Ground => panic!(),
    };
    let mut loop_ = vec![];

    loop {
        println!("pos = {} {}", pos.row, pos.col);
        pos = pos.move_(&dir);
        println!("new pos = {} {}", pos.row, pos.col);
        loop_.push(pos.clone());
        if pos.row == initial_pos.row && pos.col == initial_pos.col {
            break;
        }

        let new_tile = grid[pos.row][pos.col];
        println!("new tile = {:?}", new_tile);
        dir = match new_tile {
            Tile::Vertical => dir,
            Tile::Horizontal => dir,
            Tile::NorthEast => {
                if dir == Direction::South {
                    Direction::East
                } else {
                    Direction::North
                }
            }
            Tile::NorthWest => {
                if dir == Direction::South {
                    Direction::West
                } else {
                    Direction::North
                }
            }
            Tile::SouthEast => {
                if dir == Direction::North {
                    Direction::East
                } else {
                    Direction::South
                }
            }
            Tile::SouthWest => {
                if dir == Direction::North {
                    Direction::West
                } else {
                    Direction::South
                }
            }
            _ => panic!(),
        };
    }

    loop_
}

fn area_enclosed(loop_: &Vec<Position>, grid: &Vec<Vec<Tile>>) -> u32 {
    let loop_points = loop_
        .iter()
        .map(|pos| (pos.row, pos.col))
        .collect::<HashSet<_>>();

    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut area = 0;
    let mut last_bend = None;
    for r in 0..num_rows {
        let mut in_grid = false;
        for c in 0..num_cols {
            let mut desp = '.';
            if loop_points.contains(&(r, c)) {
                match grid[r][c] {
                    Tile::Vertical => {
                        in_grid = !in_grid;
                        last_bend = None;
                    }
                    Tile::NorthWest => {
                        if let Some(Tile::SouthEast) = last_bend {
                            in_grid = !in_grid;
                        }
                        last_bend = None;
                    }
                    Tile::SouthWest => {
                        if let Some(Tile::NorthEast) = last_bend {
                            in_grid = !in_grid;
                        }
                        last_bend = None;
                    }
                    Tile::NorthEast => {
                        last_bend = Some(Tile::NorthEast);
                    }
                    Tile::SouthEast => {
                        last_bend = Some(Tile::SouthEast);
                    }
                    Tile::Horizontal => (),
                    Tile::Ground => panic!(),
                }
                // in_grid = match grid[r][c] {
                //     Tile::Horizontal => in_grid,
                //     Tile::NorthWest => {
                //         let toggle = match last_bend {
                //             None => in_grid,
                //             Some(Tile::NorthEast) => in_grid,
                //             Some(Tile::SouthEast) => !in_grid,
                //             _ => panic!()
                //         };
                //         last_bend = None;
                //         toggle
                //     },
                //     Tile::SouthWest => {
                //         let toggle = match last_bend {
                //             None => in_grid,
                //             Some(Tile::SouthEast) => in_grid,
                //             Some(Tile::NorthEast) => !in_grid,
                //             _ => panic!()
                //         };
                //         last_bend = None;
                //         toggle
                //     }

                //      | Tile::NorthWest | Tile::SouthWest => in_grid,
                //     _ => !in_grid,
                desp = '█';
            } else if in_grid {
                // println!("{} {}", r, c);
                area += 1;
                desp = 'I';
            }
            print!("{}", desp);
        }
        println!("");
    }

    area
}

fn parse_input(raw_input: &str) -> (Position, Vec<Vec<Tile>>) {
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
                    println!("{} {}", r, c);
                    starting_pos = Some(Position {
                        row: r + 1,
                        col: c + 1,
                    });
                    Tile::Ground // we'll fix below
                }
                _ => panic!(),
            };
            row.push(tile);
        }
        row.push(Tile::Ground);
        grid.push(row);
    }

    grid.push(vec![Tile::Ground; num_cols + 2]);

    let starting_pos = starting_pos.unwrap();
    let up = grid[starting_pos.row - 1][starting_pos.col];
    let down = grid[starting_pos.row + 1][starting_pos.col];
    let left = grid[starting_pos.row][starting_pos.col - 1];
    let right = grid[starting_pos.row][starting_pos.col + 1];

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

    grid[starting_pos.row][starting_pos.col] = starting_tile;

    (starting_pos, grid)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 10: Pipe Maze");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    //     // let raw_input = "-L|F7
    //     // 7S-7|
    //     // L|7||
    //     // -L-J|
    //     // L|-JF";
    //     // let raw_input = "7-F7-
    //     // .FJ|7
    //     // SJLL7
    //     // |F--J
    //     // LJ.LJ";
    //     let raw_input = "...........
    // .S-------7.
    // .|F-----7|.
    // .||.....||.
    // .||.....||.
    // .|L-7.F-J|.
    // .|..|.|..|.
    // .L--J.L--J.
    // ...........";
    //     let raw_input = "FF7FSF7F7F7F7F7F---7
    // L|LJ||||||||||||F--J
    // FL-7LJLJ||||||LJL-77
    // F--JF--7||LJLJ7F7FJ-
    // L---JF-JLJ.||-FJLJJ7
    // |F|F-JF---7F7-L7L|7|
    // |FFJF7L7F-JF7|JL---7
    // 7-L-JL7||F7|L7F-7F7|
    // L.L7LFJ|||||FJL7||LJ
    // L7JLJL-JLJLJL--JLJ.L";
    let (start_pos, grid) = parse_input(&raw_input);
    println!("{:?}", grid[start_pos.row][start_pos.col]);

    let loop_ = find_loop(&start_pos, &grid);
    let part_one = loop_.len() / 2;
    let part_two = area_enclosed(&loop_, &grid);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
