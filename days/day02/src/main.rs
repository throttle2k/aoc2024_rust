use common::read_input;

fn is_report_safe(report: &[usize]) -> bool {
    report.windows(2).all(|level| {
        level[0] < level[1] && (level[1] - level[0]) >= 1 && (level[1] - level[0]) <= 3
    }) || report.windows(2).all(|level| {
        level[0] > level[1] && (level[0] - level[1]) >= 1 && (level[0] - level[1]) <= 3
    })
}

fn count_safe(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(|report| {
            if is_report_safe(&report) {
                Some(report)
            } else {
                None
            }
        })
        .count()
}

fn main() {
    let input = read_input("day02.txt");
    println!("Part 1 = {}", count_safe(&input));
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(count_safe(input), 2);
    }
}
