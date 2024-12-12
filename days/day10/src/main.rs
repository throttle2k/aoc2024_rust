use common::read_input;

#[derive(Debug)]
struct Spot {
    row: usize,
    col: usize,
    height: usize,
}

#[derive(Debug)]
struct TopographicMap {
    rows: usize,
    cols: usize,
    spots: Vec<Spot>,
}

impl From<&str> for TopographicMap {
    fn from(value: &str) -> Self {
        let rows = value.trim().lines().count();
        let cols = value.trim().lines().nth(0).unwrap().len();
        let spots = value
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, l)| {
                l.chars()
                    .enumerate()
                    .map(|(col, c)| Spot {
                        row,
                        col,
                        height: c.to_digit(10).unwrap() as usize,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { rows, cols, spots }
    }
}

impl TopographicMap {
    fn height_of(&self, (spot_row, spot_col): (usize, usize)) -> usize {
        let idx = spot_row * self.cols + spot_col;
        self.spots.get(idx).unwrap().height
    }

    fn viable_neighbors(&self, (spot_row, spot_col): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(isize, isize)> = vec![];
        if spot_row > 0 {
            neighbors.push((-1, 0));
        };
        if spot_row < self.rows - 1 {
            neighbors.push((1, 0));
        };
        if spot_col > 0 {
            neighbors.push((0, -1));
        };
        if spot_col < self.cols - 1 {
            neighbors.push((0, 1));
        };
        neighbors
            .iter()
            .filter_map(|(delta_row, delta_col)| {
                let neighbor = (
                    (spot_row as isize + delta_row) as usize,
                    (spot_col as isize + delta_col) as usize,
                );
                if self.height_of(neighbor) == self.height_of((spot_row, spot_col)) + 1 {
                    Some(neighbor)
                } else {
                    None
                }
            })
            .collect()
    }

    fn trail_heads(&self) -> Vec<(usize, usize)> {
        self.spots
            .iter()
            .filter(|spot| spot.height == 0)
            .map(|spot| (spot.row, spot.col))
            .collect()
    }

    fn walk(&self, trail_head: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut queue = vec![(trail_head)];
        while !queue.is_empty() {
            let current_spot = queue.remove(0);
            if self.height_of(current_spot) == 9 {
                result.push(current_spot);
            }
            self.viable_neighbors(current_spot)
                .iter()
                .for_each(|neighbor| {
                    queue.push(neighbor.clone());
                });
        }
        result
    }

    fn get_trail_head_score(&self, trail_head: (usize, usize)) -> usize {
        let mut paths = self.walk(trail_head);
        paths.sort();
        paths.dedup();
        paths.len()
    }

    fn get_trail_head_ranking(&self, trail_head: (usize, usize)) -> usize {
        self.walk(trail_head).len()
    }

    fn sum_trail_head_scores(&self) -> usize {
        self.trail_heads()
            .iter()
            .map(|trail_head| self.get_trail_head_score(*trail_head))
            .sum()
    }

    fn sum_trail_head_rankings(&self) -> usize {
        self.trail_heads()
            .iter()
            .map(|trail_head| self.get_trail_head_ranking(*trail_head))
            .sum()
    }
}

fn main() {
    let input = read_input("day10.txt");
    let topographic_map = TopographicMap::from(input.as_str());
    println!("Part 1 = {}", topographic_map.sum_trail_head_scores());
    println!("Part 2 = {}", topographic_map.sum_trail_head_rankings());
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn test_walk() {
        let input = r#"9990999
9991999
9992999
6543456
7111117
8111118
9111119"#;
        let topographic_map = TopographicMap::from(input);
        assert_eq!(topographic_map.walk((0, 3)), vec![(6, 0), (6, 6)]);
    }

    #[test]
    fn test_get_trail_head_score() {
        let input = r#"9190919
9991598
9992997
6543456
7651987
8761111
9871999"#;
        let topographic_map = TopographicMap::from(input);
        assert_eq!(topographic_map.get_trail_head_score((0, 3)), 4);
    }

    #[test]
    fn test_get_multiple_trail_head_scores() {
        let input = r#"1091911
2991819
3999799
4567654
9918193
9919192
9991901"#;
        let topographic_map = TopographicMap::from(input);
        assert_eq!(topographic_map.get_trail_head_score((0, 1)), 1);
        assert_eq!(topographic_map.get_trail_head_score((6, 5)), 2);
    }

    #[test]
    fn part1() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let topographic_map = TopographicMap::from(input);
        assert_eq!(topographic_map.sum_trail_head_scores(), 36);
    }

    #[test]
    fn test_get_trail_ranking() {
        let input = r#"9999909
9943219
9959929
9965439
9979949
9187659
9191111"#;
        let topographic_map = TopographicMap::from(input);
        assert_eq!(topographic_map.get_trail_head_ranking((0, 5)), 3);
    }

    #[test]
    fn part2() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        let topographic_map = TopographicMap::from(input);
        assert_eq!(topographic_map.sum_trail_head_rankings(), 81);
    }
}
