use std::collections::HashMap;

use common::read_input;

#[derive(Debug, Ord, Eq, PartialOrd)]
struct Cheat {
    start: (usize, usize),
    end: (usize, usize),
}

impl PartialEq for Cheat {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }
}

impl Cheat {
    fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
struct Race {
    _walls: Vec<(usize, usize)>,
    track: Vec<(usize, usize)>,
    cols: usize,
    rows: usize,
}

fn neighbors((col, row): (usize, usize), cols: usize, rows: usize) -> Vec<(usize, usize)> {
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
                ((col as isize) + delta_col) as usize,
                ((row as isize) + delta_row) as usize,
            )
        })
        .collect()
}

fn distance((from_col, from_row): (usize, usize), (to_col, to_row): (usize, usize)) -> usize {
    let delta_col = if to_col >= from_col {
        to_col - from_col
    } else {
        from_col - to_col
    };
    let delta_row = if to_row >= from_row {
        to_row - from_row
    } else {
        from_row - to_row
    };
    delta_col + delta_row
}

impl From<&str> for Race {
    fn from(value: &str) -> Self {
        let (walls, start, cols, rows) = value.trim().lines().enumerate().fold(
            (Vec::new(), (0, 0), 0, 0),
            |(walls, start, cols, rows), (row, l)| {
                l.trim().chars().enumerate().fold(
                    (walls, start, cols, rows),
                    |(mut walls, mut start, mut cols, mut rows), (col, tile)| {
                        match tile {
                            '.' | 'E' => (),
                            '#' => walls.push((col, row)),
                            'S' => {
                                start = (col, row);
                            }
                            c => panic!("Unknown tile {c}"),
                        }
                        if col > cols {
                            cols = col
                        }
                        if row > rows {
                            rows = row;
                        }
                        (walls, start, cols, rows)
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
            _walls: walls,
            track,
            cols,
            rows,
        }
    }
}

impl Race {
    fn cheats(&self, (from_col, from_row): (usize, usize), duration: usize) -> Vec<Cheat> {
        let duration: isize = duration as isize;
        (-duration as isize..=duration)
            .flat_map(|delta_row| {
                (-duration as isize..=duration)
                    .filter_map(|delta_col| {
                        if (delta_col, delta_row) != (-1, 0)
                            && (delta_col, delta_row) != (1, 0)
                            && (delta_col, delta_row) != (0, -1)
                            && (delta_col, delta_row) != (0, 1)
                            && (delta_col, delta_row) != (0, 0)
                        {
                            Some((delta_col, delta_row))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .map(|(delta_col, delta_row)| {
                (from_col as isize + delta_col, from_row as isize + delta_row)
            })
            .filter_map(|(col, row)| {
                if col >= 0 && row >= 0 {
                    Some((col as usize, row as usize))
                } else {
                    None
                }
            })
            .filter(|(col, row)| *col < self.cols && *row < self.rows)
            .filter(|tile| distance((from_col, from_row), *tile) <= duration as usize)
            .filter(|tile| self.track.contains(tile))
            .filter(|&tile| {
                self.track.iter().position(|&t| t == (from_col, from_row))
                    < self.track.iter().position(|&t| t == tile)
            })
            .map(|tile| Cheat::new((from_col, from_row), tile))
            .collect()
    }

    fn find_valid_cheats(&self, duration: usize) -> HashMap<usize, usize> {
        let mut cheats = self
            .track
            .iter()
            .flat_map(|&tile| self.cheats(tile, duration))
            .collect::<Vec<_>>();
        cheats.sort_unstable();
        cheats.dedup();
        cheats
            .iter()
            .fold(HashMap::new(), |mut cheat_saving, cheat| {
                let (start, end) = (cheat.start, cheat.end);
                let idx_start = self.track.iter().position(|&tile| tile == start).unwrap();
                let idx_end = self.track.iter().position(|&tile| tile == end).unwrap();
                let saved_ps = if idx_start > idx_end {
                    idx_start - idx_end - distance(start, end)
                } else {
                    idx_end - idx_start - distance(start, end)
                };
                if saved_ps > 0 {
                    cheat_saving
                        .entry(saved_ps)
                        .and_modify(|num_tracks| *num_tracks += 1)
                        .or_insert(1);
                }
                cheat_saving
            })
    }
}

fn main() {
    let input = read_input("day20.txt");
    let race = Race::from(input.as_str());
    let saved_ps = race.find_valid_cheats(2);
    println!(
        "Part 1 = {}",
        saved_ps
            .iter()
            .filter(|(&ps, _)| ps >= 100)
            .map(|(_, num)| num)
            .sum::<usize>()
    );
    let saved_ps = race.find_valid_cheats(20);
    println!(
        "Part 2 = {}",
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
        assert_eq!(race.find_valid_cheats(2).values().sum::<usize>(), 44);
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
        let saved_ps = race.find_valid_cheats(2);
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

    #[test]
    fn part2() {
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
        let saved_ps = race.find_valid_cheats(20);
        assert_eq!(saved_ps.get(&50).unwrap(), &32);
        assert_eq!(saved_ps.get(&52).unwrap(), &31);
        assert_eq!(saved_ps.get(&54).unwrap(), &29);
        assert_eq!(saved_ps.get(&56).unwrap(), &39);
        assert_eq!(saved_ps.get(&58).unwrap(), &25);
        assert_eq!(saved_ps.get(&60).unwrap(), &23);
        assert_eq!(saved_ps.get(&62).unwrap(), &20);
        assert_eq!(saved_ps.get(&64).unwrap(), &19);
        assert_eq!(saved_ps.get(&66).unwrap(), &12);
        assert_eq!(saved_ps.get(&68).unwrap(), &14);
        assert_eq!(saved_ps.get(&70).unwrap(), &12);
        assert_eq!(saved_ps.get(&72).unwrap(), &22);
        assert_eq!(saved_ps.get(&74).unwrap(), &4);
        assert_eq!(saved_ps.get(&76).unwrap(), &3);
    }
}
