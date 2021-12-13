use std::fs;

#[derive(Debug)]
struct Board {
    board1: Vec<Vec<i32>>,
    board2: Vec<Vec<i32>>,
}

trait DeepCopy {
    fn deep_copy(&self) -> Board;
}

impl DeepCopy for Board {
    fn deep_copy(&self) -> Board {
        let board1 = self.board1.iter().map(|x| x.iter().map(|&x| x).collect()).collect();
        let board2 = self.board2.iter().map(|x| x.iter().map(|&x| x).collect()).collect();
        Board {board1, board2}
    }
}

fn main() {
    let read = fs::read_to_string("input/day4.txt")
        .expect("Could not read file");

    let lines: Vec<&str> = read
        .split("\n")
        .collect();

    let input : Vec<i32> = lines[0].split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let iter = lines[2..].rsplit(|x| x == &"");

    let mut boards: Vec<Board> = vec![];

    for board in iter {
        let board1: Vec<Vec<i32>> = board
            .iter()
            .map(|x| x.split(' ').filter(|x| x != &"").map(|y| y.parse().unwrap()).collect())
            .collect();

        let board2: Vec<Vec<i32>> = vec![0,1,2,3,4]
            .iter()
            .map(|x| board1.iter().map(|y| y[*x]).collect())
            .collect();

        boards.push(Board{board1, board2});
    }

    let part1 = find_board(&boards, &input);
    println!("Day 4 part 1: {}", part1);

    let part2 = find_board2(&boards, &input);
    println!("Day 4 part 2: {}", part2);
}

fn find_board(boards: &Vec<Board>, input: &Vec<i32>) -> i32 {
    let mut boards: Vec<Board> = boards.iter().map(|x| x.deep_copy()).collect();

    for number in input.iter() {
        for board in boards.iter_mut() {
            for row in board.board1.iter_mut() {
                row.retain(|&x| x != *number);
            }
            if board.board1.iter().any(|x| x.len() == 0) {
                let score: i32 = board.board1.iter().map(|x| x.iter().sum::<i32>()).sum();
                return score * number;
            }
            for column in board.board2.iter_mut() {
                column.retain(|&x| x != *number);
            }
            if board.board2.iter().any(|x| x.len() == 0) {
                let score: i32 = board.board2.iter().map(|x| x.iter().sum::<i32>()).sum();
                return score * number;
            }
        }
    }
    0
}

fn find_board2(boards: &Vec<Board>, input: &Vec<i32>) -> i32 {
    let mut boards: Vec<Board> = boards.iter().map(|x| x.deep_copy()).collect();

    let mut result = 0;

    for number in input.iter() {
        for board in boards.iter_mut() {
            if board.board1.iter().any(|x| x.len() == 0) || board.board2.iter().any(|x| x.len() == 0) {
                continue;
            }
            for row in board.board1.iter_mut() {
                row.retain(|&x| x != *number);
            }
            if board.board1.iter().any(|x| x.len() == 0) {
                let score: i32 = board.board1.iter().map(|x| x.iter().sum::<i32>()).sum();
                result = score * number;
                continue;
            }
            for column in board.board2.iter_mut() {
                column.retain(|&x| x != *number);
            }
            if board.board2.iter().any(|x| x.len() == 0) {
                let score: i32 = board.board2.iter().map(|x| x.iter().sum::<i32>()).sum();
                result = score * number;
                continue;
            }
        }
    }
    result
}