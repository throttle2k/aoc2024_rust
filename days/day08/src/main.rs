use std::isize;

use common::read_input;

#[derive(Debug, Clone)]
struct Antenna {
    row: usize,
    col: usize,
    frequency: char,
}

impl PartialEq for Antenna {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Antenna {
    fn new(row: usize, col: usize, frequency: char) -> Self {
        Antenna {
            row,
            col,
            frequency,
        }
    }

    fn antinode(
        &self,
        other: &Antenna,
        with_resonance: bool,
        rows: usize,
        cols: usize,
    ) -> Vec<(usize, usize)> {
        let delta_col = other.col as isize - self.col as isize;
        let delta_row = other.row as isize - self.row as isize;
        let start = if with_resonance { 0 } else { 1 };
        (start..)
            .map(|i| (i * delta_row, i * delta_col))
            .map(|(delta_row, delta_col)| {
                (
                    other.row as isize + delta_row,
                    other.col as isize + delta_col,
                )
            })
            .enumerate()
            .take_while(|(i, (antinode_row, antinode_col))| {
                let in_bounds = *antinode_col >= 0
                    && *antinode_col < cols as isize
                    && *antinode_row >= 0
                    && *antinode_row < rows as isize;
                if with_resonance {
                    in_bounds
                } else {
                    in_bounds && *i == 0
                }
            })
            .map(|(_, (antinode_row, antinode_col))| (antinode_row as usize, antinode_col as usize))
            .collect::<Vec<(usize, usize)>>()
    }
}

#[derive(Debug)]
struct Roof {
    antennas: Vec<Antenna>,
    rows: usize,
    cols: usize,
}

impl From<&str> for Roof {
    fn from(value: &str) -> Self {
        let cols = value.lines().nth(0).unwrap().len();
        let rows = value.lines().count();
        let antennas = value
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                    .map(|(col, frequency)| Antenna::new(row, col, frequency))
                    .collect::<Vec<_>>()
            })
            .collect();
        Roof {
            antennas,
            rows,
            cols,
        }
    }
}

impl Roof {
    fn get_pairs(&self) -> Vec<(Antenna, Antenna)> {
        self.antennas
            .iter()
            .flat_map(|a| {
                vec![a.clone(); self.antennas.len()]
                    .into_iter()
                    .zip(self.antennas.clone())
                    .collect::<Vec<(Antenna, Antenna)>>()
            })
            .filter(|(a0, a1)| a0.frequency == a1.frequency && *a0 != *a1)
            .map(|(a0, a1)| (a0.clone(), a1))
            .collect()
    }

    fn find_antinodes(&self, with_resonance: bool) -> Vec<(usize, usize)> {
        let mut antinodes = self
            .get_pairs()
            .iter()
            .flat_map(|(antenna1, antenna2)| {
                antenna1.antinode(antenna2, with_resonance, self.rows, self.cols)
            })
            .collect::<Vec<_>>();
        antinodes.sort();
        antinodes.dedup();
        antinodes
    }
}

fn main() {
    let input = read_input("day08.txt");
    let roof = Roof::from(input.as_ref());
    println!("Part 1 = {}", roof.find_antinodes(false).len());
    println!("Part 2 = {}", roof.find_antinodes(true).len());
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    #[test]
    fn test_antinode() {
        let antenna1 = Antenna::new(3, 4, 'a');
        let antenna2 = Antenna::new(5, 5, 'a');
        assert_eq!(
            &(7, 6),
            antenna1.antinode(&antenna2, false, 10, 10).first().unwrap()
        );
        assert_eq!(
            &(1, 3),
            antenna2.antinode(&antenna1, false, 10, 10).first().unwrap()
        );
    }

    #[test]
    fn test_antinode_2() {
        let antenna1 = Antenna::new(3, 4, 'a');
        let antenna2 = Antenna::new(5, 5, 'a');
        let antenna3 = Antenna::new(4, 8, 'a');
        assert_eq!(
            &(7, 6),
            antenna1.antinode(&antenna2, false, 10, 10).first().unwrap()
        );
        assert_eq!(
            &(1, 3),
            antenna2.antinode(&antenna1, false, 10, 10).first().unwrap()
        );
        assert_eq!(
            &(2, 0),
            antenna3.antinode(&antenna1, false, 10, 10).first().unwrap()
        );
        assert_eq!(
            &(6, 2),
            antenna3.antinode(&antenna2, false, 10, 10).first().unwrap()
        );
    }

    #[test]
    fn test_get_pairs() {
        let antenna1 = Antenna::new(3, 4, 'a');
        let antenna2 = Antenna::new(5, 5, 'a');
        let antenna3 = Antenna::new(4, 8, 'a');
        let roof = Roof {
            antennas: vec![antenna1.clone(), antenna2.clone(), antenna3.clone()],
            rows: 10,
            cols: 10,
        };
        let pairs = roof.get_pairs();
        assert_eq!(pairs.len(), 6);
        assert!(pairs.contains(&(antenna1.clone(), antenna2.clone())));
        assert!(pairs.contains(&(antenna1.clone(), antenna3.clone())));
        assert!(pairs.contains(&(antenna2.clone(), antenna1.clone())));
        assert!(pairs.contains(&(antenna2.clone(), antenna3.clone())));
        assert!(pairs.contains(&(antenna3.clone(), antenna1.clone())));
        assert!(pairs.contains(&(antenna3.clone(), antenna2.clone())));
    }

    #[test]
    fn test_find_antinodes() {
        let antenna1 = Antenna::new(3, 4, 'a');
        let antenna2 = Antenna::new(5, 5, 'a');
        let antenna3 = Antenna::new(4, 8, 'a');
        let roof = Roof {
            antennas: vec![antenna1, antenna2, antenna3],
            rows: 10,
            cols: 10,
        };
        let antinodes = roof.find_antinodes(false);
        assert_eq!(antinodes.len(), 4);
        assert!(antinodes.contains(&(7, 6)));
        assert!(antinodes.contains(&(1, 3)));
        assert!(antinodes.contains(&(2, 0)));
        assert!(antinodes.contains(&(6, 2)));
    }

    #[test]
    fn part1() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        let roof = Roof::from(input);
        let antinodes = roof.find_antinodes(false);
        assert_eq!(antinodes.len(), 14);
    }

    #[test]
    fn part2() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        let roof = Roof::from(input);
        let antinodes = roof.find_antinodes(true);
        assert_eq!(antinodes.len(), 34);
    }
}
