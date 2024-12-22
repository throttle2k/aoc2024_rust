use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

use common::read_input;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Press,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => String::from("^"),
            Direction::Down => String::from("v"),
            Direction::Left => String::from("<"),
            Direction::Right => String::from(">"),
            Direction::Press => String::from("A"),
        }
    }
}

#[derive(Debug)]
struct Edge {
    direction: Direction,
    from: char,
    to: char,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    directions: Vec<Direction>,
    current: char,
    remaining_index: usize, // Instead of storing the whole Vec
    current_distance: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.current_distance.cmp(&other.current_distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(
        directions: Vec<Direction>,
        current: char,
        remaining_index: usize,
        output: &str,
        distances: &HashMap<(char, char), usize>,
    ) -> Self {
        let current_distance = if remaining_index < output.len() {
            *distances
                .get(&(current, output.chars().nth(remaining_index).unwrap()))
                .unwrap_or(&0)
                + directions.len()
        } else {
            directions.len()
        };

        Self {
            directions,
            current,
            remaining_index,
            current_distance,
        }
    }
}

#[derive(Debug)]
struct Keyboard {
    edges: Vec<Edge>,
    distances: HashMap<(char, char), usize>,
}

impl Keyboard {
    fn new(edges: Vec<Edge>) -> Self {
        let keys: HashSet<char> = edges
            .iter()
            .flat_map(
                |&Edge {
                     from,
                     to,
                     direction: _,
                 }| vec![from, to],
            )
            .collect();
        let mut distances = HashMap::new();

        for &from in &keys {
            for &to in &keys {
                if from == to {
                    distances.insert((from, to), 0);
                } else {
                    distances.insert((from, to), usize::MAX);
                }
            }
        }

        for &Edge {
            from,
            to,
            direction: _,
        } in &edges
        {
            distances.insert((from, to), 1);
        }

        for &k in &keys {
            for &i in &keys {
                for &j in &keys {
                    if let Some(&d_ik) = distances.get(&(i, k)) {
                        if let Some(&d_kj) = distances.get(&(k, j)) {
                            let d_ij = distances.get(&(i, j)).unwrap_or(&usize::MAX);
                            if d_ik != usize::MAX && d_kj != usize::MAX {
                                distances.insert((i, j), d_ik.saturating_add(d_kj).min(*d_ij));
                            }
                        }
                    }
                }
            }
        }

        Self { edges, distances }
    }

    fn new_numeric() -> Self {
        let edge78 = Edge::new('7', '8', Direction::Right);
        let edge74 = Edge::new('7', '4', Direction::Down);
        let edge87 = Edge::new('8', '7', Direction::Left);
        let edge85 = Edge::new('8', '5', Direction::Down);
        let edge89 = Edge::new('8', '9', Direction::Right);
        let edge98 = Edge::new('9', '8', Direction::Left);
        let edge96 = Edge::new('9', '6', Direction::Down);
        let edge47 = Edge::new('4', '7', Direction::Up);
        let edge45 = Edge::new('4', '5', Direction::Right);
        let edge41 = Edge::new('4', '1', Direction::Down);
        let edge54 = Edge::new('5', '4', Direction::Left);
        let edge58 = Edge::new('5', '8', Direction::Up);
        let edge56 = Edge::new('5', '6', Direction::Right);
        let edge52 = Edge::new('5', '2', Direction::Down);
        let edge65 = Edge::new('6', '5', Direction::Left);
        let edge69 = Edge::new('6', '9', Direction::Up);
        let edge63 = Edge::new('6', '3', Direction::Down);
        let edge14 = Edge::new('1', '4', Direction::Up);
        let edge12 = Edge::new('1', '2', Direction::Right);
        let edge21 = Edge::new('2', '1', Direction::Left);
        let edge25 = Edge::new('2', '5', Direction::Up);
        let edge23 = Edge::new('2', '3', Direction::Right);
        let edge20 = Edge::new('2', '0', Direction::Down);
        let edge32 = Edge::new('3', '2', Direction::Left);
        let edge36 = Edge::new('3', '6', Direction::Up);
        let edge3a = Edge::new('3', 'A', Direction::Down);
        let edge02 = Edge::new('0', '2', Direction::Up);
        let edge0a = Edge::new('0', 'A', Direction::Right);
        let edgea0 = Edge::new('A', '0', Direction::Left);
        let edgea3 = Edge::new('A', '3', Direction::Up);
        let edges = vec![
            edgea0, edgea3, edge0a, edge02, edge12, edge14, edge20, edge21, edge23, edge25, edge3a,
            edge32, edge36, edge41, edge45, edge47, edge52, edge54, edge56, edge58, edge63, edge65,
            edge69, edge74, edge78, edge85, edge87, edge89, edge96, edge98,
        ];
        Self::new(edges)
    }

