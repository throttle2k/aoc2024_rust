use std::str::FromStr;

use common::read_input;
use itertools::Itertools;

fn to_heights(input: &str) -> [usize; 5] {
    input.lines().fold([0; 5], |mut heights, l| {
        l.trim().char_indices().for_each(|idx_c| match idx_c {
            (idx, '#') => heights[idx] += 1,
            (_, '.') => (),
            (_, c) => panic!("Unknown char {c}"),
        });
        heights
    })
}

#[derive(Debug)]
struct Lock {
    pins: [usize; 5],
}

impl FromStr for Lock {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().lines().next().unwrap() != "#####" {
            Err("This is not a lock".to_string())
        } else {
            let s = s.trim().lines().skip(1).collect::<Vec<_>>().join("\n");
            let pins = to_heights(&s);
            Ok(Self { pins })
        }
    }
}

impl ToString for Lock {
    fn to_string(&self) -> String {
        self.pins.iter().map(|h| format!("{h}")).join(",")
    }
}

#[derive(Debug)]
struct Key {
    heights: [usize; 5],
}

impl FromStr for Key {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().lines().last().unwrap() != "#####" {
            Err("This is not a key".to_string())
        } else {
            let reversed = s
                .trim()
                .lines()
                .rev()
                .skip(1)
                .collect::<Vec<_>>()
                .join("\n");
            let heights = to_heights(&reversed);
            Ok(Self { heights })
        }
    }
}

impl ToString for Key {
    fn to_string(&self) -> String {
        self.heights.iter().map(|h| format!("{h}")).join(",")
    }
}

impl Key {
    fn fit(&self, lock: &Lock) -> bool {
        self.heights
            .iter()
            .enumerate()
            .all(|(pos, h)| h + lock.pins[pos] <= 5)
    }
}

fn parse_input(input: &str) -> (Vec<Key>, Vec<Lock>) {
    input
        .trim()
        .split("\n\n")
        .fold((vec![], vec![]), |(mut keys, mut locks), key_or_lock| {
            if let Ok(key) = key_or_lock.parse() {
                keys.push(key);
            } else if let Ok(lock) = key_or_lock.parse() {
                locks.push(lock);
            } else {
                panic!("Unknown input {key_or_lock}");
            }
            (keys, locks)
        })
}

fn main() {
    let input = read_input("day25.txt");
    let (keys, locks) = parse_input(input.as_str());
    let mut fits = 0;
    keys.iter().for_each(|k| {
        locks.iter().for_each(|l| {
            if k.fit(l) {
                fits += 1;
            }
        });
    });
    println!("Part 1 = {}", fits);
}

#[cfg(test)]
mod day25_tests {
    use super::*;

    #[test]
    fn test_parse_key_ok() {
        let input = r#".....
#....
#....
#...#
#.#.#
#.###
#####"#;
        let key: Result<Key, String> = input.parse();
        assert!(key.is_ok());
        let key = key.unwrap();
        assert_eq!(key.to_string(), "5,0,2,1,3");
    }

    #[test]
    fn test_parse_key_ko() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
....."#;
        let key: Result<Key, String> = input.parse();
        assert!(key.is_err());
    }

    #[test]
    fn test_parse_lock_ok() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
....."#;
        let lock: Result<Lock, String> = input.parse();
        assert!(lock.is_ok());
        let lock = lock.unwrap();
        assert_eq!(lock.to_string(), "0,5,3,4,3");
    }

    #[test]
    fn test_parse_lock_ko() {
        let input = r#".....
#....
#....
#...#
#.#.#
#.###
#####"#;
        let lock: Result<Lock, String> = input.parse();
        assert!(lock.is_err());
    }

    #[test]
    fn test_parse_input() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;
        let (keys, locks) = parse_input(input);
        assert_eq!(keys.len(), 3);
        assert_eq!(
            keys.iter().map(|k| k.to_string()).collect::<Vec<_>>(),
            vec!["5,0,2,1,3", "4,3,4,0,2", "3,0,2,0,1"]
        );
        assert_eq!(locks.len(), 2);
        assert_eq!(
            locks.iter().map(|l| l.to_string()).collect::<Vec<_>>(),
            vec!["0,5,3,4,3", "1,2,0,5,3"]
        );
    }

    #[test]
    fn test_key_fit_ok() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

.....
#....
#....
#....
#.#.#
#.###
#####"#;
        let (keys, locks) = parse_input(input);
        assert!(keys[0].fit(&locks[0]));
    }

    #[test]
    fn test_key_fit_ko() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

.....
#....
#....
#...#
#.#.#
#.###
#####"#;
        let (keys, locks) = parse_input(input);
        assert!(!keys[0].fit(&locks[0]));
    }

    #[test]
    fn part1() {
        let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"#;
        let (keys, locks) = parse_input(input);
        let mut fits = 0;
        keys.iter().for_each(|k| {
            locks.iter().for_each(|l| {
                if k.fit(l) {
                    fits += 1;
                }
            });
        });
        assert_eq!(fits, 3);
    }
}
