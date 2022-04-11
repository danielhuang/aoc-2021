use aoc_2021::input;
use itertools::Itertools;

fn main() {
    let input = input!("4");

    let lines = input.lines().collect_vec();

    let draws = lines[0]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec();

    let boards = input
        .split("\n\n")
        .skip(1)
        .map(|x| {
            x.lines()
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();

    let ((_, part1), (_, part2)) = boards
        .iter()
        .map(|board| get_board_result(&draws, board))
        .minmax_by_key(|x| x.0)
        .into_option()
        .unwrap();

    dbg!(part1, part2);
}

fn get_board_result(draws: &[i64], board: &[Vec<i64>]) -> (usize, i64) {
    (0..draws.len())
        .find_map(|i| {
            let partial = &draws[0..i];
            if is_board_winning(board, partial) {
                Some((
                    i,
                    board
                        .iter()
                        .flatten()
                        .filter(|&&x| !partial.contains(&x))
                        .sum::<i64>()
                        * partial.iter().last().copied().unwrap(),
                ))
            } else {
                None
            }
        })
        .unwrap()
}

fn is_board_winning(board: &[Vec<i64>], drawn_markers: &[i64]) -> bool {
    (0..5).any(|i| (0..5).all(|j| drawn_markers.contains(&board[i][j])))
        || (0..5).any(|i| (0..5).all(|j| drawn_markers.contains(&board[j][i])))
}
