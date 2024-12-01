use common::read_input;

fn find_diff(list1: &[i32], list2: &[i32]) -> i32 {
    let mut list1 = list1.to_vec();
    list1.sort();
    let mut list2 = list2.to_vec();
    list2.sort();
    list1
        .iter()
        .zip(list2)
        .map(|(i1, i2)| (i2 - i1).abs())
        .sum()
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut split = l.trim().split_whitespace();
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.last().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((vec![], vec![]), |(mut list1, mut list2), (i1, i2)| {
            list1.push(i1);
            list2.push(i2);
            (list1, list2)
        })
}

fn main() {
    let input = read_input("day01.txt");
    let (list1, list2) = parse_input(&input);
    println!("Part1 = {}", find_diff(&list1, &list2));
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let (list1, list2) = parse_input(input);
        assert_eq!(find_diff(&list1, &list2), 11);
    }
}
