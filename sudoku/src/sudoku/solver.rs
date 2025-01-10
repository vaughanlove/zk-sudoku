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
// ie, having no conflicting 1 in the columns means that "box has a 1 in position 1" can only exist once in the solution.
use crate::sudoku::board::Board;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{self, write};
use std::rc::Rc;
struct Node {
    column_header: Option<Rc<RefCell<Node>>>,
    up: Option<Rc<RefCell<Node>>>,
    down: Option<Rc<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    name: Option<String>, // for column header
    size: usize,          // for column header
    value: Option<bool>,
    is_header: bool,
}
impl Node {
    fn new(value: Option<bool>, name: Option<String>, size: usize, is_header: bool) -> Self {
        Node {
            column_header: None,
            up: None,
            down: None,
            left: None,
            right: None,
            value,
            name,
            size,
            is_header,
        }
    }
    fn new_header(name: String) -> Rc<RefCell<Self>> {
        let header = Rc::new(RefCell::new(Node {
            column_header: None,
            up: None,
            down: None,
            left: None,
            right: None,
            value: Some(false),
            name: Some(name),
            size: 0,
            is_header: true,
        }));
        let header_clone = header.clone();
        header.borrow_mut().column_header = Some(header_clone.clone());
        header.borrow_mut().up = Some(header_clone.clone());
        header.borrow_mut().down = Some(header_clone);
        header
    }
    fn link_right(
        current: Rc<RefCell<Self>>,
        right: Rc<RefCell<Self>>,
    ) -> Result<(), &'static str> {
        // borrow the Rc of the node C, where A - N - C, is self_rc, new, self_rc.right
        let old_right = current
            .clone()
            .borrow()
            .right
            .clone()
            .ok_or("Node must be initialized in a circular structure.")?;

        // borrow the reference counter to the RefCell of type Node, set left and right to maintain circular references.
        right.borrow_mut().left = Some(current.clone());
        right.borrow_mut().right = Some(old_right.clone());

        current.borrow_mut().right = Some(right.clone());
        old_right.borrow_mut().left = Some(right.clone());
        Ok(())
    }

    fn link_left(current: Rc<RefCell<Node>>, left: Rc<RefCell<Node>>) -> Result<(), &'static str> {
        let old_left = current
            .borrow()
            .left
            .clone()
            .ok_or("Node must be initialized in circular structure")?;

        left.borrow_mut().left = Some(old_left);
        left.borrow_mut().right = Some(current.clone());

        current.borrow_mut().left = Some(left);
        Ok(())
    }

    fn link_up(current: Rc<RefCell<Node>>, up: Rc<RefCell<Node>>) -> Result<(), &'static str> {
        let old_up = current
            .borrow()
            .up
            .clone()
            .expect("Node must be initialized in circular structure");
        {
            let mut new_up = up.borrow_mut();
            new_up.down = Some(current.clone());
            new_up.up = Some(old_up);
        }
        current.borrow_mut().down = Some(up);

        Ok(())
    }

    fn link_down(current: Rc<RefCell<Node>>, down: Rc<RefCell<Node>>) -> Result<(), &'static str> {
        let old = current
            .clone()
            .borrow()
            .down
            .clone()
            .ok_or("failed to unwrap old")?;

        down.borrow_mut().down = Some(old.clone());
        down.borrow_mut().up = Some(current.clone());

        current.borrow_mut().down = Some(down.clone());
        old.borrow_mut().up = Some(down.clone());
        Ok(())
    }
}

struct DancingLinks {
    header: Rc<RefCell<Node>>,
}
impl DancingLinks {
    fn new() -> Self {
        let header = Rc::new(RefCell::new(Node {
            column_header: None,
            up: None,
            down: None,
            left: None,
            right: None,
            name: Some(String::from("h")),
            size: 0,
            value: Some(false),
            is_header: true,
        }));

        {
            let mut h = header.borrow_mut();
            h.right = Some(header.clone());
            h.left = Some(header.clone());
        }

        DancingLinks { header }
    }

