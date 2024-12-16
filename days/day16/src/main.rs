use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    isize, usize,
};

use common::read_input;

const TURN_COST: usize = 1000;
const MOVE_COST: usize = 1;

#[derive(Debug)]
enum Tile {
    Floor,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Floor,
            '#' => Self::Wall,
            c => panic!("Unknown tile {c}"),
        }
    }
}

impl Tile {
    fn is_walkable(&self) -> bool {
        match self {
            Tile::Floor => true,
            Tile::Wall => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<(isize, isize)> for Direction {
    fn from(value: (isize, isize)) -> Self {
        match value {
            (-1, 0) => Self::West,
            (1, 0) => Self::East,
            (0, -1) => Self::North,
            (0, 1) => Self::South,
            (r, c) => panic!("Unknown direction ({r},{c})"),
        }
    }
}

impl Direction {
    fn cost_to(&self, next: &Direction) -> usize {
        match (self, next) {
            (Direction::North, Direction::North) => 0,
            (Direction::North, Direction::East) => TURN_COST,
            (Direction::North, Direction::South) => 2 * TURN_COST,
            (Direction::North, Direction::West) => TURN_COST,
            (Direction::East, Direction::North) => TURN_COST,
            (Direction::East, Direction::East) => 0,
            (Direction::East, Direction::South) => TURN_COST,
            (Direction::East, Direction::West) => 2 * TURN_COST,
            (Direction::South, Direction::North) => 2 * TURN_COST,
            (Direction::South, Direction::East) => TURN_COST,
            (Direction::South, Direction::South) => 0,
            (Direction::South, Direction::West) => TURN_COST,
            (Direction::West, Direction::North) => TURN_COST,
            (Direction::West, Direction::East) => 2 * TURN_COST,
            (Direction::West, Direction::South) => TURN_COST,
            (Direction::West, Direction::West) => 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    position: (usize, usize),
    direction: Direction,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(position: (usize, usize), cost: usize) -> Self {
        Self {
            position,
            direction: Direction::East,
            cost,
        }
    }

    fn to(&self, (col, row): (isize, isize)) -> Self {
        let direction = (col, row).into();
        let cost = self.direction.cost_to(&direction) + MOVE_COST;
        let position = (
            (self.position.0 as isize + col) as usize,
            (self.position.1 as isize + row) as usize,
        );
        Self {
            position,
            direction,
            cost: self.cost + cost,
        }
    }
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Vec<Tile>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let tiles = value
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        'S' => {
                            start = (col, row);
                            '.'.into()
                        }
                        'E' => {
                            end = (col, row);
                            '.'.into()
                        }
                        c => c.into(),
                    })
                    .collect()
            })
            .collect();
        Self { tiles, start, end }
    }
}

impl Maze {
    fn heuristic(&self, (col, row): (usize, usize)) -> usize {
        self.end.0 - col + self.end.1 - row
    }

    fn find_lowest_score(&self) -> usize {
        let initial_state = State::new(self.start, self.heuristic(self.start));
        let mut priority_queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
        priority_queue.push(Reverse(initial_state));
        let mut visited = HashSet::new();

        while let Some(Reverse(state)) = priority_queue.pop() {
            if state.position == self.end {
                return state.cost;
            }
            if visited.contains(&(state.position, state.direction.clone())) {
                continue;
            }
            visited.insert((state.position, state.direction.clone()));

            self.valid_neighbors(&state).iter().for_each(|neighbor| {
                priority_queue.push(Reverse(neighbor.clone()));
            });
        }
        unreachable!("Destination is unreachable");
    }

    fn tile_at(&self, (col, row): (usize, usize)) -> &Tile {
        &self.tiles[row][col]
    }

    fn valid_neighbors(&self, state: &State) -> Vec<State> {
        let mut deltas = vec![];
        if state.position.0 > 0 {
            deltas.push((-1, 0));
        }
        if state.position.0 < self.tiles[0].len() - 1 {
            deltas.push((1, 0));
        }
        if state.position.1 > 0 {
            deltas.push((0, -1));
        }
        if state.position.1 < self.tiles.len() - 1 {
            deltas.push((0, 1));
        }
        deltas
            .iter()
            .map(|delta| state.to(*delta))
            .filter(|new_state| self.tile_at(new_state.position).is_walkable())
            .collect()
    }
}

fn main() {
    let input = read_input("day16.txt");
    let maze = Maze::from(input.as_str());
    println!("Part 1 = {}", maze.find_lowest_score());
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn test_find_lowest_score() {
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;
        let maze = Maze::from(input);
        assert_eq!(maze.find_lowest_score(), 7036);
    }

    #[test]
    fn part1() {
        let input = r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"#;
        let maze = Maze::from(input);
        assert_eq!(maze.find_lowest_score(), 11048);
    }
}
