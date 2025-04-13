#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]
// src/main.rs
use openvm::io::{read_vec, reveal};
use sudoku::core::board::{Board, Difficulty};
use sudoku::core::solver::DancingLinks;
extern crate alloc;
use alloc::vec::Vec;

openvm::entry!(main);

fn main() {
    // read_vec is a helper function that reads passed input from the hint stream.
    let user_input: Vec<u8> = read_vec();

    // Generate a board from a seed.
    let mut board = Board::from_seed(666, Some(Difficulty::Medium));

    let mut dl = DancingLinks::new();
    dl.init_header_row();
    dl.init_constraint_matrix();
    let sol = dl.solve_with_partial(&board).unwrap();
    let solution_board = DancingLinks::to_sudoku_board(sol);

    // unless you unwrap this, the execution doesn't panic.
    board.apply_user_input_to_board(user_input);
    
    // #[cfg(not(feature = "std"))]
    // println!("User playing board {}", board);

    // let valid = board.validate();
    let valid = false;
    
    // #[cfg(not(feature = "std"))]
    // println!("user solution is {}", valid);

    reveal(1 as u32, 0);
}
