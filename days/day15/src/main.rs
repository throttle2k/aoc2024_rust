use core::panic;
use std::cell::RefCell;

use common::read_input;

#[derive(Debug, Clone)]
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
struct WarehouseBox {
    col: usize,
    row: usize,
    glyph: String,
    scaled: bool,
}

impl WarehouseBox {
    fn to_string(&self, col: usize) -> String {
        if self.col == col {
            self.glyph.chars().nth(0).unwrap().to_string()
        } else {
            self.glyph.chars().nth(1).unwrap().to_string()
        }
    }

    fn new(col: usize, row: usize, scaled: bool) -> Self {
        let glyph = match scaled {
            true => "[]".to_string(),
            false => "O".to_string(),
        };
        Self {
            col,
            row,
            scaled,
            glyph,
        }
    }

    fn gps(&self) -> usize {
        self.row * 100 + self.col
    }

    fn is_in(&self, col: usize, row: usize) -> bool {
        let same_col = if self.scaled {
            self.col == col || self.col + 1 == col
        } else {
            self.col == col
        };
        same_col && self.row == row
    }

    fn will_hit(&self, col: usize, row: usize, movement: &Movement) -> bool {
        let same_col = if self.scaled {
            match movement {
                Movement::North | Movement::South => {
                    self.col == col || self.col + 1 == col || self.col == col + 1
                }
                Movement::East | Movement::West => {
                    self.col == col || self.col + 1 == col || self.col == col + 1
                }
            }
        } else {
            self.col == col
        };
        same_col && self.row == row
    }
}

