use std::collections::HashMap;

use common::read_input;

#[derive(Debug, PartialEq)]
struct Cheat {
    start: (usize, usize),
    end: (usize, usize),
}

impl Cheat {
    fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
struct Race {
    _start: (usize, usize),
    _end: (usize, usize),
    walls: Vec<(usize, usize)>,
    track: Vec<(usize, usize)>,
    cols: usize,
    rows: usize,
}

fn neighbors((row, col): (usize, usize), cols: usize, rows: usize) -> Vec<(usize, usize)> {
    let mut deltas: Vec<(isize, isize)> = vec![];
    if col > 0 {
        deltas.push((-1, 0));
    }
    if col < cols - 1 {
        deltas.push((1, 0));
    }
    if row > 0 {
        deltas.push((0, -1));
    }
    if row < rows - 1 {
        deltas.push((0, 1));
    }
    deltas
        .iter()
        .map(|(delta_col, delta_row)| {
            (
                ((row as isize) + delta_row) as usize,
                ((col as isize) + delta_col) as usize,
            )
        })
        .collect()
}

impl From<&str> for Race {
    fn from(value: &str) -> Self {
        let (walls, start, end, cols, rows) = value.trim().lines().enumerate().fold(
            (Vec::new(), (0, 0), (0, 0), 0, 0),
            |(walls, start, end, cols, rows), (row, l)| {
                l.trim().chars().enumerate().fold(
                    (walls, start, end, cols, rows),
                    |(mut walls, mut start, mut end, mut cols, mut rows), (col, tile)| {
                        match tile {
                            '.' => (),
                            '#' => walls.push((col, row)),
                            'S' => {
                                start = (col, row);
                            }
                            'E' => {
                                end = (col, row);
                            }
                            c => panic!("Unknown tile {c}"),
                        }
                        if col > cols {
                            cols = col
                        }
                        if row > rows {
                            rows = row;
                        }
                        (walls, start, end, cols, rows)
                    },
                )
            },
        );

        let mut queue = vec![start];
        let mut track = vec![start];
        while !queue.is_empty() {
            let current = queue.remove(0);
            neighbors(current, cols, rows)
                .iter()
                .filter(|next| !walls.contains(next))
                .for_each(|next| {
                    if !track.contains(next) {
                        queue.push(*next);
                        track.push(*next);
                    }
                });
        }

        Self {
            walls,
            track,
            _start: start,
            _end: end,
            cols,
            rows,
        }
    }
}

impl Race {
    fn valid_cheat(&self, (wall_col, wall_row): &(usize, usize)) -> Vec<Cheat> {
        let horizontal_cheat = if !self.walls.contains(&(wall_col - 1, *wall_row))
            && !self.walls.contains(&(wall_col + 1, *wall_row))
        {
            Some(Cheat::new(
                (wall_col - 1, *wall_row),
                (wall_col + 1, *wall_row),
            ))
        } else {
            None
        };
        let vertical_cheat = if !self.walls.contains(&(*wall_col, wall_row - 1))
            && !self.walls.contains(&(*wall_col, wall_row + 1))
        {
            Some(Cheat::new(
                (*wall_col, wall_row - 1),
                (*wall_col, wall_row + 1),
            ))
        } else {
            None
        };
        let result = vec![horizontal_cheat, vertical_cheat];
        result.into_iter().filter_map(|cheat| cheat).collect()
    }

    fn find_valid_cheats(&self) -> HashMap<usize, usize> {
        let cheats = self
            .walls
            .iter()
            .filter(|(col, row)| *col > 0 && *col < self.cols && *row > 0 && *row < self.rows)
            .flat_map(|tile| self.valid_cheat(tile))
            .collect::<Vec<_>>();
        cheats
            .iter()
            .fold(HashMap::new(), |mut cheat_saving, cheat| {
                let (start, end) = (cheat.start, cheat.end);
                let idx_start = self.track.iter().position(|&tile| tile == start).unwrap();
                let idx_end = self.track.iter().position(|&tile| tile == end).unwrap();
                let saved_ps = if idx_start > idx_end {
                    idx_start - idx_end - 2
                } else {
                    idx_end - idx_start - 2
                };
                cheat_saving
                    .entry(saved_ps)
                    .and_modify(|num_tracks| *num_tracks += 1)
                    .or_insert(1);
                cheat_saving
            })
    }
}

fn main() {
    let input = read_input("day20.txt");
    let race = Race::from(input.as_str());
    let saved_ps = race.find_valid_cheats();
    println!(
        "Part 1 = {}",
        saved_ps
            .iter()
            .filter(|(&ps, _)| ps >= 100)
            .map(|(_, num)| num)
            .sum::<usize>()
    );
}

#[cfg(test)]
mod day20_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let race = Race::from(input);
        assert_eq!(race.track.len() - 1, 84);
    }

    #[test]
    fn test_valid_cheats() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let race = Race::from(input);
        assert_eq!(
            race.valid_cheat(&(4, 1)),
            vec![Cheat {
                start: (3, 1),
                end: (5, 1)
            }]
        );
        assert_eq!(race.valid_cheat(&(10, 2)), vec![]);
        assert_eq!(
            race.valid_cheat(&(8, 13)),
            vec![Cheat {
                start: (7, 13),
                end: (9, 13)
            }]
        );
        assert_eq!(
            race.valid_cheat(&(8, 3)),
            vec![Cheat {
                start: (7, 3),
                end: (9, 3)
            }]
        );
        assert_eq!(
            race.valid_cheat(&(3, 10)),
            vec![Cheat {
                start: (3, 9),
                end: (3, 11)
            }]
        );
        assert_eq!(
            race.valid_cheat(&(10, 3)),
            vec![Cheat {
                start: (9, 3),
                end: (11, 3)
            }]
        );
    }

    #[test]
    fn test_find_valid_cheats() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let race = Race::from(input);
        assert_eq!(race.find_valid_cheats().values().sum::<usize>(), 44);
    }

    #[test]
    fn part1() {
        let input = r#"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"#;
        let race = Race::from(input);
        let saved_ps = race.find_valid_cheats();
        assert_eq!(saved_ps.get(&2).unwrap(), &14);
        assert_eq!(saved_ps.get(&4).unwrap(), &14);
        assert_eq!(saved_ps.get(&6).unwrap(), &2);
        assert_eq!(saved_ps.get(&8).unwrap(), &4);
        assert_eq!(saved_ps.get(&10).unwrap(), &2);
        assert_eq!(saved_ps.get(&12).unwrap(), &3);
        assert_eq!(saved_ps.get(&20).unwrap(), &1);
        assert_eq!(saved_ps.get(&36).unwrap(), &1);
        assert_eq!(saved_ps.get(&38).unwrap(), &1);
        assert_eq!(saved_ps.get(&40).unwrap(), &1);
        assert_eq!(saved_ps.get(&64).unwrap(), &1);
    }
}
