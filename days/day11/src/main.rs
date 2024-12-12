use std::fmt::Display;

use common::read_input;

#[derive(Debug)]
struct Stones {
    pebbles: Vec<u64>,
}

impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        let pebbles = value
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Self { pebbles }
    }
}

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pebbles
                .iter()
                .map(|n| format!("{n}"))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl Stones {
    fn step(self) -> Self {
        let pebbles = self
            .pebbles
            .iter()
            .flat_map(|p| match p {
                0 => vec![1],
                n if format!("{n}").len() % 2 == 0 => {
                    let as_string = format!("{n}");
                    let (first_half, second_half) = as_string.split_at(as_string.len() / 2);
                    let first_half = first_half.parse().unwrap();
                    let second_half = second_half.parse().unwrap();
                    vec![first_half, second_half]
                }
                n => vec![n * 2024],
            })
            .collect();
        Self { pebbles }
    }

    fn step_for(self, n: usize) -> Self {
        if n == 0 {
            self
        } else {
            self.step().step_for(n - 1)
        }
    }
}

fn main() {
    let input = read_input("day11.txt");
    let stones = Stones::from(input.as_str());
    println!("Part 1 = {}", stones.step_for(25).pebbles.len());
}

#[cfg(test)]
mod day11_tests {
    use parameterized::parameterized;

    use super::*;

    #[test]
    fn test_step() {
        let input = "0 1 10 99 999";
        let stones = Stones::from(input);
        assert_eq!(stones.step().to_string(), "1 2024 1 0 9 9 2021976");
    }

    #[test]
    fn test_step_for() {
        let input = "125 17";
        let stones = Stones::from(input);
        assert_eq!(
            stones.step_for(6).to_string(),
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
        );
    }

    #[parameterized(
        steps = { 1, 2, 3, 4, 5, 6, 25 },
        expected = { 3, 4, 5, 9, 13, 22, 55312 }
    )]
    fn part1(steps: usize, expected: usize) {
        let input = "125 17";
        let stones = Stones::from(input);
        assert_eq!(stones.step_for(steps).pebbles.len(), expected);
    }
}
