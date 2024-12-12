use std::collections::HashMap;

use common::read_input;

#[derive(Debug)]
struct Stones {
    pebbles: HashMap<u64, usize>,
}

impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        let pebbles = value
            .trim()
            .split_whitespace()
            .fold(HashMap::new(), |mut pebbles, n| {
                pebbles
                    .entry(n.parse().unwrap())
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
                pebbles
            });
        Self { pebbles }
    }
}

impl Stones {
    fn blink(self, n: usize) -> Stones {
        if n == 0 {
            return self;
        }
        let mut new_pebbles = HashMap::<u64, usize>::new();
        self.pebbles.iter().for_each(|(k, num)| match k {
            0 => {
                new_pebbles
                    .entry(1)
                    .and_modify(|v| *v += num)
                    .or_insert(*num);
            }
            n if format!("{n}").len() % 2 == 0 => {
                let as_string = format!("{n}");
                let (first_half, second_half) = as_string.split_at(as_string.len() / 2);
                let first_half = first_half.parse().unwrap();
                let second_half = second_half.parse().unwrap();
                new_pebbles
                    .entry(first_half)
                    .and_modify(|v| *v += num)
                    .or_insert(*num);
                new_pebbles
                    .entry(second_half)
                    .and_modify(|v| *v += num)
                    .or_insert(*num);
            }
            n => {
                new_pebbles
                    .entry(n * 2024)
                    .and_modify(|v| *v += num)
                    .or_insert(*num);
            }
        });
        Self {
            pebbles: new_pebbles,
        }
        .blink(n - 1)
    }

    fn blink_for(self, n: usize) -> Self {
        self.blink(n)
    }
}

fn main() {
    let input = read_input("day11.txt");
    let stones = Stones::from(input.as_str());
    println!(
        "Part 1 = {}",
        stones.blink_for(25).pebbles.values().sum::<usize>()
    );
    let stones = Stones::from(input.as_str());
    println!(
        "Part 2 = {}",
        stones.blink_for(75).pebbles.values().sum::<usize>()
    );
}

#[cfg(test)]
mod day11_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        steps = { 1, 2, 3, 4, 5, 6, 25 },
        expected = { 3, 4, 5, 9, 13, 22, 55312 }
    )]
    fn part1(steps: usize, expected: usize) {
        let input = "125 17";
        let stones = Stones::from(input);
        let stones = stones.blink_for(steps);
        assert_eq!(stones.pebbles.values().sum::<usize>(), expected);
    }
}
