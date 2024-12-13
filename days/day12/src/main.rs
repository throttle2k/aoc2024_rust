use common::read_input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Region {
    plant: char,
    plots: Vec<(usize, usize)>,
    area: usize,
    perimeter: usize,
}

impl Region {
    fn new(plant: char, mut plots: Vec<(usize, usize)>, as_sides: bool) -> Self {
        plots.sort();
        let area = plots.len();
        let (v_fences, h_fences) = fences(&plots);
        let perimeter = if !as_sides {
            v_fences.len() + h_fences.len()
        } else {
            count_sides(v_fences, h_fences)
        };
        Self {
            plant,
            plots,
            area,
            perimeter,
        }
    }

    fn from(garden: &[&[char]], start: (usize, usize), as_sides: bool) -> Self {
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
        Self::new(plant, visited, as_sides)
    }

    fn fence_price(&self) -> usize {
        self.area * self.perimeter
    }
}

fn fences(
    plots: &Vec<(usize, usize)>,
) -> (
    Vec<((isize, isize), (isize, isize))>,
    Vec<((isize, isize), (isize, isize))>,
) {
    plots.iter().fold(
        (vec![], vec![]),
        |(mut v_fences, mut h_fences), (plot_row, plot_col)| {
            let plot_row = *plot_row as isize;
            let plot_col = *plot_col as isize;
            if plot_row == 0 {
                h_fences.push(((-1, plot_col), (0, plot_col)));
            } else if !plots.contains(&(plot_row as usize - 1, plot_col as usize)) {
                h_fences.push(((plot_row - 1, plot_col), (plot_row, plot_col)));
            }
            if plot_col == 0 {
                v_fences.push(((plot_row, -1), (plot_row, 0)));
            } else if !plots.contains(&(plot_row as usize, plot_col as usize - 1)) {
                v_fences.push(((plot_row, plot_col - 1), (plot_row, plot_col)));
            }
            if !plots.contains(&(plot_row as usize + 1, plot_col as usize)) {
                h_fences.push(((plot_row + 1, plot_col), (plot_row, plot_col)));
            }
            if !plots.contains(&(plot_row as usize, plot_col as usize + 1)) {
                v_fences.push(((plot_row, plot_col + 1), (plot_row, plot_col)));
            }
            (v_fences, h_fences)
        },
    )
}

fn fences_v_sides(fences: &mut Vec<((isize, isize), (isize, isize))>) -> usize {
    let mut sides = 0;

    while !fences.is_empty() {
        let (current_outer, current_inner) = fences.remove(0);

        let mut same_axis = fences
            .iter()
            .filter(|(other_outer, other_inner)| {
                other_outer.1 == current_outer.1 && other_inner.1 == current_inner.1
            })
            .collect::<Vec<_>>();
        same_axis.sort_by(|(a, _), (b, _)| a.0.cmp(&b.0));

        let min_index = if current_outer.0 > 0 {
            (0..current_outer.0)
                .rev()
                .take_while(|index| {
                    same_axis.contains(&&((*index, current_outer.1), (*index, current_inner.1)))
                })
                .last()
                .unwrap_or(current_outer.0)
        } else {
            current_outer.0
        };

        let max_index = (current_outer.0 + 1..)
            .take_while(|index| {
                same_axis.contains(&&((*index, current_outer.1), (*index, current_inner.1)))
            })
            .last()
            .unwrap_or(current_outer.0);

        (min_index..=max_index)
            .map(|index| ((index, current_outer.1), (index, current_inner.1)))
            .for_each(|same_side| fences.retain(|side| *side != same_side));

        sides += 1;
    }
    sides
}

fn fences_h_sides(fences: &mut Vec<((isize, isize), (isize, isize))>) -> usize {
    let mut sides = 0;

    while !fences.is_empty() {
        let (current_outer, current_inner) = fences.remove(0);

        let mut same_axis = fences
            .iter()
            .filter(|(other_outer, other_inner)| {
                other_outer.0 == current_outer.0 && other_inner.0 == current_inner.0
            })
            .collect::<Vec<_>>();
        same_axis.sort_by(|(a, _), (b, _)| a.1.cmp(&b.1));

        let min_index = if current_outer.1 > 0 {
            (0..current_outer.1)
                .rev()
                .take_while(|index| {
                    same_axis.contains(&&((current_outer.0, *index), (current_inner.0, *index)))
                })
                .last()
                .unwrap_or(current_outer.1)
        } else {
            current_outer.1
        };

        let max_index = (current_outer.1 + 1..)
            .take_while(|index| {
                same_axis.contains(&&((current_outer.0, *index), (current_inner.0, *index)))
            })
            .last()
            .unwrap_or(current_outer.1);

        (min_index..=max_index)
            .map(|index| ((current_outer.0, index), (current_inner.0, index)))
            .for_each(|same_side| fences.retain(|side| *side != same_side));

        sides += 1;
    }
    sides
}

fn count_sides(
    mut v_fences: Vec<((isize, isize), (isize, isize))>,
    mut h_fences: Vec<((isize, isize), (isize, isize))>,
) -> usize {
    let vertical_sides = fences_v_sides(&mut v_fences);

    let horizontal_sides = fences_h_sides(&mut h_fences);

    vertical_sides + horizontal_sides
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

impl Garden {
    fn from(value: &str, as_sides: bool) -> Self {
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
                    .map(|(col, _)| Region::from(&plots, (row, col), as_sides))
                    .collect::<Vec<Region>>()
            })
            .collect::<Vec<Region>>();
        regions.sort();
        regions.dedup();
        Self { regions }
    }

    fn fence_price(&self) -> usize {
        self.regions.iter().map(|r| r.fence_price()).sum()
    }
}

