use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use common::read_input;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
struct Computer(String);

impl ToString for Computer {
    fn to_string(&self) -> String {
        let Computer(name) = self;
        name.to_string()
    }
}

impl Computer {
    fn name_starts_with(&self, letter: &str) -> bool {
        let Computer(name) = self;
        name.starts_with(letter)
    }
}

#[derive(Debug, Clone)]
struct Connection {
    from: Computer,
    to: Vec<Computer>,
}

impl From<&str> for Connection {
    fn from(value: &str) -> Self {
        let (from, to) = value.split_once('-').unwrap();
        Self {
            from: Computer(from.to_string()),
            to: vec![Computer(to.to_string())],
        }
    }
}

#[derive(Debug)]
struct Lan {
    connections: Vec<Connection>,
}

impl From<&str> for Lan {
    fn from(value: &str) -> Self {
        let mut connections: Vec<Connection> = vec![];
        value.trim().lines().for_each(|l| {
            let direct = Connection::from(l);
            let (from, to) = l.split_once('-').unwrap();
            let reverse = format!("{to}-{from}");
            let reverse = Connection::from(reverse.as_str());
            if let Some(connection) = connections.iter_mut().find(|c| c.from == direct.from) {
                connection.to.extend(direct.to.clone());
            } else {
                connections.push(direct.clone());
            }
            if let Some(connection) = connections.iter_mut().find(|c| c.from == reverse.from) {
                connection.to.extend(reverse.to);
            } else {
                connections.push(reverse);
            }
        });
        Self { connections }
    }
}

impl Lan {
    fn find_three_connections(&self) -> Vec<[Computer; 3]> {
        let mut three_connections = self
            .connections
            .iter()
            .filter_map(|conn| {
                if conn.to.len() >= 2 {
                    Some(
                        conn.to
                            .iter()
                            .cartesian_product(conn.to.clone())
                            .filter_map(|(first, second)| {
                                if self
                                    .connections
                                    .iter()
                                    .find(|c| c.from == *first)
                                    .unwrap()
                                    .to
                                    .contains(&second)
                                {
                                    Some([conn.from.clone(), first.clone(), second.clone()])
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                } else {
                    None
                }
            })
            .flat_map(|v| v)
            .collect::<Vec<_>>();
        three_connections.iter_mut().for_each(|conns| conns.sort());
        three_connections.sort();
        three_connections.dedup();
        three_connections
    }

    fn find_triple_connections_with_letter(&self, letter: &str) -> Vec<[Computer; 3]> {
        self.find_three_connections()
            .into_iter()
            .filter(|comps| comps.iter().any(|c| c.name_starts_with(letter)))
            .collect::<Vec<_>>()
    }

    fn find_maximal_cliques(&self) -> Vec<HashSet<Computer>> {
        let mut graph: HashMap<Computer, HashSet<Computer>> = HashMap::new();
        self.connections.iter().cloned().for_each(|connection| {
            for to in connection.to {
                let from = connection.from.clone();
                graph
                    .entry(from.clone())
                    .and_modify(|set| {
                        set.insert(to.clone());
                    })
                    .or_insert(HashSet::from_iter(vec![to.clone()]));
                graph
                    .entry(to)
                    .and_modify(|set| {
                        set.insert(from.clone());
                    })
                    .or_insert(HashSet::from_iter(vec![from]));
            }
        });
        let mut cliques = Vec::new();
        let nodes = graph.keys().cloned().collect();
        bron_kerbosch(HashSet::new(), nodes, HashSet::new(), &graph, &mut cliques);
        cliques
    }

    fn find_password(&self) -> String {
        let cliques = self.find_maximal_cliques();
        let max_clique = cliques.iter().max_by_key(|set| set.len()).unwrap();
        let mut as_vec = max_clique.iter().collect::<Vec<_>>();
        as_vec.sort();
        as_vec.iter().map(|&c| c.to_string()).join(",")
    }
}

fn bron_kerbosch<T>(
    r: HashSet<T>,
    mut p: HashSet<T>,
    mut x: HashSet<T>,
    graph: &HashMap<T, HashSet<T>>,
    cliques: &mut Vec<HashSet<T>>,
) where
    T: Clone + Eq + Hash,
{
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
        return;
    }
    p.clone().iter().for_each(|node| {
        let mut new_r = r.clone();
        new_r.insert(node.clone());

        let new_p = p.intersection(&graph[&node]).cloned().collect();
        let new_x = x.intersection(&graph[&node]).cloned().collect();

        bron_kerbosch(new_r, new_p, new_x, graph, cliques);

        p.remove(&node);
        x.insert(node.clone());
    });
}

fn main() {
    let input = read_input("day23.txt");
    let lan = Lan::from(input.as_str());
    println!(
        "Part 1 = {}",
        lan.find_triple_connections_with_letter("t").len()
    );
    println!("Part 2 = {}", lan.find_password());
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn test_find_three_connections() {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
        let lan = Lan::from(input);
        assert_eq!(lan.find_three_connections().len(), 12);
    }

    #[test]
    fn part1() {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
        let lan = Lan::from(input);
        assert_eq!(lan.find_triple_connections_with_letter("t").len(), 7);
    }

    #[test]
    fn part2() {
        let input = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;
        let lan = Lan::from(input);
        assert_eq!(lan.find_password(), "co,de,ka,ta");
    }
}
