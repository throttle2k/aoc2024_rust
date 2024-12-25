use std::{collections::HashMap, usize};

use common::read_input;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Press,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Press => panic!("Cannot undo press"),
        }
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => String::from("^"),
            Direction::Right => String::from(">"),
            Direction::Down => String::from("v"),
            Direction::Left => String::from("<"),
            Direction::Press => String::from("A"),
        }
    }
}

#[derive(Debug)]
struct Edge {
    from: char,
    to: char,
    direction: Direction,
}

impl Edge {
    fn new(from: char, to: char, direction: Direction) -> Self {
        Self {
            from,
            to,
            direction,
        }
    }
}

fn compute_min_paths(edges: &[Edge]) -> HashMap<(char, char), Vec<Vec<Direction>>> {
    let mut min_paths = HashMap::new();
    let keys = edges.iter().map(|e| e.from).collect::<Vec<_>>();
    keys.iter()
        .cartesian_product(keys.clone())
        .for_each(|(from, to)| {
            let path = compute_min_path(*from, to, edges, &min_paths);
            let reverse_path = path
                .iter()
                .filter(|p| !p.is_empty())
                .map(|p| {
                    p.iter()
                        .cloned()
                        .rev()
                        .map(|d| d.reverse())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            min_paths.insert((*from, to), path);
            min_paths.insert((to, *from), reverse_path);
            min_paths.insert((*from, *from), vec![vec![]]);
            min_paths.insert((to, to), vec![vec![]]);
        });

    min_paths
}

fn neighbors(key: char, edges: &[Edge]) -> Vec<&Edge> {
    edges.iter().filter(|e| e.from == key).collect()
}

fn compute_min_path(
    from: char,
    to: char,
    edges: &[Edge],
    memo: &HashMap<(char, char), Vec<Vec<Direction>>>,
) -> Vec<Vec<Direction>> {
    let mut result = vec![vec![]];
    let mut queue = vec![(vec![], from)];
    let mut min_length = usize::max_value();

    while !queue.is_empty() {
        let (current_directions, current_key) = queue.remove(0);
        if current_directions.len() > min_length {
            continue;
        }
        if let Some(directions) = memo.get(&(current_key, to)) {
            let mut paths: Vec<Vec<Direction>> = directions
                .iter()
                .cloned()
                .map(|old_path| {
                    let mut new_directions = current_directions.clone();
                    new_directions.extend(old_path);
                    new_directions
                })
                .collect::<Vec<_>>();
            let paths_min_length = paths
                .iter()
                .map(|d| d.len())
                .min()
                .unwrap_or(usize::max_value());
            if paths_min_length < min_length {
                min_length = paths_min_length;
                result.clear();
            }
            paths = paths
                .iter()
                .cloned()
                .filter(|d| d.len() == paths_min_length)
                .collect();
            result.extend(paths);
            continue;
        }
        if current_key == to {
            if current_directions.len() < min_length {
                min_length = current_directions.len();
                result.clear();
            }
            result.push(current_directions);
            continue;
        }
        neighbors(current_key, edges).iter().for_each(|neighbor| {
            let mut next_directions = current_directions.clone();
            next_directions.push(neighbor.direction.clone());
            let next_key = neighbor.to.clone();
            queue.push((next_directions, next_key));
        });
    }
    result
}

#[derive(Debug)]
struct Keypad {
    min_paths: HashMap<(char, char), Vec<Vec<Direction>>>,
}

impl Keypad {
    fn new(edges: Vec<Edge>) -> Self {
        let min_paths = compute_min_paths(&edges);
        Self { min_paths }
    }

    fn new_numeric() -> Self {
        let mut edges = vec![];
        edges.push(Edge::new('7', '8', Direction::Right));
        edges.push(Edge::new('7', '4', Direction::Down));
        edges.push(Edge::new('8', '7', Direction::Left));
        edges.push(Edge::new('8', '5', Direction::Down));
        edges.push(Edge::new('8', '9', Direction::Right));
        edges.push(Edge::new('9', '8', Direction::Left));
        edges.push(Edge::new('9', '6', Direction::Down));
        edges.push(Edge::new('4', '7', Direction::Up));
        edges.push(Edge::new('4', '5', Direction::Right));
        edges.push(Edge::new('4', '1', Direction::Down));
        edges.push(Edge::new('5', '4', Direction::Left));
        edges.push(Edge::new('5', '8', Direction::Up));
        edges.push(Edge::new('5', '6', Direction::Right));
        edges.push(Edge::new('5', '2', Direction::Down));
        edges.push(Edge::new('6', '5', Direction::Left));
        edges.push(Edge::new('6', '9', Direction::Up));
        edges.push(Edge::new('6', '3', Direction::Down));
        edges.push(Edge::new('1', '4', Direction::Up));
        edges.push(Edge::new('1', '2', Direction::Right));
        edges.push(Edge::new('2', '1', Direction::Left));
        edges.push(Edge::new('2', '5', Direction::Up));
        edges.push(Edge::new('2', '3', Direction::Right));
        edges.push(Edge::new('2', '0', Direction::Down));
        edges.push(Edge::new('3', '2', Direction::Left));
        edges.push(Edge::new('3', '6', Direction::Up));
        edges.push(Edge::new('3', 'A', Direction::Down));
        edges.push(Edge::new('0', '2', Direction::Up));
        edges.push(Edge::new('0', 'A', Direction::Right));
        edges.push(Edge::new('A', '0', Direction::Left));
        edges.push(Edge::new('A', '3', Direction::Up));
        Self::new(edges)
    }

    fn new_directional() -> Self {
        let mut edges = vec![];
        edges.push(Edge::new('^', 'A', Direction::Right));
        edges.push(Edge::new('^', 'v', Direction::Down));
        edges.push(Edge::new('A', '^', Direction::Left));
        edges.push(Edge::new('A', '>', Direction::Down));
        edges.push(Edge::new('<', 'v', Direction::Right));
        edges.push(Edge::new('v', '<', Direction::Left));
        edges.push(Edge::new('v', '^', Direction::Up));
        edges.push(Edge::new('v', '>', Direction::Right));
        edges.push(Edge::new('>', 'v', Direction::Left));
        edges.push(Edge::new('>', 'A', Direction::Up));
        Self::new(edges)
    }

    fn find_shortest_sequence(
        &self,
        target: &str,
        depth: usize,
        memo: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        if let Some(length) = memo.get(&(target.to_string(), depth)) {
            return *length;
        }
        let min_length = format!("A{target}")
            .chars()
            .tuple_windows()
            .map(|(from, to)| {
                let shortest_paths = self.min_paths.get(&(from, to)).unwrap();
                let min_length = match depth {
                    0 => shortest_paths[0].len() + 1,
                    _ => shortest_paths
                        .iter()
                        .cloned()
                        .map(|mut path| {
                            path.push(Direction::Press);
                            let path = path.iter().map(|d| d.to_string()).collect::<String>();
                            Keypad::new_directional().find_shortest_sequence(&path, depth - 1, memo)
                        })
                        .min()
                        .unwrap(),
                };
                min_length
            })
            .sum::<usize>();
        memo.insert((target.to_string(), depth), min_length);
        min_length
    }

    fn calculate_complexity(&self, input: &str, num_robot_stages: usize) -> usize {
        self.find_shortest_sequence(input, num_robot_stages, &mut HashMap::new())
            * input.trim_end_matches('A').parse::<usize>().unwrap()
    }

    fn calculate_total_complexity(&self, input: &str, num_robot_stages: usize) -> usize {
        input
            .trim()
            .lines()
            .map(|l| self.calculate_complexity(l, num_robot_stages))
            .sum()
    }
}

fn main() {
    let input = read_input("day21.txt");
    let keypad = Keypad::new_numeric();
    println!(
        "Part 1 = {}",
        keypad.calculate_total_complexity(input.as_str(), 2)
    );
    println!(
        "Part 2 = {}",
        keypad.calculate_total_complexity(input.as_str(), 25)
    );
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn test_numeric_keypad() {
        let keypad = Keypad::new_numeric();
        assert_eq!(
            keypad.min_paths.get(&('A', '0')),
            Some(vec![vec![Direction::Left]].as_ref())
        );
        let paths_0_9 = keypad.min_paths.get(&('0', '9'));
        assert!(paths_0_9.is_some());
        let paths_0_9 = paths_0_9.unwrap();
        assert_eq!(paths_0_9.len(), 4);
        assert!(paths_0_9.contains(
            vec![
                Direction::Up,
                Direction::Up,
                Direction::Up,
                Direction::Right,
            ]
            .as_ref()
        ));
        assert!(paths_0_9.contains(
            vec![
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Up,
            ]
            .as_ref()
        ));
        assert!(paths_0_9.contains(
            vec![
                Direction::Up,
                Direction::Right,
                Direction::Up,
                Direction::Up,
            ]
            .as_ref()
        ));
        assert!(paths_0_9.contains(
            vec![
                Direction::Right,
                Direction::Up,
                Direction::Up,
                Direction::Up,
            ]
            .as_ref()
        ));
    }

    #[test]
    fn test_directional_keypad() {
        let keypad = Keypad::new_directional();
        assert_eq!(
            keypad.min_paths.get(&('A', '^')),
            Some(vec![vec![Direction::Left]].as_ref())
        );
        let paths_l_a = keypad.min_paths.get(&('<', 'A'));
        assert!(paths_l_a.is_some());
        let paths_l_a = paths_l_a.unwrap();
        assert_eq!(paths_l_a.len(), 2);
        assert!(
            paths_l_a.contains(vec![Direction::Right, Direction::Right, Direction::Up,].as_ref())
        );
        assert!(
            paths_l_a.contains(vec![Direction::Right, Direction::Up, Direction::Right,].as_ref())
        );
    }

    #[test]
    fn test_find_seq_lengths() {
        let input = "029A";
        let keypad = Keypad::new_numeric();
        let min_length = keypad.find_shortest_sequence(input, 2, &mut HashMap::new());
        assert_eq!(min_length, 68);
    }

    #[test]
    fn test_calculate_complexity() {
        let input = "029A";
        let keypad = Keypad::new_numeric();
        assert_eq!(keypad.calculate_complexity(input, 2), 68 * 29);
    }

    #[test]
    fn part1() {
        let input = r#"029A
980A
179A
456A
379A"#;
        let keypad = Keypad::new_numeric();
        assert_eq!(keypad.calculate_total_complexity(input, 2), 126384);
    }
}