    /// This function instantiates the skeleton of the constraint header column and returns the DancingLinks root.
    fn init_header_row(mut self) -> Self {
        let mut prev = self.header.clone();

        // cell position constraints
        // println!("{}", prev.clone().borrow());

        // let first_header = Node::new_header(format!("R{}C{}", 0, 0));

        // Node::link_right(prev.clone(), first_header.clone()).expect("First link failed");

        // prev = first_header;
        // println!("{}", prev.clone().borrow());
        for i in 0..81 {
            // link h to first position
            let header_name = format!("R{}C{}", (i / 9) + 1, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            Node::link_down(new_header.clone(), new_header.clone());
            prev = new_header;
            // if (i == 0) {
            //     Node::link_right(self.header.clone(), new_header);
            // }
        }
        // row constraints - ie, row 1 has a 1, row 1 has a 2, etc
        for i in 0..81 {
            let header_name = format!("R{}#{}", i / 9 + 1, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            Node::link_down(new_header.clone(), new_header.clone());
            prev = new_header;
        }
        // column constraints - ie, col 1 has a 1, col 1 has a 2, etc
        for i in 0..81 {
            let header_name = format!("C{}#{}", i / 9 + 1, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            Node::link_down(new_header.clone(), new_header.clone());
            prev = new_header;
        }
        // box contarints - ie, cell 1 has a 1, etc
        for i in 0..81 {
            let header_name = format!("B{}#{}", i / 9 + 1, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            Node::link_down(new_header.clone(), new_header.clone());
            prev = new_header;
        }
        assert!(
            self.header.borrow().right.is_some(),
            "Header must have right link"
        );
        self
    }

    fn verify_header_row_is_circular(&self) -> Result<(), &'static str> {
        let mut count = 0;
        let mut next = self
            .header
            .clone()
            .borrow()
            .right
            .clone()
            .ok_or("no right link")?;
        // go right
        while (!Rc::ptr_eq(&self.header.clone(), &next.clone())) {
            next = next
                .clone()
                .borrow()
                .right
                .clone()
                .ok_or("right link broken")?;

            if count == 1000 {
                break;
            }
            count += 1;
        }
        count = 0;
        // go left
        while (!Rc::ptr_eq(&self.header.clone(), &next.clone())) {
            next = next
                .clone()
                .borrow()
                .left
                .clone()
                .ok_or("right link broken")?;

            if count == 1000 {
                break;
            }
            count += 1;
        }
        Ok(())
    }

    fn get_col(&self, col_name: &String) -> Result<Rc<RefCell<Node>>, &'static str> {
        let mut count = 0;
        let mut next = self
            .header
            .clone()
            .borrow()
            .right
            .clone()
            .ok_or("no right link")?;
        // go right
        while (!Rc::ptr_eq(&self.header.clone(), &next.clone())) {
            next = next
                .clone()
                .borrow()
                .right
                .clone()
                .ok_or("right link broken")?;
            if String::eq(
                next.clone().borrow().name.as_ref().ok_or("no name")?,
                col_name,
            ) {
                return Ok(next.clone());
            }
            if count == 1000 {
                break;
            }
            count += 1;
        }

        // Header row is not circular
        Err("Header is not circular")
    }
    fn verify_column_is_circular(&self, col_name: &String) -> Result<bool, &'static str> {
        let col_header = self.get_col(col_name).unwrap().clone();
        println!("{}", col_header.clone().borrow());
        let mut count = 0;
        let mut next = col_header
            .clone()
            .borrow()
            .down
            .clone()
            .ok_or("downward link broken")?;

        while !Rc::ptr_eq(&col_header.clone(), &next.clone()) {
            // println!("{}", count);
            next = next
                .clone()
                .borrow()
                .down
                .clone()
                .ok_or("downward link broken")?;
            count += 1;

            if count == 1000 {
                return Ok(false);
            }
        }
        Ok(true)
    }
    // create the empty constraint matrix after initialization
    fn init_constraint_matrix(&mut self) -> Result<(), &'static str> {
        let mut column_header_vec: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(81 * 4 + 1);

        let mut current = self
            .header
            .clone()
            .borrow()
            .right
            .clone()
            .ok_or("h link broken")?;
        // while the node doesn't point to itself (end of list)
        loop {
            // println!("{}", current.clone().borrow());
            column_header_vec.push(current.clone());
            let next = {
                let curr_ref = current.borrow();
                curr_ref.right.clone().ok_or("broken link")?
            };
            if Rc::ptr_eq(&current, &self.header) {
                break;
            }
            current = next;
        }

        for row in 0..9 {
            for col in 0..9 {
                for num in 1..=9 {
                    // calculate the column indicies
                    // ie, cell constraint 1 for (1, 1) is 0
                    let cell_idx = row * 9 + col; //covers the first 81
                    let row_idx = 81 + row * 9 + num - 1;
                    let col_idx = 81 * 2 + col * 9 + num - 1;
                    let box_idx = 81 * 3 + ((row / 3) * 3 + col / 3) * 9 + num - 1;

                    let nodes: Vec<Rc<RefCell<Node>>> = vec![
                        Rc::new(RefCell::new(Node::new(Some(true), None, 0, false))),
                        Rc::new(RefCell::new(Node::new(Some(true), None, 0, false))),
                        Rc::new(RefCell::new(Node::new(Some(true), None, 0, false))),
                        Rc::new(RefCell::new(Node::new(Some(true), None, 0, false))),
                    ];

                    // horizontally link the nodes
                    let first = nodes[0].clone();
                    first.borrow_mut().left = Some(nodes[3].clone());
                    first.borrow_mut().right = Some(nodes[1].clone());

                    for i in 1..3 {
                        // nodes 1 and 2
                        let curr = nodes[i].clone();
                        curr.borrow_mut().left = Some(nodes[i - 1].clone());
                        curr.borrow_mut().right = Some(nodes[i + 1].clone());
                    }

                    // Close the circle with last node
                    let last = nodes[3].clone();
                    last.borrow_mut().left = Some(nodes[2].clone());
                    last.borrow_mut().right = Some(nodes[0].clone());

                    for (&idx, node) in [cell_idx, row_idx, col_idx, box_idx]
                        .iter()
                        .zip(nodes.iter())
                    {
                        let mut col_header = column_header_vec[idx].clone();
                        let temp = col_header.clone();
                        if let Some(ref name) = temp.borrow().name {
                            if name == "h" {
                                col_header =
                                    temp.borrow().right.clone().ok_or("Broken header link")?;
                            }
                        }
                        node.borrow_mut().column_header = Some(col_header.clone());

                        // the node needs to link to the bottom of the column.
                        let header_debug = col_header.clone();
                        // println!(
                        //     "Header '{}': up exists? {}",
                        //     header_debug
                        //         .borrow()
                        //         .name
                        //         .as_ref()
                        //         .unwrap_or(&"unnamed".to_string()),
                        //     header_debug.borrow().up.is_some()
                        // );
                        let prev_last = col_header
                            .borrow()
                            .up
                            .clone()
                            .ok_or("error")
                            .expect("borrow col_header");
                        Node::link_down(prev_last.clone(), node.clone());
                        // Node::link_down(node.clone(), col_header.clone());
                        // println!("{}", node.clone().borrow());
                        // println!("{}", prev_last.clone().borrow());
                        // println!("{}", col_header.clone().borrow());
                        col_header.borrow_mut().size += 1;
                    }
                    // Link nodes horizontally (circular)
                    for i in 0..4 {
                        let curr = &nodes[i];
                        let next = &nodes[(i + 1) % 4];
                        Node::link_right(curr.clone(), next.clone());
                    }
                    // println!("{}", DancingLinks::debug_row_links(nodes[0].clone()));
                }
            }
        }
        Ok(())
    }
    fn debug_row_links(start_node: Rc<RefCell<Node>>) -> String {
        let mut result = String::new();
        let mut visited = HashSet::new();
        let mut count = 0;

        // Maximum iterations to prevent infinite loops during debugging
        const MAX_ITERATIONS: usize = 100;

        let mut current_ptr = Rc::as_ptr(&start_node);

        while !visited.contains(&current_ptr) && count < MAX_ITERATIONS {
            visited.insert(current_ptr);
            count += 1;

            // Scope the borrow to release it before moving to next node
            let col_name = {
                let node = unsafe { &*current_ptr };
                let node_ref = node.borrow();
                node_ref
                    .column_header
                    .as_ref()
                    .map(|h| h.borrow().name.clone())
                    .flatten()
                    .unwrap_or_else(|| "unnamed".to_string())
            };

            result.push_str(&format!("{} -> ", col_name));

            // Get next node pointer
            let next_ptr = {
                let node = unsafe { &*current_ptr };
                let node_ref = node.borrow();
                node_ref.right.as_ref().map(|right| Rc::as_ptr(right))
            };

            match next_ptr {
                Some(ptr) => current_ptr = ptr,
                None => {
                    result.push_str("BROKEN_LINK!");
                    break;
                }
            }
        }

        result.push_str("\n");
        format!("Row links (count: {}): {}", count, result)
    }
    fn cover(&self, column_node: Rc<RefCell<Node>>) -> Result<(), &'static str> {
        // this is the key point of Knuth's DLX.
        let covered_column = column_node.clone();
        {
            let left = covered_column
                .borrow()
                .left
                .clone()
                .ok_or("left link broken")?;
            let right = covered_column
                .borrow()
                .right
                .clone()
                .ok_or("right link broken")?;
            // todo, make link_right better...
            left.borrow_mut().right = Some(right.clone());
            right.borrow_mut().left = Some(left.clone());
        }

