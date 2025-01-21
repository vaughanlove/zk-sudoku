use sudoku::core::board::{Board, Difficulty};
use sudoku::core::solver::DancingLinks;
fn main() {
    let mut board = Board::from_seed(666, Some(Difficulty::Medium));
    println!("Board generated! {}", board);

    let mut dl = DancingLinks::new();
    dl.init_header_row();
    dl.init_constraint_matrix();
    let sol = dl.solve_with_partial(&board).unwrap();
    let solution_board = DancingLinks::to_sudoku_board(sol);

    let user_input = vec![
        7, 5, 3, 8, 2, 1, 6, 9, 4, 1, 2, 4, 3, 6, 9, 5, 7, 8, 6, 8, 9, 4, 5, 7, 1, 2, 3, 2, 9, 1,
        5, 7, 3, 8, 4, 6, 8, 4, 7, 2, 1, 6, 9, 3, 5, 5, 3, 6, 9, 4, 8, 2, 1, 7, 3, 7, 2, 1, 8, 5,
        4, 6, 9, 4, 6, 5, 7, 9, 2, 3, 8, 1, 9, 1, 8, 6, 3, 4, 7, 5, 2,
    ];

    // unless you unwrap this, the function doesn't panic.
    board.apply_user_input_to_board(user_input);
    println!("User playing board {}", board);

    let valid = board.validate();

    println!("user solution is {}", valid);
}
