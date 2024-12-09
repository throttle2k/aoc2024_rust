use common::read_input;

#[derive(Debug, PartialEq)]
struct File(usize);

impl ToString for File {
    fn to_string(&self) -> String {
        format!("[{}]", self.0)
    }
}

#[derive(Debug)]
struct DiskMap {
    disk: Vec<Option<File>>,
}

impl ToString for DiskMap {
    fn to_string(&self) -> String {
        self.disk
            .iter()
            .map(|f| match f {
                Some(file) => file.to_string(),
                None => "[_]".to_string(),
            })
            .collect()
    }
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let (disk, _) = value.trim().chars().enumerate().fold(
            (Vec::<Option<File>>::new(), 0),
            |(mut disk, mut count), (idx, c)| {
                if idx % 2 == 0 {
                    (0..c.to_digit(10).unwrap()).for_each(|_| disk.push(Some(File(count))));
                    count += 1;
                } else {
                    (0..c.to_digit(10).unwrap()).for_each(|_| disk.push(None));
                }
                (disk, count)
            },
        );
        Self { disk }
    }
}

impl DiskMap {
    fn defrag(&mut self) {
        let mut head_pointer = 0;
        let mut tail_pointer = self.disk.len() - 1;
        while head_pointer < tail_pointer {
            if let Some(file) = self.disk.get(head_pointer) {
                if file.is_some() {
                    head_pointer += 1;
                } else {
                    while tail_pointer > head_pointer && self.disk.get(tail_pointer).is_none() {
                        tail_pointer -= 1;
                    }
                    if tail_pointer > head_pointer {
                        self.disk.swap(head_pointer, tail_pointer);
                        tail_pointer -= 1;
                    }
                }
                // println!("{}", self.to_string());
                // let head_spaces = "   ".to_string().repeat(head_pointer);
                // let tail_spaces = "   ".to_string().repeat(tail_pointer);
                // println!("{head_spaces} H");
                // println!("{tail_spaces} T");
            }
        }
    }

    fn checksum(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .filter_map(|(idx, file)| match file {
                Some(f) => Some(idx * f.0),
                None => None,
            })
            .sum()
    }
}

fn main() {
    let input = read_input("day09.txt");
    let mut disk_map = DiskMap::from(input.as_str());
    disk_map.defrag();
    println!("Part 1 = {}", disk_map.checksum());
}

#[cfg(test)]
mod day09_tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(
        input = { "12345", "2333133121414131402" },
        expected = { "[0][_][_][1][1][1][_][_][_][_][2][2][2][2][2]", "[0][0][_][_][_][1][1][1][_][_][_][2][_][_][_][3][3][3][_][4][4][_][5][5][5][5][_][6][6][6][6][_][7][7][7][_][8][8][8][8][9][9]" }
    )]
    fn test_parse_input(input: &str, expected: &str) {
        let disk_map = DiskMap::from(input);
        assert_eq!(disk_map.to_string(), expected);
    }

    #[parameterized(
        input = { "12345", "2333133121414131402" },
        expected = { "[0][2][2][1][1][1][2][2][2][_][_][_][_][_][_]", "[0][0][9][9][8][1][1][1][8][8][8][2][7][7][7][3][3][3][6][4][4][6][5][5][5][5][6][6][_][_][_][_][_][_][_][_][_][_][_][_][_][_]" }
    )]
    fn test_defrag(input: &str, expected: &str) {
        let mut disk_map = DiskMap::from(input);
        disk_map.defrag();
        assert_eq!(disk_map.to_string(), expected);
    }

    #[test]
    fn part1() {
        let input = "2333133121414131402";
        let mut disk_map = DiskMap::from(input);
        disk_map.defrag();
        assert_eq!(1928, disk_map.checksum());
    }
}
