// board state. assume 9x9 with 3x3 cells.
// needs to have a set seed.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    // row-wise indexing, ie) index i maps to cell (i // 9, i % 9)
    cells: [u8; 81],
}

#[derive(Debug)]
pub enum SudokuError {
    OutOfBounds,
    InvalidValue,
}

impl Board {
    // fn from_seed(seed: u32) -> Self {
    //     //generate random bytes and create a sudoku board based on difficulty
    // }
    fn from_array(data: [u8; 81]) -> Result<Board, SudokuError> {
        // check that the data all lies in [0, 9]. 0 represents an empty cell.
        if data.iter().any(|&x| x > 9) {
            return Err(SudokuError::InvalidValue);
        }

        Ok(Board { cells: data })
    }

    // naive sudoku board solver. todo: experiment with making this faster for the zkVM.
    fn solve(&self) -> bool {
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
    // flat_map effectively takes the lists created inside and appends them into a 'flat' vector.
    // 3 rows of cells (3x3 cells for a 9x9 grid).
    (0..3)
        .flat_map(|row| {
            // 3 columns of cells, find the starting index (top left cell) and return a list of length 3.
            (0..3).map(move |col| start_idx + row * 9 + col)
        })
        .collect() // collect the flattened lists into a vector of size 9.
}

fn main() {
    let mut state = Board { cells: [0; 81] };
    println!("{:?}", state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board_from_data() {
        let valid_cells: [u8; 81] = [
            7, 9, 6, 5, 8, 1, 4, 2, 3, 2, 4, 1, 9, 3, 7, 5, 6, 8, 8, 3, 5, 6, 2, 4, 9, 1, 7, 6, 8,
            7, 3, 5, 2, 1, 4, 9, 4, 1, 9, 8, 7, 6, 3, 5, 2, 3, 5, 2, 4, 1, 9, 7, 8, 6, 1, 7, 8, 2,
            4, 3, 6, 9, 5, 5, 6, 3, 1, 9, 8, 2, 7, 4, 9, 2, 4, 7, 6, 5, 8, 3, 1,
        ];
        let mut board = Board { cells: valid_cells };
        let solved = board.solve();
        // println!("{:?}", board);
    }
}