    fn new_directional() -> Self {
        let edgeua = Edge::new('^', 'A', Direction::Right);
        let edgeud = Edge::new('^', 'v', Direction::Down);
        let edgeau = Edge::new('A', '^', Direction::Left);
        let edgear = Edge::new('A', '>', Direction::Down);
        let edgeld = Edge::new('<', 'v', Direction::Right);
        let edgedl = Edge::new('v', '<', Direction::Left);
        let edgedu = Edge::new('v', '^', Direction::Up);
        let edgedr = Edge::new('v', '>', Direction::Right);
        let edgerd = Edge::new('>', 'v', Direction::Left);
        let edgera = Edge::new('>', 'A', Direction::Up);
        let edges = vec![
            edgeua, edgeud, edgeau, edgear, edgeld, edgedl, edgedu, edgedr, edgerd, edgera,
        ];
        Self::new(edges)
    }

    fn neighbors(&self, key: char) -> Vec<&Edge> {
        self.edges.iter().filter(|edge| edge.from == key).collect()
    }

    fn find_sequence(&self, output: &str) -> Vec<Vec<Direction>> {
        let mut queue = BinaryHeap::new();
        let mut visited = HashMap::new();
        let output_chars: Vec<char> = output.chars().collect();
        let state = State::new(vec![], 'A', 0, output, &self.distances);

        let mut results = Vec::new();
        let mut min_length = usize::MAX;

        // Initialize with first state
        visited.insert((state.current, 0), HashSet::from([0])); // Track set of lengths for each state
        queue.push(Reverse(state));

        while let Some(Reverse(State {
            directions,
            current,
            remaining_index,
            current_distance: _,
        })) = queue.pop()
        {
            if !remaining_index < output_chars.len() && directions.len() >= min_length {
                continue;
            }

            if remaining_index >= output_chars.len() {
                if directions.len() <= min_length {
                    if directions.len() < min_length {
                        min_length = directions.len();
                        results.clear();
                    }
                    results.push(directions);
                }
                continue;
            }

            let target_char = output_chars[remaining_index];

            if current == target_char {
                let mut new_directions = directions.clone();
                new_directions.push(Direction::Press);
                let new_state = State::new(
                    new_directions,
                    current,
                    remaining_index + 1,
                    output,
                    &self.distances,
                );

                let lengths = visited
                    .entry((new_state.current, remaining_index + 1))
                    .or_insert_with(HashSet::new);

                // Add this path if it's not longer than any we've seen
                if new_state.directions.len() <= min_length
                    && !lengths.iter().any(|&l| l < new_state.directions.len())
                {
                    lengths.insert(new_state.directions.len());
                    queue.push(Reverse(new_state));
                }
            }

            for Edge { to, direction, .. } in self.neighbors(current) {
                let mut new_directions = directions.clone();
                new_directions.push(direction.clone());

                if new_directions.len() < min_length {
                    let next_state = State::new(
                        new_directions,
                        *to,
                        remaining_index,
                        output,
                        &self.distances,
                    );

                    let lengths = visited
                        .entry((next_state.current, remaining_index))
                        .or_insert_with(HashSet::new);

                    // Add this path if it's not longer than any we've seen
                    if !lengths.iter().any(|&l| l < next_state.directions.len()) {
                        lengths.insert(next_state.directions.len());
                        queue.push(Reverse(next_state));
                    }
                }
            }
        }

        results
    }
}

fn find_complexity(output: &str) -> usize {
    let numeric_code = output
        .chars()
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let keyboard_depressurized = Keyboard::new_numeric();
    let depressurized = keyboard_depressurized.find_sequence(output);
    let depressurized = depressurized
        .iter()
        .map(|seq| seq.iter().map(|d| d.to_string()).collect::<String>())
        .collect::<Vec<_>>();
    let depressurized_min_length = depressurized.iter().map(|s| s.len()).min().unwrap();
    let depressurized = depressurized
        .iter()
        .filter(|s| s.len() == depressurized_min_length)
        .collect::<Vec<_>>();
    let radiation = depressurized
        .iter()
        .flat_map(|output| {
            let keyboard_radiation = Keyboard::new_directional();
            let radiation = keyboard_radiation.find_sequence(output);
            radiation
                .iter()
                .map(|seq| seq.iter().map(|d| d.to_string()).collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let radiation_min_length = radiation.iter().map(|s| s.len()).min().unwrap();
    let radiation = radiation
        .iter()
        .filter(|s| s.len() == radiation_min_length)
        .collect::<Vec<_>>();
    let freezing = radiation
        .iter()
        .flat_map(|output| {
            let keyboard_freezing = Keyboard::new_directional();
            let freezing = keyboard_freezing.find_sequence(output);
            freezing
                .iter()
                .map(|seq| seq.iter().map(|d| d.to_string()).collect::<String>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let freezing_min_length = freezing.iter().map(|s| s.len()).min().unwrap();
    let freezing = freezing
        .iter()
        .filter(|s| s.len() == freezing_min_length)
        .collect::<Vec<_>>();
    freezing.get(0).unwrap().len() * numeric_code
}

fn main() {
    let input = read_input("day21.txt");
    let outputs = input.trim().lines().collect::<Vec<_>>();
    let total_complexity = outputs
        .iter()
        .map(|output| find_complexity(output))
        .sum::<usize>();
    println!("Part 1 = {}", total_complexity);
}

#[cfg(test)]
mod day21_tests {
    use parameterized::parameterized;

    use super::*;

    #[test]
    fn test_find_sequences_depressurized() {
        let output = "029A";
        let keyboard = Keyboard::new_numeric();
        let sequences = keyboard.find_sequence(output);
        let sequences = sequences
            .iter()
            .map(|s| s.iter().map(|d| d.to_string()).collect::<String>())
            .collect::<Vec<_>>();
        assert!(sequences.iter().any(|s| s == "<A^A>^^AvvvA"));
    }

    #[test]
    fn test_find_sequences_radiation() {
        let output = "<A^A>^^AvvvA";
        let keyboard = Keyboard::new_directional();
        let sequences = keyboard.find_sequence(output);
        let sequences = sequences
            .iter()
            .map(|s| s.iter().map(|d| d.to_string()).collect::<String>())
            .collect::<Vec<_>>();
        assert!(sequences
            .iter()
            .any(|s| s == "v<<A>>^A<A>AvA<^AA>A<vAAA>^A"));
    }

    #[parameterized]
    fn test_find_sequences_freezing() {
        let output = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let keyboard = Keyboard::new_directional();
        let sequences = keyboard.find_sequence(output);
        let sequences = sequences
            .iter()
            .map(|s| s.iter().map(|d| d.to_string()).collect::<String>())
            .collect::<Vec<_>>();
        assert!(sequences
            .iter()
            .any(|s| s == "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"));
    }

    #[parameterized(
        output = { "029A", "980A", "179A", "456A", "379A" },
        expected = { 68*29, 60*980, 68*179, 64*456, 64*379 }
    )]
    fn part1(output: &str, expected: usize) {
        assert_eq!(find_complexity(output), expected);
    }
}
