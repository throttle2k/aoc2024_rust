use core::panic;
use std::usize;

use common::read_input;

type Position = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
enum Cell {
    Floor,
    Obstruction,
}

#[derive(Debug)]
struct LabMap {
    rows: usize,
    cols: usize,
    grid: Vec<Cell>,
}

impl LabMap {
    fn cell_at(&self, (row, col): Position) -> &Cell {
        self.grid.get(row * self.cols + col).unwrap()
    }

    fn block_cell_at(&self, (row, col): Position) -> Self {
        let idx = row * self.cols + col;
        let new_grid = self
            .grid
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, cell)| if i == idx { Cell::Obstruction } else { cell })
            .collect::<Vec<_>>();
        Self {
            grid: new_grid,
            rows: self.rows,
            cols: self.cols,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn step(&self, map: &LabMap) -> Option<Self> {
        let next_position = match self.direction {
            Direction::North => (self.position.0 as isize - 1, self.position.1 as isize),
            Direction::East => (self.position.0 as isize, self.position.1 as isize + 1),
            Direction::South => (self.position.0 as isize + 1, self.position.1 as isize),
            Direction::West => (self.position.0 as isize, self.position.1 as isize - 1),
        };
        if next_position.0 < 0
            || next_position.0 as usize >= map.rows
            || next_position.1 < 0
            || next_position.1 as usize >= map.cols
        {
            return None;
        }
        let next_position = (next_position.0 as usize, next_position.1 as usize);
        match map.cell_at(next_position) {
            Cell::Floor => Some(Self {
                position: next_position,
                direction: self.direction.clone(),
            }),
            Cell::Obstruction => Some(Self {
                position: self.position.clone(),
                direction: match self.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::South,
                    Direction::South => Direction::West,
                    Direction::West => Direction::North,
                },
            }),
        }
    }

    fn walk(&self, map: &LabMap) -> Vec<Position> {
        let mut positions = vec![self.position];
        let mut guard_positions = vec![self.clone()];
        let mut current = self.clone();
        while let Some(guard) = current.step(map) {
            current = guard.clone();
            if !positions.contains(&guard.position) {
                positions.push(guard.position);
            }
            if guard_positions.contains(&guard) {
                return vec![];
            } else {
                guard_positions.push(guard.clone());
            }
        }
        positions
    }

    fn count_loops(&self, map: &LabMap) -> usize {
        let path = self.walk(&map);
        path.iter()
            .filter_map(|(row, col)| {
                if (row, col) != (&self.position.0, &self.position.1)
                    && matches!(map.cell_at((*row, *col)), Cell::Floor)
                {
                    let new_map = map.block_cell_at((*row, *col));
                    if self.walk(&new_map).len() > 0 {
                        None
                    } else {
                        Some(1)
                    }
                } else {
                    None
                }
            })
            .sum()
    }
}

fn parse_input(input: &str) -> (LabMap, Guard) {
    let (cells, position, row) = input.lines().fold(
        (
            Vec::<Cell>::new(),
            (usize::max_value(), usize::max_value()),
            0,
        ),
        |(mut cells, mut position, row), l| {
            l.trim().chars().enumerate().for_each(|(col, c)| match c {
                '.' => {
                    cells.push(Cell::Floor);
                }
                '#' => {
                    cells.push(Cell::Obstruction);
                }
                '^' => {
                    cells.push(Cell::Floor);
                    position = (row, col);
                }
                c => panic!("Unknown character in map: {c}"),
            });
            (cells, position, row + 1)
        },
    );
    (
        LabMap {
            rows: row,
            cols: input.lines().nth(0).unwrap().len(),
            grid: cells,
        },
        Guard {
            position,
            direction: Direction::North,
        },
    )
}

fn main() {
    let input = read_input("day06.txt");
    let (map, guard) = parse_input(&input);
    println!("Part 1 = {}", guard.walk(&map).iter().count());
    println!("Part 2 = {}", guard.count_loops(&map));
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let (map, guard) = parse_input(input);
        assert_eq!(guard.walk(&map).iter().count(), 41);
    }

    #[test]
    fn part2() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        let (map, guard) = parse_input(input);
        assert_eq!(guard.count_loops(&map), 6);
    }
}
