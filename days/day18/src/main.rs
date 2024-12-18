use common::read_input;

#[derive(Debug, Clone)]
enum MemoryCell {
    Sane,
    Corrupted,
}

impl ToString for MemoryCell {
    fn to_string(&self) -> String {
        match self {
            MemoryCell::Sane => String::from("."),
            MemoryCell::Corrupted => String::from("#"),
        }
    }
}

#[derive(Debug)]
struct Memory {
    cols: usize,
    rows: usize,
    start: (usize, usize),
    target: (usize, usize),
    cells: Vec<Vec<MemoryCell>>,
    corruption: Vec<(usize, usize)>,
}

impl ToString for Memory {
    fn to_string(&self) -> String {
        self.cells
            .iter()
            .map(|row| row.iter().map(|tile| tile.to_string()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Memory {
    fn new(cols: usize, rows: usize, corruption: Vec<(usize, usize)>) -> Self {
        Self {
            cols,
            rows,
            start: (0, 0),
            target: (cols - 1, rows - 1),
            cells: vec![vec![MemoryCell::Sane; cols]; rows],
            corruption,
        }
    }

    fn drop(&mut self) {
        let loc = self.corruption.remove(0);
        self.cells[loc.1][loc.0] = MemoryCell::Corrupted
    }

    fn neighbors(&self, (col, row): (usize, usize)) -> Vec<(usize, usize)> {
        let mut deltas = vec![];
        if col > 0 {
            deltas.push((-1, 0));
        }
        if col < self.cols - 1 {
            deltas.push((1, 0));
        }
        if row > 0 {
            deltas.push((0, -1));
        }
        if row < self.rows - 1 {
            deltas.push((0, 1));
        }
        deltas
            .iter()
            .filter_map(|(delta_col, delta_row)| {
                let (neighbor_col, neighbor_row) = (
                    (col as isize + delta_col) as usize,
                    (row as isize + delta_row) as usize,
                );
                match self.cells[neighbor_row][neighbor_col] {
                    MemoryCell::Sane => Some((neighbor_col, neighbor_row)),
                    MemoryCell::Corrupted => None,
                }
            })
            .collect()
    }

    fn escape(&mut self) -> usize {
        let mut visited = vec![];
        let mut queue = vec![(0, self.start)];
        while !queue.is_empty() {
            let (depth, current) = queue.remove(0);
            if current == self.target {
                return depth;
            }

            if !visited.contains(&current) {
                visited.push(current.clone());
                self.neighbors(current).iter().for_each(|neighbor| {
                    queue.push((depth + 1, neighbor.clone()));
                });
            }
        }
        unreachable!()
    }
}

fn main() {
    let input = read_input("day18.txt");
    let input = input
        .trim()
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(col, row)| (col.parse().unwrap(), row.parse().unwrap()))
        .collect::<Vec<(usize, usize)>>();
    let mut memory = Memory::new(71, 71, input);
    (0..1024).for_each(|_| {
        memory.drop();
    });
    println!("Part 1 = {}", memory.escape());
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn test_drop() {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        let input = input
            .trim()
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(col, row)| (col.parse().unwrap(), row.parse().unwrap()))
            .collect::<Vec<(usize, usize)>>();
        let mut memory = Memory::new(7, 7, input);
        (0..12).for_each(|_| {
            memory.drop();
        });
        assert_eq!(
            memory.to_string(),
            r#"...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#...."#
        );
    }

    #[test]
    fn part1() {
        let input = r#"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"#;
        let input = input
            .trim()
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(col, row)| (col.parse().unwrap(), row.parse().unwrap()))
            .collect::<Vec<(usize, usize)>>();
        let mut memory = Memory::new(7, 7, input);
        (0..12).for_each(|_| {
            memory.drop();
        });
        assert_eq!(memory.escape(), 22);
    }
}
