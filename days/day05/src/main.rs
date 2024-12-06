use common::read_input;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq)]
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

    fn reorder(&self, ordering_rules: &HashMap<usize, Vec<usize>>) -> Self {
        let mut in_degree: HashMap<usize, usize> = HashMap::new();
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

        // Filter the ordering rules to only include numbers in updates
        let update_set: HashSet<usize> = self.0.iter().copied().collect();

        for (&key, dependencies) in ordering_rules {
            if update_set.contains(&key) {
                for &dep in dependencies {
                    if update_set.contains(&dep) {
                        graph.entry(key).or_default().push(dep);
                        *in_degree.entry(dep).or_insert(0) += 1;
                    }
                }
                in_degree.entry(key).or_insert(0); // Ensure key is in the in-degree map
            }
        }

        // Initialize the queue with nodes having in_degree == 0
        let mut queue: VecDeque<usize> = update_set
            .iter()
            .filter(|&&node| *in_degree.get(&node).unwrap_or(&0) == 0)
            .copied()
            .collect();

        let mut sorted: Vec<usize> = Vec::new();

        while let Some(current) = queue.pop_front() {
            sorted.push(current);

            if let Some(dependents) = graph.get(&current) {
                let mut sorted_dependents: Vec<usize> = dependents
                    .iter()
                    .filter(|&&d| update_set.contains(&d))
                    .copied()
                    .collect();

                sorted_dependents.sort_by_key(|&d| {
                    ordering_rules
                        .get(&current)
                        .and_then(|deps| deps.iter().position(|&x| x == d))
                        .unwrap_or(usize::MAX)
                });

                for dependent in sorted_dependents {
                    if let Some(in_degree_count) = in_degree.get_mut(&dependent) {
                        *in_degree_count -= 1;
                        if *in_degree_count == 0 {
                            queue.push_back(dependent);
                        }
                    }
                }
            }
        }

        // Add remaining nodes from update that weren't sorted
        for &node in &self.0 {
            if !sorted.contains(&node) {
                sorted.push(node);
            }
        }

        Self(sorted)
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

    fn sum_mid_incorrect_order(&self) -> usize {
        self.updates
            .iter()
            .filter(|update| !update.is_valid(&self.ordering_rules))
            .map(|update| update)
            .map(|update| update.reorder(&self.ordering_rules))
            .map(|update| update)
            .map(|update| update.get_mid())
            .sum()
    }
}

fn main() {
    let input = read_input("day05.txt");
    let pages = Pages::from(input.as_str());
    println!("Part 1 = {}", pages.sum_mid());
    println!("Part 2 = {}", pages.sum_mid_incorrect_order());
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

    #[parameterized(
        input = { vec![75,97,47,61,53], vec![61,13,29], vec![97,13,75,29,47] },
        expected = { vec![97,75,47,61,53], vec![61,29,13], vec![97,75,47,29,13] }
    )]
    fn test_reorder(input: Vec<usize>, expected: Vec<usize>) {
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
        assert_eq!(update.reorder(&rules), Update(expected));
    }

    #[test]
    fn part2() {
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
        assert_eq!(pages.sum_mid_incorrect_order(), 123);
    }
}