        let mut row = covered_column
            .borrow()
            .down
            .clone()
            .ok_or("down link broken")?;
        while !Rc::ptr_eq(&covered_column, &row) {
            let next_row = row.borrow().down.clone().ok_or("down link broken")?;

            let mut col = row.borrow().right.clone().ok_or("right link broken")?;
            while !Rc::ptr_eq(&row, &col) {
                let next_col = col.borrow().right.clone().ok_or("right link broken")?;

                let up = col.borrow().up.clone().ok_or("up link broken")?;
                let down = col.borrow().down.clone().ok_or("down link broken")?;
                up.borrow_mut().down = Some(down.clone());
                down.borrow_mut().up = Some(up.clone());

                if let Some(ref col_header) = col.borrow().column_header {
                    col_header.borrow_mut().size -= 1;
                }

                col = next_col;
            }

            row = next_row;
        }
        Ok(())
    }
    // the opposite of cover
    fn uncover(&self, column_node: Rc<RefCell<Node>>) -> Result<(), &'static str> {
        let mut row = column_node
            .clone()
            .borrow()
            .up
            .clone()
            .ok_or("up link broken")?;
        while !Rc::ptr_eq(&column_node, &row) {
            let mut left_node = row
                .clone()
                .borrow()
                .left
                .clone()
                .ok_or("right link broken")?;
            while !Rc::ptr_eq(&row, &left_node) {
                // reintroduce the links
                // node.up.down -> node
                // node.down.up -> node.up

                let current = left_node.clone();
                let up_node = current
                    .clone()
                    .borrow()
                    .up
                    .clone()
                    .ok_or("up link broken")?;
                let down_node = current
                    .clone()
                    .borrow()
                    .down
                    .clone()
                    .ok_or("down link broken")?;
                up_node.borrow_mut().down = Some(current.clone());
                down_node.borrow_mut().up = Some(current.clone());

                if let Some(ref col_header) = current.borrow().column_header {
                    col_header.borrow_mut().size += 1;
                }
                left_node = left_node
                    .clone()
                    .borrow()
                    .left
                    .clone()
                    .ok_or("right link broken")?;
            }

            row.clone()
                .borrow()
                .right
                .clone()
                .ok_or("up link broken")?
                .borrow_mut()
                .left = Some(
                row.clone()
                    .borrow()
                    .left
                    .clone()
                    .ok_or("down link broken")
                    .unwrap(),
            );

            row.clone()
                .borrow()
                .left
                .clone()
                .ok_or("up link broken")?
                .borrow_mut()
                .right = Some(
                row.clone()
                    .borrow()
                    .right
                    .clone()
                    .ok_or("down link broken")
                    .unwrap(),
            );
            row = row.clone().borrow().up.clone().ok_or("up link broken")?;
        }
        Ok(())
    }

    fn solve(&self) -> Result<(), &'static str> {
        // this will hold the end solution.
        let mut solution: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(81);
        // looking at this, I should abstract the borrowing right/left and checking to helper functions.
        let head = self.header.clone();
        // if we have solved the sparse matrix
        if (Rc::ptr_eq(
            &head.borrow().right.clone().ok_or("right link broken")?,
            &head.clone(),
        )) {
            return Ok(());
        } else {
            let mut column = head.borrow().right.clone().ok_or("right lnk broken")?;
            // cover(column) - todo write cover
            // println!("{}", column.clone().borrow());

            while (!Rc::ptr_eq(
                &column
                    .clone()
                    .borrow()
                    .down
                    .clone()
                    .ok_or("down link failed")?,
                &column
                    .clone()
                    .borrow()
                    .column_header
                    .clone()
                    .ok_or("matrix init failed")?,
            )) {
                // println!("{}", column.clone().borrow());
                column = column
                    .clone()
                    .borrow()
                    .down
                    .clone()
                    .ok_or("down link broken")?;
            }
        }

        Ok(())
    }

    // note to self: right now I have every possible constraint satisfied,
    // if I want to solve a board that has clues already in it, simply remove constraints that conflict
    // with the hints that I have.
    // so if I have a clue that a 5 is at position (6,7), then I keep the constraint that
    // corresponds to that exact constraint, and remove every other constraint at (6,7) and box for 5 etc.
    // DLX can start at any submatrix, so hints are basicually just choosing the
    // first n steps of the puzzle.

    // fn initialize_empty_rows(&self) -> Self {
    //     // the best way to do this is column-wise
    // }
    // // go cell by cell in the 9x9 sudoku board (represented as a 81 element array)
    // for each cell, generate 9 rows to represent [1,9].
    // 729 total rows (9 elements) * (81 positions)
    // fill in constraint columns according to rules.
    // assumes a valid sudoku board going in.
    fn from_sudoku_board(mut self, board: Board) -> Self {
        let mut starting_idx = 0;
        for cell in board.cells {
            println!("{:?}", cell);
        }
        self
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node [")?;
        if let Some(ref name) = self.name {
            write!(f, "{}", name)?;
        } else {
            write!(f, "<unnamed>")?;
        }
        write!(f, "]")?;

        // display neighbor names
        write!(f, "Links [")?;

        let get_node_name = |node: &Option<Rc<RefCell<Node>>>| {
            node.as_ref()
                .map(|n| {
                    n.borrow()
                        .name
                        .as_ref()
                        .map_or("<unnamed>".to_string(), |s| s.clone())
                })
                .unwrap_or_else(|| "<none>".to_string())
        };

        write!(f, "Up: {}, ", get_node_name(&self.up))?;
        write!(f, "Down: {}, ", get_node_name(&self.down))?;
        write!(f, "Left: {}, ", get_node_name(&self.left))?;
        write!(f, "Right: {}", get_node_name(&self.right))?;

        if self.is_header {
            write!(f, ", Size: {}", self.size)?;
        }

        write!(f, "]")
    }
}
impl fmt::Display for DancingLinks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DancingLinks [")?;

        // Print header row
        write!(f, "  Header Row: ")?;
        let mut node = self.header.clone();
        loop {
            // Scope the borrow to ensure it's dropped before node reassignment
            let next_node = {
                let borrowed = node.borrow();
                write!(
                    f,
                    "{} ",
                    borrowed.name.as_ref().unwrap_or(&String::from(""))
                )?;
                // Clone the right reference while borrowed
                borrowed.right.clone()
            }; // borrowed is dropped here

            match next_node {
                Some(right) => {
                    if Rc::ptr_eq(&right, &self.header) {
                        break;
                    }
                    node = right;
                }
                None => break,
            }
        }
        writeln!(f)?;

        // Similar pattern for column sizes
        writeln!(f, "  Column Sizes:")?;
        let mut node = self.header.clone();
        loop {
            let next_node = {
                let borrowed = node.borrow();
                writeln!(
                    f,
                    "    {}: {}",
                    borrowed.name.as_ref().unwrap_or(&String::from("")),
                    borrowed.size
                )?;
                borrowed.right.clone()
            };

            match next_node {
                Some(right) => {
                    if Rc::ptr_eq(&right, &self.header) {
                        break;
                    }
                    node = right;
                }
                None => break,
            }
        }

        write!(f, "]")
    }
}
#[cfg(test)]
mod solver_tests {
    use super::*;

