// module for the sudoku Board class.

// board state. assume 9x9 with 3x3 cells.
// needs to have a set seed.
use crate::core::error::SudokuError;
use crate::core::random::*;
use core::fmt;
extern crate alloc;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::println;

#[cfg(not(feature = "std"))]
use crate::println;

use super::solver;
use crate::core::solver::DancingLinks;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    // row-wise indexing, ie) index i maps to cell (i // 9, i % 9)
    pub cells: [u8; 81],
}

impl Board {
    //generate random bytes and Create a sudoku board based on difficulty
    pub fn from_seed(seed: u32, difficulty: Option<Difficulty>) -> Self {
        let difficulty = difficulty.unwrap_or(Difficulty::Easy);

        let mut rng = SimpleRng::new(seed);
        let random_array = generate_unique_array(&mut rng);
        let mut cells = [0; 81];
        cells[..9].copy_from_slice(&random_array);

        let temp = Board { cells: cells };

        let mut dl = DancingLinks::new();
        dl.init_header_row();
        dl.init_constraint_matrix();
        let sol = dl.solve_with_partial(&temp).unwrap();
        let mut board = DancingLinks::to_sudoku_board(sol);

        // now remove elements randomly
        // Create indices 0..81 and shuffle them
        let count = match difficulty {
            Difficulty::Easy => 39,
            Difficulty::Medium => 30,
            Difficulty::Hard => 23,
        };

        let random_indices = generate_random_indices(&mut rng, count);
        for &idx in random_indices.iter().take(count) {
            board.cells[idx as usize] = 0; // Assuming 0 represents an empty cell
        }
        board
    }

    pub fn from_array(data: [u8; 81]) -> Result<Board, SudokuError> {
        // check that the data all lies in [0, 9]. 0 represents an empty cell.
        if data.iter().any(|&x| x > 9) {
            return Err(SudokuError::InvalidValue);
        }

        Ok(Board { cells: data })
    }

    pub fn apply_user_input_to_board(&mut self, user_input: Vec<u8>) -> Result<bool, &'static str> {
        for (cell, &input) in self.cells.iter_mut().zip(&user_input) {
            println!("existing node: {}, user inputting: {}", *cell, input);
            if *cell != 0 && input != *cell {
                return Err("user input is replacing a pre-defined hint.");
            }

            *cell = input;
        }

