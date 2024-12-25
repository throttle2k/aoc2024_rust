use common::read_input;

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

#[allow(dead_code)]
fn peek_next_numbers(secret_num: usize, n: usize) -> Vec<usize> {
    let (result, _) = (0..n).fold((vec![], secret_num), |(mut result, secret_num), _| {
        let secret_num = process(secret_num);
        result.push(secret_num);
        (result, secret_num)
    });
    result
}

fn process_next_numbers(secret_num: usize, n: usize) -> usize {
    (0..n).fold(secret_num, |secret_num, _| process(secret_num))
}

fn process_multiple(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .map(|secret_num| process_next_numbers(secret_num, 2000))
        .sum()
}

fn main() {
    let input = read_input("day22.txt");
    println!("Part 1 = {}", process_multiple(input.as_str()));
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
}
