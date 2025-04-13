//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use sudoku::core::board::{Board, Difficulty};
use sudoku::core::solver::DancingLinks;
extern crate alloc;
use alloc::vec::Vec;
use fibonacci_lib::{PublicValuesStruct};
use alloy_sol_types::SolType;
pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let n = sp1_zkvm::io::read::<u32>();
    let user_input =  sp1_zkvm::io::read::<Vec<u8>>();

    println!("{:?}", user_input);

    let mut board = Board::from_seed(666, Some(Difficulty::Medium));

    #[cfg(not(feature = "std"))]
    println!("Board generated! {}", board);

    let mut dl = DancingLinks::new();
    dl.init_header_row();
    dl.init_constraint_matrix();
    let sol = dl.solve_with_partial(&board).unwrap();
    let solution_board = DancingLinks::to_sudoku_board(sol);

    // unless you unwrap this, the execution doesn't panic.
    board.apply_user_input_to_board(user_input);
    #[cfg(not(feature = "std"))]
    println!("User playing board {}", board);

    let valid = board.validate();
    #[cfg(not(feature = "std"))]
    println!("user solution is {}", valid);
        let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {valid });
    sp1_zkvm::io::commit_slice(&bytes);
}
