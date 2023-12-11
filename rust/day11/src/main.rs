use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

struct Data {
    galaxies: Vec<(usize, usize)>,
    row_voids: Vec<bool>,
    col_voids: Vec<bool>,
}

impl Data {
    fn from_str(s: &str) -> Self {
        let field = s
            .trim()
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let num_rows = field.len();
        let num_cols = field[0].len();

        let row_voids = (0..num_rows)
            .map(|r| !(0..num_cols).any(|c| field[r][c]))
            .collect::<Vec<_>>();

        let col_voids = (0..num_cols)
            .map(|c| !(0..num_rows).any(|r| field[r][c]))
            .collect::<Vec<_>>();

        let galaxies = (0..num_rows)
            .flat_map(|r| {
                (0..num_cols)
                    .filter_map(|c| {
                        if field[r][c].clone() {
                            Some((r, c))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            galaxies,
            row_voids,
            col_voids,
        }
    }

    fn get_distances(&self, expansion: usize) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .map(|(i, g_i)| {
                let &(r_i, c_i) = g_i;
                self.galaxies
                    .iter()
                    .skip(i)
                    .map(|g_j| {
                        let &(r_j, c_j) = g_j;

                        let (r_min, r_max) = if r_i < r_j { (r_i, r_j) } else { (r_j, r_i) };
                        let (c_min, c_max) = if c_i < c_j { (c_i, c_j) } else { (c_j, c_i) };

                        let d_r = (r_max - r_min)
                            + (expansion - 1)
                                * (r_min..=r_max).filter(|&rr| self.row_voids[rr]).count();
                        let d_c = (c_max - c_min)
                            + (expansion - 1)
                                * (c_min..=c_max).filter(|&cc| self.col_voids[cc]).count();

                        d_r + d_c
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 11: Cosmic Expansion");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let data = Data::from_str(&raw_input);

    let part_one = data.get_distances(2);
    let part_two = data.get_distances(1000000);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
