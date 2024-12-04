use common::read_input;

fn get_rows(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn get_char(input: &str, (row, col): (usize, usize)) -> char {
    input.lines().nth(row).unwrap().chars().nth(col).unwrap()
}

fn get_columns(input: &str) -> Vec<String> {
    (0..input.lines().nth(0).unwrap().len())
        .map(|col| {
            (0..input.lines().count())
                .map(|row| get_char(input, (row, col)))
                .collect::<String>()
        })
        .collect()
}

fn get_diag_bl_tr(input: &str) -> Vec<String> {
    let rows = input.lines().count();
    let cols = input.lines().nth(0).unwrap().len();
    (0..rows + cols - 1)
        .map(|row| {
            (0..=row)
                .filter_map(|col| {
                    if row - col < rows && col < cols {
                        Some(get_char(input, (row - col, col)))
                    } else {
                        None
                    }
                })
                .collect::<String>()
        })
        .collect()
}

fn get_diag_br_tl(input: &str) -> Vec<String> {
    let input = input
        .lines()
        .map(|l| l.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    get_diag_bl_tr(&input)
}

#[derive(Debug)]
enum ParserState {
    ReadX,
    ReadM,
    ReadA,
    ReadS,
    Done,
}

fn transition(state: ParserState, c: char) -> ParserState {
    match (state, c) {
        (_, 'X') => ParserState::ReadM,
        (ParserState::ReadM, 'M') => ParserState::ReadA,
        (ParserState::ReadA, 'A') => ParserState::ReadS,
        (ParserState::ReadS, 'S') => ParserState::Done,
        _ => ParserState::ReadX,
    }
}

fn count_xmas_in_row(row: &str) -> usize {
    let (_, count) = row.chars().fold(
        (ParserState::ReadX, 0),
        |(state, count), c| match transition(state, c) {
            ParserState::Done => (ParserState::ReadX, count + 1),
            new_state => (new_state, count),
        },
    );
    count
}

fn count_xmas(input: &str) -> usize {
    let rows = get_rows(input);
    let columns = get_columns(input);
    let diag_1 = get_diag_bl_tr(input);
    let diag_2 = get_diag_br_tl(input);
    let mut count = 0;
    count += rows
        .iter()
        .map(|row| {
            let rev = row.chars().rev().collect::<String>();
            count_xmas_in_row(row) + count_xmas_in_row(&rev)
        })
        .sum::<usize>();
    count += columns
        .iter()
        .map(|row| {
            let rev = row.chars().rev().collect::<String>();
            count_xmas_in_row(row) + count_xmas_in_row(&rev)
        })
        .sum::<usize>();
    count += diag_1
        .iter()
        .map(|row| {
            let rev = row.chars().rev().collect::<String>();
            count_xmas_in_row(row) + count_xmas_in_row(&rev)
        })
        .sum::<usize>();
    count += diag_2
        .iter()
        .map(|row| {
            let rev = row.chars().rev().collect::<String>();
            count_xmas_in_row(row) + count_xmas_in_row(&rev)
        })
        .sum::<usize>();
    count
}

fn count_x_mas(input: &str) -> usize {
    let input = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let rows = input.len();
    let cols = input.first().unwrap().len();
    (1..rows - 1)
        .map(|row| {
            (1..cols - 1)
                .map(move |col| (row, col))
                .filter(|(row, col)| *input.get(*row).unwrap().get(*col).unwrap() == 'A')
                .filter(|(row, col)| {
                    (*input.get(row - 1).unwrap().get(col - 1).unwrap() == 'M'
                        && *input.get(row + 1).unwrap().get(col + 1).unwrap() == 'S')
                        || (*input.get(row - 1).unwrap().get(col - 1).unwrap() == 'S'
                            && *input.get(row + 1).unwrap().get(col + 1).unwrap() == 'M')
                })
                .filter(|(row, col)| {
                    (*input.get(row - 1).unwrap().get(col + 1).unwrap() == 'M'
                        && *input.get(row + 1).unwrap().get(col - 1).unwrap() == 'S')
                        || (*input.get(row - 1).unwrap().get(col + 1).unwrap() == 'S'
                            && *input.get(row + 1).unwrap().get(col - 1).unwrap() == 'M')
                })
                .count()
        })
        .sum()
}

fn main() {
    let input = read_input("day04.txt");
    println!("Part 1 = {}", count_xmas(input.as_str()));
    println!("Part 2 = {}", count_x_mas(input.as_str()));
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    #[test]
    fn test_get_rows() {
        let input = r#"XMAS
MASX
SAXM"#;
        assert_eq!(
            get_rows(input),
            vec!["XMAS".to_string(), "MASX".to_string(), "SAXM".to_string()]
        );
    }

    #[test]
    fn test_get_columns() {
        let input = r#"XMAS
MASX
SAXM"#;
        assert_eq!(
            get_columns(input),
            vec![
                "XMS".to_string(),
                "MAA".to_string(),
                "ASX".to_string(),
                "SXM".to_string()
            ]
        );
    }

    #[test]
    fn test_get_diag_bl_tr() {
        let input = r#"XMAS
MASX
SAXM"#;
        assert_eq!(
            get_diag_bl_tr(input),
            vec![
                "X".to_string(),
                "MM".to_string(),
                "SAA".to_string(),
                "ASS".to_string(),
                "XX".to_string(),
                "M".to_string()
            ]
        );
    }

    #[test]
    fn test_get_diag_br_tl() {
        let input = r#"XMAS
MASX
SAXM"#;
        assert_eq!(
            get_diag_br_tl(input),
            vec![
                "S".to_string(),
                "XA".to_string(),
                "MSM".to_string(),
                "XAX".to_string(),
                "AM".to_string(),
                "S".to_string()
            ]
        );
    }

    #[test]
    fn part1() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(count_xmas(input), 18);
    }

    #[test]
    fn part2() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(count_x_mas(input), 9);
    }
}
