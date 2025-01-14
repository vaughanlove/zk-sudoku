use sudoku::core::solver::DancingLinks;
fn main() {
    let mut dl = DancingLinks::new();
    dl.init_header_row();
    dl.init_constraint_matrix();
    let res = dl.solve().unwrap();
    // let board = Board::from_seed(1, None);
    // I want this main to be minimal. Think, just the inputs to the zk proof. So a (public) seed, a (public) difficulity, and (private) user inputs.
    // the API should look as follows:
    // 1. Generate the board from the seed and difficulty.
    // 2. Attempt to place the user input into the generated board.
    // 3. Validate the board and return T/F (public).
    let board = DancingLinks::to_sudoku_board(res);
    println!("{}", board.clone());
    let valid = board.validate();
    println!("Board is valid: {}", valid);
}
