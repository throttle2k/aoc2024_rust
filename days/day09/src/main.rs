use common::read_input;

#[derive(Debug, Clone)]
enum Space {
    File(usize),
    Empty,
}

impl ToString for Space {
    fn to_string(&self) -> String {
        match self {
            Space::File(id) => format!("{id}"),
            Space::Empty => format!("."),
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    kind: Space,
    size: usize,
}

impl ToString for Block {
    fn to_string(&self) -> String {
        self.kind.to_string().repeat(self.size)
    }
}

impl Block {
    fn new(kind: Space, size: usize) -> Self {
        Self { kind, size }
    }
}

#[derive(Debug)]
struct DiskMap {
    disk: Vec<Block>,
}

impl ToString for DiskMap {
    fn to_string(&self) -> String {
        self.disk.iter().map(|b| b.to_string()).collect()
    }
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let (mut disk, _) = value.trim().chars().enumerate().fold(
            (Vec::<Block>::new(), 0),
            |(mut disk, mut count), (idx, c)| {
                if idx % 2 == 0 {
                    disk.push(Block::new(
                        Space::File(count),
                        c.to_digit(10).unwrap() as usize,
                    ));
                    count += 1;
                } else {
                    disk.push(Block::new(Space::Empty, c.to_digit(10).unwrap() as usize));
                }
                (disk, count)
            },
        );
        if matches!(disk.last().unwrap().kind, Space::File(_)) {
            disk.push(Block::new(Space::Empty, 0))
        };
        Self { disk }
    }
}

impl DiskMap {
    fn move_data(&mut self, from: usize, to: usize) {
        let mut file = self.disk.remove(from);
        let space = self.disk.remove(to);
        if space.size < file.size {
            file.size -= space.size;
            self.disk
                .insert(to, Block::new(file.kind.clone(), space.size));
            self.disk.insert(from, file.clone());
            self.disk
                .insert(from + 1, Block::new(Space::Empty, space.size));
        } else {
            self.disk.insert(to, Block::new(file.kind, file.size));
            self.disk
                .insert(to + 1, Block::new(Space::Empty, space.size - file.size));
            self.disk.insert(from, Block::new(Space::Empty, file.size));
        };
    }

    fn next_file_to_move(&self, defrag: bool) -> Option<usize> {
        if defrag {
            self.disk
                .iter()
                .enumerate()
                .rev()
                .find(|(file_idx, b)| match b {
                    Block {
                        kind: Space::File(_),
                        size: s,
                    } => self
                        .disk
                        .iter()
                        .enumerate()
                        .find(|(space_idx, b)| match b {
                            Block {
                                kind: Space::Empty,
                                size: sf,
                            } if *sf >= *s => space_idx < file_idx,
                            _ => false,
                        })
                        .is_some(),
                    _ => false,
                })
                .map(|(idx, _)| idx)
        } else {
            self.disk
                .iter()
                .enumerate()
                .rev()
                .find(|(_, b)| {
                    matches!(
                        b,
                        Block {
                            kind: Space::File(_),
                            size: _
                        }
                    )
                })
                .map(|(idx, _)| idx)
        }
    }

    fn next_space(&self, size: usize, defrag: bool) -> Option<usize> {
        if defrag {
            (0..self.disk.len())
                .skip_while(|idx| match self.disk.get(*idx) {
                    Some(Block {
                        kind: Space::Empty,
                        size: s,
                    }) if *s >= size => false,
                    _ => true,
                })
                .next()
        } else {
            (0..self.disk.len())
                .skip_while(|idx| {
                    matches!(
                        self.disk.get(*idx),
                        Some(Block {
                            kind: Space::File(_),
                            size: _
                        })
                    )
                })
                .next()
        }
    }

