use std::collections::HashMap;

use common::read_input;

#[derive(Debug)]
struct Update(Vec<usize>);

impl Update {
    fn new(v: Vec<usize>) -> Self {
        Self(v)
    }

    fn is_valid(&self, ordering_rules: &HashMap<usize, Vec<usize>>) -> bool {
        self.0.iter().enumerate().all(|(idx, update)| {
            if let Some(rules) = ordering_rules.get(&update) {
                !rules.iter().any(|after| self.0[0..idx].contains(after))
            } else {
                true
            }
        })
    }

    fn get_mid(&self) -> usize {
        *self.0.iter().nth(self.0.len() / 2).unwrap()
    }
}

#[derive(Debug)]
struct Pages {
    ordering_rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Update>,
}

#[derive(Clone)]
enum InputParserState {
    Start,
    ReadRule(usize, usize),
    ReadEmptyLine,
    ReadUpdates(Vec<usize>),
    Done,
}

fn transition(state: &InputParserState, line: &str) -> InputParserState {
    match (state, line) {
        (InputParserState::Start, s) | (InputParserState::ReadRule(_, _), s)
            if !s.trim().is_empty() =>
        {
            let (before, after) = s.split_once('|').unwrap();
            InputParserState::ReadRule(before.parse().unwrap(), after.parse().unwrap())
        }
        (InputParserState::ReadRule(_, _), s) if s.trim().is_empty() => {
            InputParserState::ReadEmptyLine
        }
        (InputParserState::ReadEmptyLine, s) | (InputParserState::ReadUpdates(_), s) => {
            let updates = s
                .split(',')
                .map(|update| update.parse().unwrap())
                .collect::<Vec<_>>();
            InputParserState::ReadUpdates(updates)
        }
        _ => InputParserState::Done,
    }
}

impl From<&str> for Pages {
    fn from(value: &str) -> Self {
        let (_, ordering_rules, updates) = value.lines().fold(
            (
                InputParserState::Start,
                HashMap::<usize, Vec<usize>>::new(),
                vec![],
            ),
            |(state, mut rules, mut updates), l| {
                let new_state = transition(&state, l);
                match new_state {
                    InputParserState::ReadRule(before, after) => {
                        rules
                            .entry(before)
                            .and_modify(|v| v.push(after))
                            .or_insert(vec![after]);
                        (new_state, rules, updates)
                    }
                    InputParserState::ReadUpdates(ref v) => {
                        updates.push(v.clone());
                        (new_state.clone(), rules, updates)
                    }
                    _ => (new_state, rules, updates),
                }
            },
        );
        Pages {
            ordering_rules,
            updates: updates.iter().map(|v| Update::new(v.to_vec())).collect(),
        }
    }
}

impl Pages {
    fn sum_mid(&self) -> usize {
        self.updates
            .iter()
            .filter(|update| update.is_valid(&self.ordering_rules))
            .map(|update| update.get_mid())
            .sum()
    }
}

fn main() {
    let input = read_input("day05.txt");
    let pages = Pages::from(input.as_str());
    println!("Part 1 = {}", pages.sum_mid());
}

#[cfg(test)]
mod day05_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { vec![75,47,61,53,29], vec![97,61,53,29,13], vec![75,29,13], vec![75,97,47,61,53], vec![61,13,29], vec![97,13,75,29,47] },
        expected = { true, true, true, false, false, false }
    )]
    fn test_is_valid(input: Vec<usize>, expected: bool) {
        let rules = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"#;
        let rules = rules
            .lines()
            .fold(HashMap::<usize, Vec<usize>>::new(), |mut rules, l| {
                let (before, after) = l.split_once('|').unwrap();
                let (before, after) = (
                    before.parse::<usize>().unwrap(),
                    after.parse::<usize>().unwrap(),
                );
                rules
                    .entry(before)
                    .and_modify(|v| v.push(after))
                    .or_insert(vec![after]);
                rules
            });
        let update = Update::new(input);
        assert_eq!(update.is_valid(&rules), expected);
    }

    #[test]
    fn part1() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        let pages = Pages::from(input);
        assert_eq!(pages.sum_mid(), 143);
    }
}
