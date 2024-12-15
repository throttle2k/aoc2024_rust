use core::panic;
use std::cell::RefCell;

use common::read_input;

#[derive(Debug)]
enum Tile {
    Wall,
    Floor,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Wall => "#".to_string(),
            Tile::Floor => ".".to_string(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
            c => panic!("Unknown tile {c}"),
        }
    }
}

impl Tile {
    fn is_walkable(&self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Floor => true,
        }
    }
}

#[derive(Debug)]
struct WarehouseBox(usize, usize);

impl ToString for WarehouseBox {
    fn to_string(&self) -> String {
        "O".to_string()
    }
}

impl WarehouseBox {
    fn new(col: usize, row: usize) -> Self {
        Self(col, row)
    }

    fn gps(&self) -> usize {
        self.1 * 100 + self.0
    }
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<Tile>>,
    boxes: Vec<RefCell<WarehouseBox>>,
    robot: Robot,
}

impl ToString for Warehouse {
    fn to_string(&self) -> String {
        self.map
            .iter()
            .enumerate()
            .map(|(row, r)| {
                r.iter()
                    .enumerate()
                    .map(|(col, tile)| {
                        if col == self.robot.col && row == self.robot.row {
                            self.robot.to_string()
                        } else {
                            self.boxes
                                .iter()
                                .find_map(|b| {
                                    if b.borrow().0 == col && b.borrow().1 == row {
                                        Some(b.borrow().to_string())
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(tile.to_string())
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl From<&str> for Warehouse {
    fn from(value: &str) -> Self {
        let (warehouse, movements) = value.split_once("\n\n").unwrap();
        let movements = movements
            .trim()
            .chars()
            .filter_map(|c| {
                if c.is_whitespace() {
                    None
                } else {
                    Some(c.into())
                }
            })
            .collect();
        let mut robot = Robot::new(0, 0, movements);
        let mut boxes = vec![];
        let map = warehouse
            .trim()
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(col, c)| {
                        if c == 'O' {
                            boxes.push(RefCell::new(WarehouseBox::new(col, row)));
                            Tile::Floor
                        } else if c == '@' {
                            robot.col = col;
                            robot.row = row;
                            Tile::Floor
                        } else {
                            c.into()
                        }
                    })
                    .collect()
            })
            .collect();
        Self { map, boxes, robot }
    }
}

impl Warehouse {
    fn tile_at(&self, col: usize, row: usize) -> &Tile {
        &self.map[row][col]
    }

    fn move_box(
        &self,
        wh_box: &RefCell<WarehouseBox>,
        movement: Movement,
    ) -> Option<(usize, usize)> {
        let (prev_col, prev_row) = (wh_box.borrow().0, wh_box.borrow().1);
        let (next_col, next_row) = movement.next_position((wh_box.borrow().0, wh_box.borrow().1));
        if let Some(next_box) = self
            .boxes
            .iter()
            .find(|&b| b.borrow().0 == next_col && b.borrow().1 == next_row)
        {
            if let Some((new_col, new_row)) = self.move_box(next_box, movement) {
                wh_box.borrow_mut().0 = new_col;
                wh_box.borrow_mut().1 = new_row;
                Some((prev_col, prev_row))
            } else {
                None
            }
        } else {
            if self.tile_at(next_col, next_row).is_walkable() {
                wh_box.borrow_mut().0 = next_col;
                wh_box.borrow_mut().1 = next_row;
                Some((prev_col, prev_row))
            } else {
                None
            }
        }
    }

    fn step(mut self, movement: Movement) -> Self {
        let (next_col, next_row) = movement.next_position((self.robot.col, self.robot.row));
        if let Some(next_box) = self
            .boxes
            .iter()
            .find(|&b| b.borrow().0 == next_col && b.borrow().1 == next_row)
        {
            if let Some((new_col, new_row)) = self.move_box(next_box, movement) {
                self.robot.col = new_col;
                self.robot.row = new_row;
            }
        } else {
            if self.tile_at(next_col, next_row).is_walkable() {
                self.robot.col = next_col;
                self.robot.row = next_row;
            }
        };
        self
    }

    fn walk(mut self) -> Self {
        while !self.robot.movements.is_empty() {
            let next_movement = self.robot.movements.remove(0);
            self = self.step(next_movement);
        }
        self
    }

    fn gps(&self) -> usize {
        self.boxes.iter().map(|b| b.borrow().gps()).sum()
    }
}

#[derive(Debug, Clone)]
enum Movement {
    North,
    East,
    South,
    West,
}

impl From<char> for Movement {
    fn from(value: char) -> Self {
        match value {
            '^' => Movement::North,
            '>' => Movement::East,
            'v' => Movement::South,
            '<' => Movement::West,
            c => panic!("Unknown movement {c}"),
        }
    }
}

impl Movement {
    fn delta(&self) -> (isize, isize) {
        match self {
            Movement::North => (0, -1),
            Movement::East => (1, 0),
            Movement::South => (0, 1),
            Movement::West => (-1, 0),
        }
    }

    fn next_position(&self, (col, row): (usize, usize)) -> (usize, usize) {
        let delta = self.delta();
        (
            (col as isize + delta.0) as usize,
            (row as isize + delta.1) as usize,
        )
    }
}

#[derive(Debug)]
struct Robot {
    col: usize,
    row: usize,
    movements: Vec<Movement>,
}

impl ToString for Robot {
    fn to_string(&self) -> String {
        "@".to_string()
    }
}

impl Robot {
    fn new(col: usize, row: usize, movements: Vec<Movement>) -> Self {
        Self {
            row,
            col,
            movements,
        }
    }
}

fn main() {
    let input = read_input("day15.txt");
    let warehouse = Warehouse::from(input.as_str()).walk();
    println!("Part 1 = {}", warehouse.gps());
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn test_parse_and_to_string() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let (input_warehouse, _) = input.split_once("\n\n").unwrap();
        let warehouse = Warehouse::from(input);
        assert_eq!(warehouse.to_string(), input_warehouse);
    }

    #[test]
    fn test_step() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let warehouse = Warehouse::from(input);
        let warehouse = warehouse.step(Movement::West);
        assert_eq!(
            warehouse.to_string(),
            r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        );
        let warehouse = warehouse.step(Movement::North);
        assert_eq!(
            warehouse.to_string(),
            r#"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        );
        let warehouse = warehouse.step(Movement::North);
        assert_eq!(
            warehouse.to_string(),
            r#"########
#.@O.O.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        );
        let warehouse = warehouse.step(Movement::East);
        assert_eq!(
            warehouse.to_string(),
            r#"########
#..@OO.#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        );
        let warehouse = warehouse.step(Movement::East);
        assert_eq!(
            warehouse.to_string(),
            r#"########
#...@OO#
##..O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"#
        );
    }

    #[test]
    fn test_walk() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let warehouse = Warehouse::from(input).walk();
        assert_eq!(
            warehouse.to_string(),
            r#"########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########"#
        );
    }

    #[test]
    fn test_walk_2() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        let warehouse = Warehouse::from(input).walk();
        assert_eq!(
            warehouse.to_string(),
            r#"##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########"#
        );
    }

    #[test]
    fn test_gps() {
        let input = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        let warehouse = Warehouse::from(input).walk();
        assert_eq!(warehouse.gps(), 2028)
    }

    #[test]
    fn part1() {
        let input = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        let warehouse = Warehouse::from(input).walk();
        assert_eq!(warehouse.gps(), 10092);
    }
}