    fn compact(&mut self) {
        let mut from = self.next_file_to_move(false);
        if from.is_none() {
            return;
        }
        let mut to = self.next_space(self.disk.get(from.unwrap()).unwrap().size, false);
        while from.is_some() && to.is_some() {
            let from_idx = from.unwrap();
            let to_idx = to.unwrap();
            if from_idx < to_idx {
                return;
            }
            self.move_data(from_idx, to_idx);
            from = self.next_file_to_move(false);
            if from.is_none() {
                return;
            }
            to = self.next_space(self.disk.get(from.unwrap()).unwrap().size, false);
        }
    }

    fn defrag(&mut self) {
        (1..self.disk.len() - 1)
            .rev()
            .for_each(|idx| match self.disk.get(idx).unwrap() {
                Block {
                    kind: Space::File(_),
                    size: s,
                } => {
                    if let Some(to) = self.next_space(*s, true) {
                        if to < idx {
                            self.move_data(idx, to);
                        }
                    }
                }
                _ => (),
            });
    }

    fn checksum(&self) -> usize {
        let mut count = 0;
        self.disk
            .iter()
            .filter_map(|block| match block.kind {
                Space::File(id) => {
                    let sum = (0..block.size)
                        .map(|_| {
                            let val = count * id;
                            count += 1;
                            val
                        })
                        .sum::<usize>();
                    Some(sum)
                }
                Space::Empty => {
                    count += block.size;
                    None
                }
            })
            .sum()
    }
}

fn main() {
    let input = read_input("day09.txt");
    let mut disk_map = DiskMap::from(input.as_str());
    disk_map.compact();
    println!("Part 1 = {}", disk_map.checksum());
    let mut disk_map = DiskMap::from(input.as_str());
    disk_map.defrag();
    println!("Part 2 = {}", disk_map.checksum());
}

#[cfg(test)]
mod day09_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "12345", "2333133121414131402" },
        expected = { "0..111....22222", "00...111...2...333.44.5555.6666.777.888899" }
    )]
    fn test_parse_input(input: &str, expected: &str) {
        let disk_map = DiskMap::from(input);
        assert_eq!(disk_map.to_string(), expected);
    }

    #[parameterized(
        input = { "12345", "2333133121414131402" },
        expected = { "022111222......", "0099811188827773336446555566.............." }
    )]
    fn test_compact(input: &str, expected: &str) {
        let mut disk_map = DiskMap::from(input);
        disk_map.compact();
        assert_eq!(disk_map.to_string(), expected);
    }

    #[test]
    fn part1() {
        let input = "2333133121414131402";
        let mut disk_map = DiskMap::from(input);
        disk_map.compact();
        assert_eq!(1928, disk_map.checksum());
    }

    #[test]
    fn part2() {
        let input = "2333133121414131402";
        let mut disk_map = DiskMap::from(input);
        disk_map.defrag();
        assert_eq!(2858, disk_map.checksum());
    }

    #[parameterized(
        input = { "12345", "123456" },
        expected = { Some(4), Some(4) }
    )]
    fn test_find_next_file(input: &str, expected: Option<usize>) {
        let disk_map = DiskMap::from(input);
        assert_eq!(disk_map.next_file_to_move(false), expected);
    }

    #[test]
    fn test_find_next_file_defrag() {
        let disk_map = DiskMap::from("1351346");
        assert_eq!(disk_map.next_file_to_move(true), Some(4));
    }

    #[parameterized(
        input = { "12345", "123456" },
        expected = { Some(1), Some(1) }
    )]
    fn test_next_space(input: &str, expected: Option<usize>) {
        let disk_map = DiskMap::from(input);
        assert_eq!(disk_map.next_space(5, false), expected);
    }

    #[parameterized(
        input = { "12345", "123456" },
        expected = { Some(3), Some(3) }
    )]
    fn test_next_space_defrag(input: &str, expected: Option<usize>) {
        let disk_map = DiskMap::from(input);
        assert_eq!(disk_map.next_space(3, true), expected);
    }

    #[test]
    fn test_defrag() {
        let input = "2333133121414131402";
        let mut disk_map = DiskMap::from(input);
        disk_map.defrag();
        assert_eq!(
            disk_map.to_string(),
            "00992111777.44.333....5555.6666.....8888.."
        )
    }
}
