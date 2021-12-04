use std::fs::read_to_string;

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}

#[derive(Copy, Clone, Default)]
struct MarkableNumber {
    num: u32,
    marked: bool,
}

type Board = Vec<Vec<MarkableNumber>>;

fn bingo() -> (Vec<u32>, Vec<Board>) {
    let file = read_to_string("input.txt").unwrap();
    let mut lines = file.lines();
    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut boards: Vec<Board> = Vec::new();

    // build all the boards
    while let Some(_) = lines.next() {
        let mut board: Board = Board::new();
        for _ in 0..5 {
            let line = lines.next().unwrap();
            board.push(
                line.split_whitespace()
                    .map(|s| MarkableNumber {
                        num: s.parse().unwrap(),
                        marked: false,
                    })
                    .collect(),
            );
        }

        boards.push(board);
    }

    (draws, boards)
}

fn board_won(board: &mut Board, draw: u32) -> bool {
    board
        .iter_mut()
        .flatten()
        .filter(|n| n.num == draw)
        .for_each(|n| n.marked = true);

    // check if there's a winner
    for i in 0..5 {
        let mut marked_col = 0;
        let mut marked_line = 0;

        for j in 0..5 {
            if board[i][j].marked {
                marked_col += 1;
            }

            if board[j][i].marked {
                marked_line += 1;
            }
        }

        if marked_col == 5 || marked_line == 5 {
            return true;
        }
    }

    false
}

fn part_one() -> u32 {
    let (draws, mut boards) = bingo();

    for draw in draws {
        for board in &mut boards {
            if board_won(board, draw) {
                let unmarked_sum = board
                    .iter()
                    .flatten()
                    .filter_map(|n| (!n.marked).then(|| n.num))
                    .sum::<u32>();

                return unmarked_sum * draw;
            }
        }
    }

    unreachable!()
}

fn part_two() -> u32 {
    let (draws, mut boards) = bingo();
    let mut new_wins = Vec::new();
    let mut last_board = Board::new();
    let mut last_draw = 0;
    for draw in draws {
        for (board_number, board) in boards.iter_mut().enumerate() {
            if board_won(board, draw) {
                new_wins.push(board_number);
            }
        }

        // the last board won
        if boards.len() == 1 && new_wins.len() == 1 {
            last_board = boards[0].clone();
            last_draw = draw;
            break;
        }

        new_wins.sort_unstable();
        for board_number in new_wins.iter().rev() {
            boards.remove(*board_number);
        }
        new_wins.clear();
    }

    let unmarked_sum = last_board
        .iter()
        .flatten()
        .filter_map(|n| (!n.marked).then(|| n.num))
        .sum::<u32>();

    unmarked_sum * last_draw
}
