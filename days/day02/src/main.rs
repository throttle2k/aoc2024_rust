use common::read_input;

fn is_report_safe(report: &[usize]) -> bool {
    report.windows(2).all(|level| {
        level[0] < level[1] && (level[1] - level[0]) >= 1 && (level[1] - level[0]) <= 3
    }) || report.windows(2).all(|level| {
        level[0] > level[1] && (level[0] - level[1]) >= 1 && (level[0] - level[1]) <= 3
    })
}

fn is_safe_removing(report: &[usize]) -> bool {
    (0..report.len()).any(|i| {
        let mut reduced = report.to_vec();
        reduced.remove(i);
        is_report_safe(&reduced)
    })
}

fn count_safe(input: &str, can_remove: bool) -> usize {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter_map(|report| {
            let is_safe = if can_remove {
                is_safe_removing(&report)
            } else {
                is_report_safe(&report)
            };
            if is_safe {
                Some(report)
            } else {
                None
            }
        })
        .count()
}

fn main() {
    let input = read_input("day02.txt");
    println!("Part 1 = {}", count_safe(&input, false));
    println!("Part 2 = {}", count_safe(&input, true));
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
        assert_eq!(count_safe(input, false), 2);
    }

    #[test]
    fn part2() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(count_safe(input, true), 4);
    }
}
