use common::read_input;

#[derive(Debug, Clone)]
struct Robot {
    position: (usize, usize),
    x_vel: isize,
    y_vel: isize,
}

fn parse_position(input: &str) -> (usize, usize) {
    let mut splits = input.strip_prefix("p=").unwrap().split(',');
    let x_pos = splits.next().unwrap().parse().unwrap();
    let y_pos = splits.next().unwrap().parse().unwrap();
    (x_pos, y_pos)
}

fn parse_velocity(input: &str) -> (isize, isize) {
    let mut splits = input.strip_prefix("v=").unwrap().split(',');
    let x_vel = splits.next().unwrap().parse().unwrap();
    let y_vel = splits.next().unwrap().parse().unwrap();
    (x_vel, y_vel)
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let mut splits = value.trim().split_whitespace();
        let position = parse_position(splits.next().unwrap());
        let (x_vel, y_vel) = parse_velocity(splits.next().unwrap());
        Self {
            position,
            x_vel,
            y_vel,
        }
    }
}

impl Robot {
    fn step(&self, times: usize, cols: usize, rows: usize) -> Self {
        let (x_steps, y_steps) = (0..times).fold((0, 0), |(x_steps, y_steps), _| {
            (x_steps + self.x_vel, y_steps + self.y_vel)
        });
        let position_x = self.position.0 as isize + x_steps;
        let position_x = if position_x > 0 {
            position_x as usize % cols
        } else {
            let delta_x = position_x.abs() as usize % cols;
            (cols - delta_x) % cols
        };
        let position_y = self.position.1 as isize + y_steps;
        let position_y = if position_y > 0 {
            position_y as usize % rows
        } else {
            let delta_y = position_y.abs() as usize % rows;
            (rows - delta_y) % rows
        };
        Self {
            position: (position_x, position_y),
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone)]
struct Restroom {
    robots: Vec<Robot>,
    rows: usize,
    cols: usize,
}

impl Restroom {
    fn new(input: &str, rows: usize, cols: usize) -> Self {
        let robots = input.trim().lines().map(|l| l.into()).collect();
        Self { robots, rows, cols }
    }

    fn steps(&self, times: usize) -> Self {
        let robots = self
            .robots
            .iter()
            .map(|r| r.step(times, self.cols, self.rows))
            .collect();
        Self {
            robots,
            ..self.clone()
        }
    }

    fn robots_at(&self, x: usize, y: usize) -> usize {
        self.robots.iter().filter(|r| r.position == (x, y)).count()
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        (0..self.rows)
            .map(|y| {
                (0..self.cols)
                    .map(|x| match self.robots_at(x, y) {
                        0 => ".".to_string(),
                        n => format!("{n}"),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn safety_factor(&self) -> usize {
        let mid_x = self.cols / 2;
        let mid_y = self.rows / 2;
        let quad_tl: usize = (0..mid_y)
            .map(|y| (0..mid_x).map(|x| self.robots_at(x, y)).sum::<usize>())
            .sum();
        let quad_tr: usize = (0..mid_y)
            .map(|y| {
                (mid_x + 1..self.cols)
                    .map(|x| self.robots_at(x, y))
                    .sum::<usize>()
            })
            .sum();
        let quad_bl: usize = (mid_y + 1..self.rows)
            .map(|y| (0..mid_x).map(|x| self.robots_at(x, y)).sum::<usize>())
            .sum();
        let quad_br: usize = (mid_y + 1..self.rows)
            .map(|y| {
                (mid_x + 1..self.cols)
                    .map(|x| self.robots_at(x, y))
                    .sum::<usize>()
            })
            .sum();
        quad_tl * quad_tr * quad_bl * quad_br
    }
}

fn main() {
    let input = read_input("day14.txt");
    let mut restroom = Restroom::new(input.as_str(), 103, 101);
    restroom = restroom.steps(100);
    println!("Part 1 = {}", restroom.safety_factor());
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn test_steps() {
        let input = "p=2,4 v=2,-3";
        let mut restroom = Restroom::new(input, 7, 11);
        assert_eq!(
            restroom.to_string(),
            r#"...........
...........
...........
...........
..1........
...........
..........."#
        );
        restroom = restroom.steps(1);
        assert_eq!(
            restroom.to_string(),
            r#"...........
....1......
...........
...........
...........
...........
..........."#
        );
        restroom = restroom.steps(1);
        assert_eq!(
            restroom.to_string(),
            r#"...........
...........
...........
...........
...........
......1....
..........."#
        );
        restroom = restroom.steps(1);
        assert_eq!(
            restroom.to_string(),
            r#"...........
...........
........1..
...........
...........
...........
..........."#
        );
        restroom = restroom.steps(1);
        assert_eq!(
            restroom.to_string(),
            r#"...........
...........
...........
...........
...........
...........
..........1"#
        );
        restroom = restroom.steps(1);
        assert_eq!(
            restroom.to_string(),
            r#"...........
...........
...........
.1.........
...........
...........
..........."#
        );
    }

    #[test]
    fn test_to_string() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        let restroom = Restroom::new(input, 7, 11);
        assert_eq!(
            restroom.to_string(),
            r#"1.12.......
...........
...........
......11.11
1.1........
.........1.
.......1..."#
        );
    }

    #[test]
    fn test_multiple_steps() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        let mut restroom = Restroom::new(input, 7, 11);
        restroom = restroom.steps(100);
        assert_eq!(
            restroom.to_string(),
            r#"......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1...."#
        );
    }

    #[test]
    fn part1() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;
        let mut restroom = Restroom::new(input, 7, 11);
        restroom = restroom.steps(100);
        assert_eq!(restroom.safety_factor(), 12);
    }
}
