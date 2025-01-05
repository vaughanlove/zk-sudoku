// my implementation of Algorithm X using Knuth's DLX
// step 1 is to convert sudoku into a exact cover problem
// you can do this by letting the rows represent number entries - ie R1C1#1 is row index 0, is the first cell of the board at (1,1) with value 1.
// with this logic, we need 9 (possible values) * 9 (row positions) * (9) (col positions) = 729 rows
// now we need to introduce the constraints.
// we know that each row can have exactly 1 of each element in [1, 9].
// we know that each column can have exactly 1 of each element in [1,9].
// we know that every cell needs to be full
// we know that every box 9*(3*3) has to be have exactly 1 of each element in [1,9]
// to convert these into columns:
// Row constraints:
// "row has a 1 in position 1", "row has a 2 in position 1", .. "row has a 9 in position 1", "row has a 1 in position 2", ... "row has a 9 in position 9".
// we can see this is 9x9=81 constraints
// similar for columns, 81 constraints
// again for boxes, same logic, "box has a 1 in position 1", "box has a 1 in position 2", etc.
// 81 constraints
// and for cells its simply about being occupied, "position 1 occupied", "position 2 occupied", "position 81 occupied".
// Why does solving this exact cover problem yield a valid sudoku board?
// because the exact cover problem finds every row such that there are no conflicting 1s in the columns.
// ie, having no conflicting 1s in the columns means that "box has a 1 in position 1" can only exist once in the solution.

struct Cell {
    column_header: Box<Cell>,
}
// This probably needs a custom struct Cell to represent for DLX
struct ConstraintMatrix {
    matrix: Vec<Vec<u8>>,
}

impl ConstraintMatrix {
    // fn from_board(board: &Board){
    //     // parse through the cells in board.cells and add entries to the constraint matrix according to the rules above.

    // };
    // fn solve();
    // fn to_board();
}
