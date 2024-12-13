use common::read_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Region {
    plant: char,
    plots: Vec<(usize, usize)>,
    area: usize,
    perimeter: usize,
}

impl Region {
    fn new(plant: char, mut plots: Vec<(usize, usize)>) -> Self {
        plots.sort();
        let (v_fences, h_fences) = plots.iter().fold(
            (0, 0),
            |(mut v_fences, mut h_fences), (plot_row, plot_col)| {
                if *plot_row == 0 {
                    h_fences += 1;
                } else if !plots.contains(&(plot_row - 1, *plot_col)) {
                    h_fences += 1;
                }
                if *plot_col == 0 {
                    v_fences += 1;
                } else if !plots.contains(&(*plot_row, plot_col - 1)) {
                    v_fences += 1;
                }
                if !plots.contains(&(plot_row + 1, *plot_col)) {
                    h_fences += 1;
                }
                if !plots.contains(&(*plot_row, plot_col + 1)) {
                    v_fences += 1;
                }
                (v_fences, h_fences)
            },
        );
        let area = plots.len();
        let perimeter = v_fences + h_fences;
        Self {
            plant,
            plots,
            area,
            perimeter,
        }
    }

    fn from(garden: &[&[char]], start: (usize, usize)) -> Self {
        let plant = garden[start.0][start.1];
        let mut queue = vec![start];
        let mut visited = vec![start];
        while !queue.is_empty() {
            let plot = queue.remove(0);
            neighbors(garden, plot).iter().for_each(|neighbor| {
                if !visited.contains(neighbor) {
                    queue.push(*neighbor);
                    visited.push(*neighbor);
                }
            });
        }
        Self::new(plant, visited)
    }

    fn fence_price(&self) -> usize {
        self.area * self.perimeter
    }
}

fn neighbors(garden: &[&[char]], (plot_row, plot_col): (usize, usize)) -> Vec<(usize, usize)> {
    let rows = garden.len();
    let cols = garden[0].len();
    let plant = garden[plot_row][plot_col];
    let mut deltas: Vec<(isize, isize)> = vec![];
    if plot_row > 0 {
        deltas.push((-1, 0));
    };
    if plot_row < rows - 1 {
        deltas.push((1, 0));
    };
    if plot_col > 0 {
        deltas.push((0, -1));
    };
    if plot_col < cols - 1 {
        deltas.push((0, 1));
    };
    let valid_neighbors =
        deltas
            .iter()
            .fold(vec![], |mut valid_neighbors, (delta_row, delta_col)| {
                let neighbor_row = (plot_row as isize + delta_row) as usize;
                let neighbor_col = (plot_col as isize + delta_col) as usize;
                if garden[neighbor_row][neighbor_col] == plant {
                    valid_neighbors.push((neighbor_row, neighbor_col));
                };
                valid_neighbors
            });
    valid_neighbors
}

#[derive(Debug)]
struct Garden {
    regions: Vec<Region>,
}

impl From<&str> for Garden {
    fn from(value: &str) -> Self {
        let plots = value
            .trim()
            .lines()
            .map(|l| l.trim().chars().map(|c| c).collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let plots: Vec<&[char]> = plots.iter().map(|plot| plot.as_slice()).collect();
        let mut regions = plots
            .iter()
            .enumerate()
            .flat_map(|(row, r)| {
                r.iter()
                    .enumerate()
                    .map(|(col, _)| Region::from(&plots, (row, col)))
                    .collect::<Vec<Region>>()
            })
            .collect::<Vec<Region>>();
        regions.sort();
        regions.dedup();
        Self { regions }
    }
}

impl Garden {
    fn fence_price(&self) -> usize {
        self.regions.iter().map(|r| r.fence_price()).sum()
    }
}

fn main() {
    let input = read_input("day12.txt");
    let garden = Garden::from(input.as_str());
    println!("Part 1 = {}", garden.fence_price());
}

#[cfg(test)]
mod day12_tests {
    use parameterized::parameterized;

    use super::*;

    #[test]
    fn test_parse_input_1() {
        let input = r#"AA
AA"#;
        let garden = Garden::from(input);
        assert_eq!(garden.regions.len(), 1);
        let region = garden.regions.get(0).unwrap();
        assert_eq!(region.perimeter, 8);
        assert_eq!(region.area, 4);
    }

    #[test]
    fn test_parse_input_2() {
        let input = r#"AAAA
BBBB"#;
        let garden = Garden::from(input);
        assert_eq!(garden.regions.len(), 2);
        let region1 = garden.regions.get(0).unwrap();
        assert_eq!(region1.perimeter, 10);
        assert_eq!(region1.area, 4);
        let region2 = garden.regions.get(1).unwrap();
        assert_eq!(region2.perimeter, 10);
        assert_eq!(region2.area, 4);
    }

    #[test]
    fn test_parse_input_3() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        let garden = Garden::from(input);
        assert_eq!(garden.regions.len(), 5);
    }

    #[parameterized(
        input = { vec![(0,0),(0,1),(0,2),(0,3)], vec![(0,0),(0,1),(1,0),(1,1)], vec![(0,0),(1,0),(1,1),(2,1)], vec![(0,0)], vec![(0,0),(0,1),(0,2)] },
        expected = { 40, 32, 40, 4, 24 }
    )]
    fn test_region_fence_price(input: Vec<(usize, usize)>, expected: usize) {
        let region = Region::new('A', input);
        assert_eq!(region.fence_price(), expected);
    }

    #[test]
    fn test_fence_price_1() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        let garden = Garden::from(input);
        assert_eq!(garden.fence_price(), 140);
    }

    #[test]
    fn test_fence_price_2() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        let garden = Garden::from(input);
        assert_eq!(garden.fence_price(), 772);
    }

    #[test]
    fn part1() {
        let input = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        let garden = Garden::from(input);
        assert_eq!(garden.fence_price(), 1930);
    }
}