    #[test]
    fn test_constraint_matrix_conversion() {
        let valid_cells: [u8; 81] = [
            7, 0, 6, 5, 8, 0, 0, 0, 0, 2, 4, 1, 0, 0, 0, 0, 0, 8, 8, 3, 5, 6, 2, 4, 9, 1, 7, 6, 8,
            7, 3, 5, 2, 1, 4, 9, 0, 0, 9, 8, 7, 0, 0, 0, 0, 0, 5, 2, 4, 1, 9, 7, 8, 6, 1, 7, 8, 2,
            4, 3, 6, 9, 5, 5, 6, 0, 0, 9, 8, 2, 0, 0, 0, 0, 0, 7, 6, 5, 8, 3, 1,
        ];
        let board = Board { cells: valid_cells };
        let mut dl = DancingLinks::new();
        dl = dl.init_header_row();
        // let cmatrix = DancingLinks::from_sudoku_board(dl, board);
    }
    #[test]
    fn test_basic_circular_link() {
        let node_a = Rc::new(RefCell::new(Node::new(
            Some(true),
            Some(String::from("a")),
            0,
            false,
        )));
        let node_b = Rc::new(RefCell::new(Node::new(
            Some(false),
            Some(String::from("b")),
            0,
            false,
        )));
        let node_c = Rc::new(RefCell::new(Node::new(
            Some(false),
            Some(String::from("c")),
            0,
            false,
        )));

        node_a.borrow_mut().right = Some(node_c.clone());
        node_c.borrow_mut().left = Some(node_a.clone());

        // test the link
        Node::link_right(node_a.clone(), node_b.clone()).expect("Link did not succeed");
    }
    #[test]
    fn test_dl_print() {
        let mut dl = DancingLinks::new();
        // println!("After new(): {}", dl);

        dl = dl.init_header_row();
        // println!("After init_header_row(): {}", dl);

        dl.init_constraint_matrix();

        // println!("After init_constraint_matrix(): {}", dl);
        // dl.solve();

        let node = dl.get_col(&"C7#6".to_string()).unwrap();
        dl.cover(node).unwrap();

        println!("After init_constraint_matrix(): {}", dl);
    }
    #[test]
    fn test_row_circular() {
        let mut dl = DancingLinks::new();
        dl = dl.init_header_row();
        dl.init_constraint_matrix();

        // assert!(dl.verify_header_row_is_circular().is_ok());
        let node = dl.get_col(&"C7#6".to_string());
        // println!("{}", node.unwrap().borrow());
        let is_vertically_circular = dl.verify_column_is_circular(&"R6#3".to_string());
        // println!("{:?}", is_vertically_circular);
        // assert!(is_vertically_circular.unwrap());

        dl.solve();
    }
    #[test]
    fn test_cover_method() {
        let mut dl = DancingLinks::new();
        dl = dl.init_header_row();
        dl.init_constraint_matrix();
        let node = dl.get_col(&"C7#6".to_string()).unwrap();
        dl.cover(node);
    }
}
