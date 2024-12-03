use common::read_input;

#[derive(Debug)]
enum ParserState {
    ReadM,
    ReadU,
    ReadL,
    ReadOpenP,
    ReadFirstNum(String),
    ReadSecondNum(i32, String),
    Done(i32),
}

fn transition(state: &ParserState, c: char) -> ParserState {
    match (state, c) {
        (ParserState::ReadM, 'm') => ParserState::ReadU,
        (ParserState::ReadU, 'u') => ParserState::ReadL,
        (ParserState::ReadL, 'l') => ParserState::ReadOpenP,
        (ParserState::ReadOpenP, '(') => ParserState::ReadFirstNum(String::new()),
        (ParserState::ReadFirstNum(s), c) if c.to_digit(10).is_some() => {
            let mut s = s.clone();
            s.push(c);
            ParserState::ReadFirstNum(s)
        }
        (ParserState::ReadFirstNum(s), ',') => {
            ParserState::ReadSecondNum(s.parse::<i32>().unwrap(), String::new())
        }
        (ParserState::ReadSecondNum(first_num, s), c) if c.to_digit(10).is_some() => {
            let mut s = s.clone();
            s.push(c);
            ParserState::ReadSecondNum(*first_num, s)
        }
        (ParserState::ReadSecondNum(first_num, s), ')') => {
            ParserState::Done(first_num * s.parse::<i32>().unwrap())
        }
        (_, _) => ParserState::ReadM,
    }
}

fn parse_input(input: &str) -> i32 {
    let (_state, mul_sum) = input
        .chars()
        .fold((ParserState::ReadM, 0), |(state, mul_sum), c| {
            let new_state = transition(&state, c);
            match new_state {
                ParserState::Done(n) => (ParserState::ReadM, mul_sum + n),
                _ => (new_state, mul_sum),
            }
        });
    mul_sum
}

fn main() {
    let input = read_input("day03.txt");
    println!("Part 1 = {}", parse_input(input.as_str()));
}

#[cfg(test)]
mod day03_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "mul(44,46)", "mul(123,4)", "mul(4*", "mul(6,9!", "?(12,34)", "mul ( 2 , 4 )" },
        expected = { 2024, 492, 0, 0, 0, 0 }
    )]
    fn test_simple(input: &str, expected: i32) {
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_input(input), 161);
    }
}
