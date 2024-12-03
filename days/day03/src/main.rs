use common::read_input;

#[derive(Debug)]
enum ParserState {
    ReadO,
    ReadApostrophe,
    ReadT,
    ReadOpenParenOrN(bool),
    ReadClosedParenDoOrDont(bool),
    DoneDoOrDont(bool),
    ReadMOrD,
    ReadU,
    ReadL,
    ReadOpenParen,
    ReadFirstNum(String),
    ReadSecondNum(i32, String),
    DoneMul(i32),
}

fn transition(state: &ParserState, c: char) -> ParserState {
    match (state, c) {
        (ParserState::ReadMOrD, 'm') => ParserState::ReadU,
        (ParserState::ReadMOrD, 'd') => ParserState::ReadO,
        (ParserState::ReadU, 'u') => ParserState::ReadL,
        (ParserState::ReadL, 'l') => ParserState::ReadOpenParen,
        (ParserState::ReadOpenParen, '(') => ParserState::ReadFirstNum(String::new()),
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
            ParserState::DoneMul(first_num * s.parse::<i32>().unwrap())
        }
        (ParserState::ReadO, 'o') => ParserState::ReadOpenParenOrN(true),
        (ParserState::ReadOpenParenOrN(true), 'n') => ParserState::ReadApostrophe,
        (ParserState::ReadOpenParenOrN(b), '(') => ParserState::ReadClosedParenDoOrDont(*b),
        (ParserState::ReadApostrophe, '\'') => ParserState::ReadT,
        (ParserState::ReadT, 't') => ParserState::ReadOpenParenOrN(false),
        (ParserState::ReadClosedParenDoOrDont(b), ')') => ParserState::DoneDoOrDont(*b),
        (_, _) => ParserState::ReadMOrD,
    }
}

fn parse_input(input: &str, check_do: bool) -> i32 {
    let (_state, mul_sum, _enabled) = input.chars().fold(
        (ParserState::ReadMOrD, 0, true),
        |(state, mul_sum, enabled), c| {
            let new_state = transition(&state, c);
            let (state, n, enabled) = match new_state {
                ParserState::DoneMul(n) => (ParserState::ReadMOrD, n, enabled),
                ParserState::DoneDoOrDont(b) => (ParserState::ReadMOrD, 0, !check_do || b),
                _ => (new_state, 0, enabled),
            };
            let mul_sum = if enabled { mul_sum + n } else { mul_sum };
            (state, mul_sum, enabled)
        },
    );
    mul_sum
}

fn main() {
    let input = read_input("day03.txt");
    println!("Part 1 = {}", parse_input(input.as_str(), false));
    println!("Part 2 = {}", parse_input(input.as_str(), true));
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
        assert_eq!(parse_input(input, false), expected);
    }

    #[test]
    fn part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_input(input, false), 161);
    }

    #[test]
    fn part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse_input(input, true), 48);
    }
}
