use std::collections::HashMap;

use common::read_input;

#[derive(Debug, Clone, PartialEq)]
struct Towel<'a>(&'a str);

impl<'a> ToString for Towel<'a> {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> From<&'a str> for Towel<'a> {
    fn from(value: &'a str) -> Self {
        Towel(value.trim())
    }
}

type Towels<'a> = Vec<Towel<'a>>;

fn can_make<'a>(
    pattern: &'a str,
    towels: &Towels<'a>,
    current: Towels<'a>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(result) = memo.get(pattern) {
        return *result;
    }

    towels
        .iter()
        .filter(|t| t.to_string().len() <= pattern.len())
        .map(|t| {
            let mut clone = current.clone();
            if pattern.starts_with(&t.to_string()) {
                clone.push(t.clone());
                let result = can_make(
                    pattern.strip_prefix(&t.to_string()).unwrap(),
                    towels,
                    clone,
                    memo,
                );
                memo.entry(&pattern)
                    .and_modify(|count| *count += result)
                    .or_insert(result);
                result
            } else {
                0
            }
        })
        .sum()
}

fn count_feasible<'a>(patterns: Vec<&str>, towels: &Towels<'a>) -> usize {
    let mut memo = HashMap::new();
    patterns
        .iter()
        .map(|pattern| {
            println!("Processing {pattern}");
            pattern
        })
        .map(|pattern| can_make(pattern, towels, vec![], &mut memo))
        .filter(|&count| count > 0)
        .count()
}

fn count_all_options<'a>(patterns: Vec<&str>, towels: &Towels<'a>) -> usize {
    let mut memo = HashMap::new();
    patterns
        .iter()
        .map(|pattern| {
            println!("Processing {pattern}");
            pattern
        })
        .map(|pattern| can_make(pattern, towels, vec![], &mut memo))
        .sum()
}

fn parse_input(input: &str) -> (Towels, Vec<&str>) {
    let (towels, patterns) = input.trim().split_once("\n\n").unwrap();
    let towels = towels.trim().split(',').map(|t| t.into()).collect();
    let patterns = patterns.split_whitespace().collect();
    (towels, patterns)
}

fn main() {
    let input = read_input("day19.txt");
    let (towels, patterns) = parse_input(input.as_str());
    println!("Part 1 = {}", count_feasible(patterns.clone(), &towels));
    println!("Part 2 = {}", count_all_options(patterns, &towels));
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        let (towels, patterns) = parse_input(input);
        assert_eq!(count_feasible(patterns, &towels), 6);
    }

    #[test]
    fn part2() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
        let (towels, patterns) = parse_input(input);
        assert_eq!(count_all_options(patterns, &towels), 16);
    }
}