        Ok(true)
    }

    // naive sudoku board validator. todo: experiment with making this faster for the zkVM.
    pub fn validate(&self) -> bool {
        const CORRECT_SORTED_ROW: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        // check rows
        for row_idx in 0..=8 {
            // need to check slices [0 - 8], [9 - 17], ..., [62, 80]
            let start_idx = row_idx * 9;
            let end_idx = row_idx * 9 + 9;
            // println!("{:?}", &self.cells[start_idx..end_idx]);
            let row = &self.cells[start_idx..end_idx];

            // check that elements [1,9] appear exactly once.
            // sort by ascending values
            let mut sorted = row.to_vec();
            sorted.sort_unstable();

            // check if sorted is element-wise equal to [1,2,3,..,9].
            if sorted != CORRECT_SORTED_ROW {
                return false;
            };
        }

        // check columns
        for _ in 0..=8 {
            // columns are indexed as [0, 9, 18, ..., 72], [1, 10, 19, .., 73]
            let col = (0..self.cells.len())
                .step_by(9)
                .map(|i| self.cells[i])
                .collect::<Vec<u8>>();
            // check that elements [1,9] appear exactly once.
            // sort by ascending values
            let mut sorted = col.to_vec();
            sorted.sort_unstable();

            // check if sorted is element-wise equal to [1,2,3,..,9].
            if sorted != CORRECT_SORTED_ROW {
                return false;
            };
        }

        // check all (9) cells
        // todo: move checking logic into a function since it's all the same.
        // indexing for this is [0, 1, 2, 9, 11, 12, 18, 19, 20], [3, 4, 5, 12, 13, 14, 21, 22, 23]
        let box_start_idxs: [usize; 9] = [0, 3, 6, 27, 30, 33, 54, 57, 60];
        let valid = box_start_idxs.iter().all(|start| {
            let cell_indices = get_cell_indices(start);
            let mut cell_values: Vec<u8> = cell_indices
                .iter()
                .map(|element| self.cells[*element])
                .collect();
            cell_values.sort_unstable();

            cell_values == CORRECT_SORTED_ROW
        });

        valid
    }
}
// get the cell indices and return them as a vector for a given starting index.
// in classic 9x9, that would be 0, 3, 6, 27, 30, 33, 54, 57, and 60.
fn get_cell_indices(start_idx: &usize) -> Vec<usize> {
    // flat_map takes the 3 vectors inside and flattens them into one vector.
    // 3 rows of cells (3x3 cells for a 9x9 grid).
    (0..3)
        .flat_map(|row| {
            // 3 columns of cells, find the starting index (top left cell) and return a list of length 3.
            (0..3).map(move |col| start_idx + row * 9 + col)
        })
        .collect()
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut iteration = 0;
        write!(f, "\r\n-------------------------------------\n");
        for s in self.cells {
            write!(f, "| {} ", s);
            iteration += 1;
            if iteration % 9 == 0 {
                write!(f, "| \r\n-------------------------------------\n");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn generate_random_board() {
        let mut board = Board::from_seed(2200, None);
        println!("{}", board);

        let mut dl = DancingLinks::new();
        dl.init_header_row();
        dl.init_constraint_matrix();
        let sol = dl.solve_with_partial(&board).unwrap();

        let solved_board = DancingLinks::to_sudoku_board(sol);
        println!("{}", solved_board);
    }

    #[test]
    fn test_apply_user_input_to_board() {
        let mut board = Board::from_seed(2200, None);
        // This vector represents a valid solution to the puzzle
        let solution = vec![
            9, 2, 7, 1, 3, 6, 8, 4, 5, // Row 1
            1, 3, 4, 2, 5, 8, 6, 7, 9, // Row 2
            5, 6, 8, 4, 7, 9, 1, 3, 2, // Row 3
            2, 7, 1, 8, 4, 3, 5, 9, 6, // Row 4
            6, 5, 3, 9, 2, 1, 4, 8, 7, // Row 5
            4, 8, 9, 7, 6, 5, 2, 1, 3, // Row 6
            7, 1, 2, 5, 9, 4, 3, 6, 8, // Row 7
            8, 9, 6, 3, 1, 2, 7, 5, 4, // Row 8
            3, 4, 5, 6, 8, 7, 9, 2, 1, // Row 9
        ];
        match board.apply_user_input_to_board(solution) {
            Ok(_) => assert!(
                board.validate(),
                "Board should be valid after applying solution"
            ),
            Err(e) => panic!("Failed to apply solution: {}", e),
        }
    }
    #[test]
    #[should_panic(expected = "user tried to overwrite existing board.")]
    fn test_failure_apply_user_input_to_board() {
        let mut board = Board::from_seed(2200, None);
        // Modified first row to conflict with a board hint
        let invalid_solution = vec![
            5, 5, 5, 1, 4, 6, 8, 4, 5, 1, 3, 4, 2, 5, 8, 6, 7, 9, 5, 6, 8, 4, 7, 9, 1, 3, 2, 2, 7,
            1, 8, 4, 3, 5, 9, 6, 6, 5, 3, 9, 2, 1, 4, 8, 7, 4, 8, 9, 7, 6, 5, 2, 1, 3, 7, 1, 2, 5,
            9, 4, 3, 6, 8, 8, 9, 6, 3, 1, 2, 7, 5, 4, 3, 4, 5, 6, 8, 7, 9, 2, 1,
        ];

        board
            .apply_user_input_to_board(invalid_solution)
            .expect("user tried to overwrite existing board.");
    }
    #[test]
    fn test_validate_valid_board() {
        let valid_cells: [u8; 81] = [
            7, 9, 6, 5, 8, 1, 4, 2, 3, 2, 4, 1, 9, 3, 7, 5, 6, 8, 8, 3, 5, 6, 2, 4, 9, 1, 7, 6, 8,
            7, 3, 5, 2, 1, 4, 9, 4, 1, 9, 8, 7, 6, 3, 5, 2, 3, 5, 2, 4, 1, 9, 7, 8, 6, 1, 7, 8, 2,
            4, 3, 6, 9, 5, 5, 6, 3, 1, 9, 8, 2, 7, 4, 9, 2, 4, 7, 6, 5, 8, 3, 1,
        ];
        let mut board = Board { cells: valid_cells };
        let valid = board.validate();
        assert_eq!(valid, true, "Validation was incorrect");
    }
    #[test]
    fn test_validate_empty_board() {
        let invalid_cells: [u8; 81] = [0; 81];
        let mut board = Board {
            cells: invalid_cells,
        };
        let valid = board.validate();
        assert_eq!(valid, false, "Validation was incorrect");
    }

    #[test]
    fn test_board_validate() {
        // same as valid_cells in first test w/ the first element changed.
        let invalid_cells: [u8; 81] = [
            1, 9, 6, 5, 8, 1, 4, 2, 3, 2, 4, 1, 9, 3, 7, 5, 6, 8, 8, 3, 5, 6, 2, 4, 9, 1, 7, 6, 8,
            7, 3, 5, 2, 1, 4, 9, 4, 1, 9, 8, 7, 6, 3, 5, 2, 3, 5, 2, 4, 1, 9, 7, 8, 6, 1, 7, 8, 2,
            4, 3, 6, 9, 5, 5, 6, 3, 1, 9, 8, 2, 7, 4, 9, 2, 4, 7, 6, 5, 8, 3, 1,
        ];
        let mut board = Board {
            cells: invalid_cells,
        };
        let valid = board.validate();
        assert_eq!(valid, false, "Validator incorrect result");
    }
}