#[derive(Debug)]
struct Warehouse {
    map: Vec<Vec<Tile>>,
    boxes: Vec<RefCell<WarehouseBox>>,
    robot: Robot,
    scaled: bool,
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
                                    if b.borrow().is_in(col, row) {
                                        Some(b.borrow().to_string(col))
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

impl Warehouse {
    fn from(value: &str, scaled: bool) -> Self {
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
                    .flat_map(|(col, c)| {
                        let num = if scaled { 2 } else { 1 };
                        if c == 'O' {
                            boxes.push(RefCell::new(WarehouseBox::new(col * num, row, scaled)));
                            vec![Tile::Floor; num]
                        } else if c == '@' {
                            robot.col = col * num;
                            robot.row = row;
                            vec![Tile::Floor; num]
                        } else {
                            vec![c.into(); num]
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            map,
            boxes,
            robot,
            scaled,
        }
    }

    fn tile_at(&self, col: usize, row: usize) -> &Tile {
        &self.map[row][col]
    }

    fn can_move_box(&self, wh_box: &RefCell<WarehouseBox>, movement: &Movement) -> bool {
        let (prev_col, prev_row) = (wh_box.borrow().col, wh_box.borrow().row);
        let (next_col, next_row) =
            movement.next_position((wh_box.borrow().col, wh_box.borrow().row));
        let walkable = match self.scaled {
            true => {
                self.tile_at(next_col, next_row).is_walkable()
                    && self.tile_at(next_col + 1, next_row).is_walkable()
            }
            false => self.tile_at(next_col, next_row).is_walkable(),
        };
        walkable
            && self
                .boxes
                .iter()
                .filter(|&b| b.borrow().col != prev_col || b.borrow().row != prev_row)
                .filter(|&b| b.borrow().will_hit(next_col, next_row, movement))
                .all(|b| self.can_move_box(b, movement))
    }

    fn move_box(
        &self,
        wh_box: &RefCell<WarehouseBox>,
        movement: &Movement,
    ) -> Option<(usize, usize)> {
        let (prev_col, prev_row) = (wh_box.borrow().col, wh_box.borrow().row);
        let (next_col, next_row) =
            movement.next_position((wh_box.borrow().col, wh_box.borrow().row));
        let walkable = match self.scaled {
            true => {
                self.tile_at(next_col, next_row).is_walkable()
                    && self.tile_at(next_col + 1, next_row).is_walkable()
            }
            false => self.tile_at(next_col, next_row).is_walkable(),
        };
        if walkable
            && self
                .boxes
                .iter()
                .filter(|&b| b.borrow().col != prev_col || b.borrow().row != prev_row)
                .filter(|&b| b.borrow().will_hit(next_col, next_row, movement))
                .all(|b| self.can_move_box(b, movement))
        {
            self.boxes
                .iter()
                .filter(|&b| b.borrow().col != prev_col || b.borrow().row != prev_row)
                .filter(|&b| b.borrow().will_hit(next_col, next_row, movement))
                .for_each(|next_box| {
                    self.move_box(next_box, movement);
                });

            wh_box.borrow_mut().col = next_col;
            wh_box.borrow_mut().row = next_row;
            Some((prev_col, prev_row))
        } else {
            None
        }
    }

    fn step(mut self, movement: Movement) -> Self {
        let (next_col, next_row) = movement.next_position((self.robot.col, self.robot.row));
        if let Some(next_box) = self
            .boxes
            .iter()
            .find(|&b| b.borrow().is_in(next_col, next_row))
        {
            if let Some((_new_col, new_row)) = self.move_box(next_box, &movement) {
                self.robot.col = (self.robot.col as isize + movement.delta().0) as usize;
                self.robot.row = new_row;
            }
        } else {
            if self.tile_at(next_col, next_row).is_walkable() {
                self.robot.col = (self.robot.col as isize + movement.delta().0) as usize;
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

impl Robot {
    fn to_string(&self) -> String {
        "@".to_string()
    }

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
    let warehouse = Warehouse::from(input.as_str(), false).walk();
    println!("Part 1 = {}", warehouse.gps());
    let warehouse = Warehouse::from(input.as_str(), true).walk();
    println!("Part 2 = {}", warehouse.gps());
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
        let warehouse = Warehouse::from(input, false);
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
        let warehouse = Warehouse::from(input, false);
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
        let warehouse = Warehouse::from(input, false).walk();
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
        let warehouse = Warehouse::from(input, false).walk();
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
        let warehouse = Warehouse::from(input, false).walk();
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
        let warehouse = Warehouse::from(input, false).walk();
        assert_eq!(warehouse.gps(), 10092);
    }

    #[test]
    fn test_to_string_scaled() {
        let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
        let warehouse = Warehouse::from(input, true);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##..........##
##....[][]@.##
##....[]....##
##..........##
##############"#
        );
    }

    #[test]
    fn test_step_scaled() {
        let input = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"#;
        let mut warehouse = Warehouse::from(input, true);
        warehouse = warehouse.step(Movement::West);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##..........##
##...[][]@..##
##....[]....##
##..........##
##############"#
        );
        warehouse = warehouse.step(Movement::South);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[].@..##
##..........##
##############"#
        );
        warehouse = warehouse.step(Movement::South);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.......@..##
##############"#
        );
        warehouse = warehouse.step(Movement::West);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##......@...##
##############"#
        );
        warehouse = warehouse.step(Movement::West);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##..........##
##...[][]...##
##....[]....##
##.....@....##
##############"#
        );
        warehouse = warehouse.step(Movement::North);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############"#
        );
        warehouse = warehouse.step(Movement::North);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##.....@....##
##..........##
##############"#
        );
        warehouse = warehouse.step(Movement::West);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##....@.....##
##..........##
##############"#
        );
        warehouse = warehouse.step(Movement::West);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##...[][]...##
##....[]....##
##...@......##
##..........##
##############"#
        );
        warehouse = warehouse.step(Movement::North);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##......##..##
##...[][]...##
##...@[]....##
##..........##
##..........##
##############"#
        );
        warehouse = warehouse.step(Movement::North);
        assert_eq!(
            warehouse.to_string(),
            r#"##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############"#
        );
    }

    #[test]
    fn part2() {
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
        let mut warehouse = Warehouse::from(input, true);
        assert_eq!(
            warehouse.to_string(),
            r#"####################
##....[]....[]..[]##
##............[]..##
##..[][]....[]..[]##
##....[]@.....[]..##
##[]##....[]......##
##[]....[]....[]..##
##..[][]..[]..[][]##
##........[]......##
####################"#
        );
        warehouse = warehouse.walk();
        assert_eq!(
            warehouse.to_string(),
            r#"####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################"#
        );
        assert_eq!(warehouse.gps(), 9021);
    }
}
