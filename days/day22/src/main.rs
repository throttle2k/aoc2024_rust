use std::collections::{HashMap, HashSet};

use common::read_input;
use itertools::Itertools;

fn mix(num: usize, val: usize) -> usize {
    val ^ num
}

fn prune(num: usize) -> usize {
    num % 16777216
}

fn step1(secret_num: usize) -> usize {
    let multiplied = secret_num * 64;
    let secret_num = mix(secret_num, multiplied);
    prune(secret_num)
}

fn step2(secret_num: usize) -> usize {
    let divided = secret_num / 32;
    let secret_num = mix(secret_num, divided);
    prune(secret_num)
}

fn step3(secret_num: usize) -> usize {
    let multiplied = secret_num * 2048;
    let secret_num = mix(secret_num, multiplied);
    prune(secret_num)
}

fn process(secret_num: usize) -> usize {
    vec![secret_num]
        .iter()
        .map(|&secret_num| step1(secret_num))
        .map(|secret_num| step2(secret_num))
        .map(|secret_num| step3(secret_num))
        .next()
        .unwrap()
}

fn peek_next_numbers(secret_num: usize, n: usize) -> Vec<usize> {
    let (result, _) = (0..n).fold((vec![], secret_num), |(mut result, secret_num), _| {
        let secret_num = process(secret_num);
        result.push(secret_num);
        (result, secret_num)
    });
    result
}

fn to_price(secret_num: usize) -> usize {
    secret_num % 10
}

fn to_deltas(prices: Vec<usize>) -> Vec<(usize, isize)> {
    prices
        .iter()
        .tuple_windows()
        .map(|(&prev, &next)| (next, next as isize - prev as isize))
        .collect()
}

fn to_bananas_map(deltas: Vec<(usize, isize)>) -> HashMap<Vec<isize>, usize> {
    deltas
        .windows(4)
        .map(|ds| {
            let deltas = vec![ds[0].1, ds[1].1, ds[2].1, ds[3].1];
            let price = ds[3].0;
            (price, deltas)
        })
        .fold(HashMap::new(), |mut bananas_map, (price, deltas)| {
            bananas_map.entry(deltas.to_vec()).or_insert(price);
            bananas_map
        })
}

fn find_best_sequences_for_bananas(maps: Vec<HashMap<Vec<isize>, usize>>) -> usize {
    let sequences = maps
        .iter()
        .fold(HashSet::new(), |mut sequences, bananas_map| {
            bananas_map.keys().cloned().for_each(|key| {
                sequences.insert(key);
            });
            sequences
        });
    sequences
        .iter()
        .map(|seq| {
            maps.iter()
                .map(|bananas_map| bananas_map.get(seq).unwrap_or(&0))
                .sum()
        })
        .max()
        .unwrap_or_default()
}

fn process_multiple(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|secret_num| *peek_next_numbers(secret_num, 2000).last().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    let secret_nums_vec = input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|secret_num| {
            let mut v = vec![secret_num];
            v.extend(peek_next_numbers(secret_num, 1999));
            v
        })
        .collect::<Vec<_>>();
    let prices = secret_nums_vec
        .iter()
        .map(|secret_nums| {
            secret_nums
                .iter()
                .map(|&secret_num| to_price(secret_num))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let deltas_vec = prices
        .iter()
        .map(|secret_nums| to_deltas(secret_nums.to_vec()))
        .collect::<Vec<_>>();
    let bananas_maps = deltas_vec
        .iter()
        .map(|deltas| to_bananas_map(deltas.to_vec()))
        .collect::<Vec<_>>();
    find_best_sequences_for_bananas(bananas_maps)
}

fn main() {
    let input = read_input("day22.txt");
    println!("Part 1 = {}", process_multiple(input.as_str()));
    println!("Part 2 = {}", part2(input.as_str()));
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_process() {
        assert_eq!(process(123), 15887950);
    }

    #[test]
    fn test_peek_next_numbers() {
        assert_eq!(
            peek_next_numbers(123, 10),
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );
    }

    #[test]
    fn part1() {
        let input = r#"1
10
100
2024"#;
        assert_eq!(process_multiple(input), 37327623);
    }

    #[test]
    fn test_to_price() {
        let mut secret_nums = vec![123];
        secret_nums.extend(peek_next_numbers(123, 9));
        let prices = secret_nums
            .iter()
            .map(|&secret_num| to_price(secret_num))
            .collect::<Vec<_>>();
        assert_eq!(prices, vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn test_to_deltas() {
        let mut secret_nums = vec![123];
        secret_nums.extend(peek_next_numbers(123, 9));
        let prices = secret_nums
            .iter()
            .map(|&secret_num| to_price(secret_num))
            .collect::<Vec<_>>();
        let deltas = to_deltas(prices);
        assert_eq!(
            deltas,
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2)
            ]
        );
    }

    #[test]
    fn test_to_bananas_maps() {
        let input = r#"1
2
3
2024"#;
        let secret_nums_vec = input
            .trim()
            .lines()
            .map(|l| l.parse().unwrap())
            .map(|secret_num| {
                let mut v = vec![secret_num];
                v.extend(peek_next_numbers(secret_num, 1999));
                v
            })
            .collect::<Vec<_>>();
        let prices = secret_nums_vec
            .iter()
            .map(|secret_nums| {
                secret_nums
                    .iter()
                    .map(|&secret_num| to_price(secret_num))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let deltas_vec = prices
            .iter()
            .map(|secret_nums| to_deltas(secret_nums.to_vec()))
            .collect::<Vec<_>>();
        let bananas_maps = deltas_vec
            .iter()
            .map(|deltas| to_bananas_map(deltas.to_vec()))
            .collect::<Vec<_>>();
        assert_eq!(bananas_maps.len(), 4);
        let seq: Vec<isize> = vec![-2, 1, -1, 3];
        assert_eq!(
            bananas_maps.iter().nth(0).unwrap().get(&seq),
            Some(7).as_ref()
        );
        assert_eq!(
            bananas_maps.iter().nth(1).unwrap().get(&seq),
            Some(7).as_ref()
        );
        assert_eq!(bananas_maps.iter().nth(2).unwrap().get(&seq), None.as_ref());
        assert_eq!(
            bananas_maps.iter().nth(3).unwrap().get(&seq),
            Some(9).as_ref()
        );
    }

    #[test]
    fn test_part2() {
        let input = r#"1
2
3
2024"#;
        assert_eq!(part2(input), 23);
    }
}
