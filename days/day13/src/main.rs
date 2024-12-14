use std::ops::{Deref, DerefMut};

use common::read_input;

#[derive(Debug, Clone)]
struct Matrix2 {
    m: [[i64; 2]; 2],
}

impl From<Vec<Vec<i64>>> for Matrix2 {
    fn from(value: Vec<Vec<i64>>) -> Self {
        if value.len() != 2 {
            panic!("Cannot process matrices with other than 2 rows");
        };
        if value[0].len() != 2 {
            panic!("Cannot process matrices with other than 2 columns");
        };
        let mut m = [[0; 2]; 2];
        value
            .iter()
            .enumerate()
            .for_each(|(row, r)| r.iter().enumerate().for_each(|(col, c)| m[row][col] = *c));
        Self { m }
    }
}

impl Deref for Matrix2 {
    type Target = [[i64; 2]; 2];

    fn deref(&self) -> &Self::Target {
        &self.m
    }
}

impl DerefMut for Matrix2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m
    }
}

impl Matrix2 {
    fn determinant(&self) -> i64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }

    fn replace_col(&self, col: usize, values: [i64; 2]) -> Self {
        let mut new_m = self.clone();
        values
            .iter()
            .enumerate()
            .for_each(|(idx, v)| new_m[idx][col] = *v);
        new_m
    }
}

struct Equation {
    m: Matrix2,
    c: [i64; 2],
}

impl Equation {
    fn from(coeff: Vec<Vec<i64>>, constants: Vec<i64>) -> Self {
        let c = [constants[0], constants[1]];
        Self { m: coeff.into(), c }
    }

    fn int_solve(&self) -> Option<(i64, i64)> {
        let det = self.m.determinant();
        if det == 0 {
            return None;
        }

        let det_0 = self.m.replace_col(0, self.c).determinant();
        let det_1 = self.m.replace_col(1, self.c).determinant();
        if det_0 % det != 0 {
            return None;
        }
        if det_1 % det != 0 {
            return None;
        }
        let x = det_0 / det;
        if x < 0 {
            return None;
        }
        let y = det_1 / det;
        if y < 0 {
            return None;
        }
        Some((x, y))
    }
}

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse_button(input: &[&str]) -> (i64, i64) {
    let x_move = input[2]
        .split('+')
        .last()
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse()
        .unwrap();
    let y_move = input[3].split('+').last().unwrap().parse().unwrap();
    (x_move, y_move)
}

fn parse_prize(input: &[&str]) -> (i64, i64) {
    let x_pos = input[1]
        .split('=')
        .last()
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse()
        .unwrap();
    let y_pos = input[2].split('=').last().unwrap().parse().unwrap();
    (x_pos, y_pos)
}

impl From<&str> for ClawMachine {
    fn from(value: &str) -> Self {
        let mut lines = value.trim().lines();
        let button_a = parse_button(&lines.next().unwrap().split_whitespace().collect::<Vec<_>>());
        let button_b = parse_button(&lines.next().unwrap().split_whitespace().collect::<Vec<_>>());
        let prize = parse_prize(&lines.next().unwrap().split_whitespace().collect::<Vec<_>>());
        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

impl ClawMachine {
    fn with_delta(&self, delta: i64) -> Self {
        Self {
            prize: (self.prize.0 + delta, self.prize.1 + delta),
            ..self.clone()
        }
    }

    fn price_for_prize(&self) -> Option<i64> {
        let coeff = vec![
            vec![self.button_a.0, self.button_b.0],
            vec![self.button_a.1, self.button_b.1],
        ];
        let constants = vec![self.prize.0, self.prize.1];
        let eq = Equation::from(coeff, constants);
        if let Some((a_times, b_times)) = eq.int_solve() {
            Some(a_times * 3 + b_times * 1)
        } else {
            None
        }
    }
}

struct Arcade {
    claw_machines: Vec<ClawMachine>,
}

impl From<&str> for Arcade {
    fn from(value: &str) -> Self {
        let claw_machines = value
            .trim()
            .split("\n\n")
            .map(|machine| machine.into())
            .collect();
        Self { claw_machines }
    }
}

impl Arcade {
    fn with_delta(self, delta: i64) -> Self {
        let claw_machines = self
            .claw_machines
            .iter()
            .map(|machine| machine.with_delta(delta))
            .collect();
        Self { claw_machines }
    }

    fn find_min_price(&self) -> i64 {
        self.claw_machines
            .iter()
            .filter_map(|machine| machine.price_for_prize())
            .sum()
    }
}

fn main() {
    let input = read_input("day13.txt");
    let arcade = Arcade::from(input.as_str());
    println!("Part 1 = {}", arcade.find_min_price());
    let arcade = Arcade::from(input.as_str()).with_delta(10000000000000);
    println!("Part 2 = {}", arcade.find_min_price());
}

#[cfg(test)]
mod day13_tests {
    use parameterized::parameterized;

    use super::*;

    #[test]
    fn test_matrix() {
        let m = Matrix2::from(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m[0][0], 1);
        assert_eq!(m[0][1], 2);
        assert_eq!(m[1][0], 3);
        assert_eq!(m[1][1], 4);
    }

    #[test]
    fn test_determinant() {
        let m = Matrix2::from(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m.determinant(), -2);
    }

    #[test]
    fn test_replace_col() {
        let m = Matrix2::from(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m[0][0], 1);
        assert_eq!(m[0][1], 2);
        assert_eq!(m[1][0], 3);
        assert_eq!(m[1][1], 4);
        let m2 = m.replace_col(0, [5, 6]);
        assert_eq!(m2[0][0], 5);
        assert_eq!(m2[0][1], 2);
        assert_eq!(m2[1][0], 6);
        assert_eq!(m2[1][1], 4);
        let m2 = m.replace_col(1, [5, 6]);
        assert_eq!(m2[0][0], 1);
        assert_eq!(m2[0][1], 5);
        assert_eq!(m2[1][0], 3);
        assert_eq!(m2[1][1], 6);
    }

    #[parameterized(
        m = { vec![vec![-5, -3],vec![-7,5]], vec![vec![1, -5],vec![3, 5]] },
        c = { vec![-29, -13], vec![5, -17] },
        expected = { Some((4, 3)), None }
    )]
    fn test_solve_equation(m: Vec<Vec<i64>>, c: Vec<i64>, expected: Option<(i64, i64)>) {
        let eq = Equation::from(m, c);
        assert_eq!(eq.int_solve(), expected);
    }

    #[test]
    fn part1() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let arcade = Arcade::from(input);
        assert_eq!(arcade.find_min_price(), 480);
    }

    #[test]
    fn part2() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;
        let arcade = Arcade::from(input).with_delta(10000000000000);
        assert_eq!(arcade.find_min_price(), 875318608908);
    }
}