fn main() {
    let input = read_input("day12.txt");
    let garden = Garden::from(input.as_str(), false);
    println!("Part 1 = {}", garden.fence_price());
    let garden = Garden::from(input.as_str(), true);
    println!("Part 2 = {}", garden.fence_price());
}

#[cfg(test)]
mod day12_tests {
    use parameterized::parameterized;

    use super::*;

    #[test]
    fn test_parse_input_1() {
        let input = r#"AA
AA"#;
        let garden = Garden::from(input, false);
        assert_eq!(garden.regions.len(), 1);
        let region = garden.regions.get(0).unwrap();
        assert_eq!(region.perimeter, 8);
        assert_eq!(region.area, 4);
    }

    #[test]
    fn test_parse_input_2() {
        let input = r#"AAAA
BBBB"#;
        let garden = Garden::from(input, false);
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
        let garden = Garden::from(input, false);
        assert_eq!(garden.regions.len(), 5);
    }

    #[parameterized(
        input = { vec![(0,0),(0,1),(0,2),(0,3)], vec![(0,0),(0,1),(1,0),(1,1)], vec![(0,0),(1,0),(1,1),(2,1)], vec![(0,0)], vec![(0,0),(0,1),(0,2)] },
        expected = { 40, 32, 40, 4, 24 }
    )]
    fn test_region_fence_price(input: Vec<(usize, usize)>, expected: usize) {
        let region = Region::new('A', input, false);
        assert_eq!(region.fence_price(), expected);
    }

    #[test]
    fn test_fence_price_1() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        let garden = Garden::from(input, false);
        assert_eq!(garden.fence_price(), 140);
    }

    #[test]
    fn test_fence_price_2() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        let garden = Garden::from(input, false);
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
        let garden = Garden::from(input, false);
        assert_eq!(garden.fence_price(), 1930);
    }

    #[test]
    fn test_fences() {
        let input = vec![(0, 0), (0, 1)];
        let (v_fences, h_fences) = fences(&input);
        assert_eq!(v_fences.len(), 2);
        assert!(v_fences.contains(&((0, -1), (0, 0))));
        assert!(v_fences.contains(&((0, 2), (0, 1))));
        assert_eq!(h_fences.len(), 4);
        assert!(h_fences.contains(&((-1, 0), (0, 0))));
        assert!(h_fences.contains(&((1, 0), (0, 0))));
        assert!(h_fences.contains(&((-1, 1), (0, 1))));
        assert!(h_fences.contains(&((1, 1), (0, 1))));
    }

    #[parameterized(
        input = { vec![(0,0)], vec![(0,0), (0,1)], vec![(0,0), (0,1), (1,0), (1,1)], vec![(0,0), (1,0), (1,1), (2,1)] },
        expected_v = {2, 2, 2, 4},
    )]
    fn test_fences_v_sides(input: Vec<(usize, usize)>, expected_v: usize) {
        let (mut v_fences, _) = fences(&input);
        let v_sides = fences_v_sides(&mut v_fences);
        assert_eq!(v_sides, expected_v);
    }

    #[parameterized(
        input = { vec![(0,0)], vec![(0,0), (0,1)], vec![(0,0), (0,1), (1,0), (1,1)], vec![(0,0), (1,0), (1,1), (2,1)] },
        expected_h = {2, 2, 2, 4},
    )]
    fn test_fences_h_sides(input: Vec<(usize, usize)>, expected_h: usize) {
        let (_, mut h_fences) = fences(&input);
        let h_sides = fences_h_sides(&mut h_fences);
        assert_eq!(h_sides, expected_h);
    }

    #[parameterized(
        input = { vec![(0,0)], vec![(0,0), (0,1)], vec![(0,0), (0,1), (1,0), (1,1)], vec![(0,0), (1,0), (1,1), (2,1)] },
        expected_v = {2, 2, 2, 4},
        expected_h = {2, 2, 2, 4},
    )]
    fn test_sides(input: Vec<(usize, usize)>, expected_v: usize, expected_h: usize) {
        let (mut v_fences, mut h_fences) = fences(&input);
        let v_sides = fences_v_sides(&mut v_fences);
        let h_sides = fences_h_sides(&mut h_fences);
        assert_eq!(v_sides, expected_v);
        assert_eq!(h_sides, expected_h);
    }

    #[test]
    fn test_fence_price_as_sides_1() {
        let input = r#"AAAA
BBCD
BBCC
EEEC"#;
        let garden = Garden::from(input, true);
        assert_eq!(garden.fence_price(), 80);
    }

    #[test]
    fn test_fence_price_as_sides_2() {
        let input = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
        let garden = Garden::from(input, true);
        assert_eq!(garden.fence_price(), 236);
    }

    #[test]
    fn test_fence_price_as_sides_3() {
        let input = r#"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"#;
        let garden = Garden::from(input, true);
        assert_eq!(garden.fence_price(), 436);
    }

    #[test]
    fn test_fence_price_as_sides_4() {
        let input = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;
        let garden = Garden::from(input, true);
        assert_eq!(garden.fence_price(), 368);
    }

    #[test]
    fn test_part2() {
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
        let garden = Garden::from(input, true);
        assert_eq!(garden.fence_price(), 1206);
    }
}
