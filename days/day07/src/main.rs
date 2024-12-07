use common::read_input;

#[derive(Debug, PartialEq)]
struct Operation {
    total: u64,
    operands: Vec<u64>,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let (total, operands) = value.split_once(":").unwrap();
        let total = total.parse().unwrap();
        let operands = operands
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Self { total, operands }
    }
}

impl Operation {
    fn apply<T>(&self, op: T) -> Self
    where
        T: Fn(u64, u64) -> u64,
    {
        let (operand1, operand2) = (self.operands[0], self.operands[1]);
        let new_operand = op(operand1, operand2);
        let mut operands = vec![new_operand];
        operands.extend_from_slice(&self.operands[2..]);
        Self {
            total: self.total.clone(),
            operands,
        }
    }
}

fn validate_operation<T>(operation: Operation, operators: &[T]) -> Result<u64, ()>
where
    T: Fn(u64, u64) -> u64,
{
    if operation.operands.len() == 1 {
        let result = if *operation.operands.get(0).unwrap() == operation.total {
            Ok(operation.total.clone())
        } else {
            Err(())
        };
        return result;
    }

    operators
        .iter()
        .map(|op| {
            let new_operation = operation.apply(op);
            validate_operation(new_operation, operators)
        })
        .skip_while(|operation| operation.is_err())
        .next()
        .unwrap_or(Err(()))
}

fn sum_of_valid<T>(input: &str, operators: &[T]) -> u64
where
    T: Fn(u64, u64) -> u64,
{
    input
        .trim()
        .lines()
        .map(|l| Operation::from(l.trim()))
        .filter_map(|op| match validate_operation(op, operators) {
            Ok(n) => Some(n),
            Err(()) => None,
        })
        .sum()
}

fn main() {
    let input = read_input("day07.txt");
    let operators = vec![|a, b| a + b, |a, b| a * b];
    println!("Part 1 = {}", sum_of_valid(input.as_str(), &operators));
}

#[cfg(test)]
mod day07_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "190: 10 19", "3267: 81 40 27", "83: 17 5" },
        expected = { Operation { total: 190, operands: vec![10, 19] }, Operation { total: 3267, operands: vec![81, 40, 27] }, Operation { total: 83, operands: vec![17, 5] } }
    )]
    fn test_parse_input(input: &str, expected: Operation) {
        assert_eq!(Operation::from(input), expected);
    }

    #[parameterized(
        input = { "190: 10 19", "3267: 81 40 27", "83: 17 5", "156: 15 6", "7290: 6 8 6 15", "161011: 16 10 13", "192: 17 8 14", "21037: 9 7 18 13", "292: 11 6 16 20"},
        expected = { Ok(190), Ok(3267), Err(()), Err(()), Err(()), Err(()), Err(()), Err(()), Ok(292) }
    )]
    fn test_validate_operation(input: &str, expected: Result<u64, ()>) {
        let operators = vec![|a, b| a + b, |a, b| a * b];
        assert_eq!(
            validate_operation(Operation::from(input), &operators),
            expected
        );
    }

    #[test]
    fn part1() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        let operators = vec![|a, b| a + b, |a, b| a * b];
        assert_eq!(sum_of_valid(input, &operators), 3749);
    }
}
