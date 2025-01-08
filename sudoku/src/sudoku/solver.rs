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
use std::fmt;
use std::ops::Deref;
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
            .borrow()
            .right
            .clone()
            .ok_or("Node must be initialized in a circular structure.")?;

        // borrow the reference counter to the RefCell of type Node, set left and right to maintain circular references.
        right.borrow_mut().left = Some(current.clone());
        right.borrow_mut().right = Some(old_right);

        current.borrow_mut().left = Some(right);

        Ok(())
    }

    fn link_left(current: Rc<RefCell<Node>>, left: Rc<RefCell<Node>>) -> Result<(), &'static str> {
        let old_left = current
            .borrow()
            .left
            .clone()
            .expect("Node must be initialized in circular structure");
        {
            let mut new_node = left.borrow_mut();
            new_node.left = Some(old_left);
            new_node.right = Some(current.clone());
        }
        current.borrow_mut().right = Some(left);
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
        let old_down = current
            .borrow()
            .down
            .clone()
            .expect("Node must be initialized in circular structure.");
        {
            let mut new_down = down.borrow_mut();
            new_down.down = Some(old_down);
            new_down.up = Some(current.clone());
        }
        current.borrow_mut().up = Some(down);
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
        // Debug first link
        let first_header = Node::new_header(format!("R{}C{}", 0, 1));
        Node::link_right(prev.clone(), first_header.clone()).expect("First link failed");
        prev = first_header;
        for i in 1..81 {
            // link h to first position
            let header_name = format!("R{}C{}", i / 9, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            prev = new_header;
            // if (i == 0) {
            //     Node::link_right(self.header.clone(), new_header);
            // }
        }
        // row constraints - ie, row 1 has a 1, row 1 has a 2, etc
        for i in 0..81 {
            let header_name = format!("R{}#{}", i / 9, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            prev = new_header;
        }
        // column constraints - ie, col 1 has a 1, col 1 has a 2, etc
        for i in 0..8 {
            let header_name = format!("C{}#{}", i / 9, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            prev = new_header;
        }
        // box contarints - ie, cell 1 has a 1, etc
        for i in 0..81 {
            let header_name = format!("B{}#{}", i / 9, (i % 9) + 1);
            let new_header = Node::new_header(header_name);
            Node::link_right(prev.clone(), new_header.clone()).expect("Linking failed");
            prev = new_header;
        }
        assert!(
            self.header.borrow().right.is_some(),
            "Header must have right link"
        );
        self
    }

    fn from_sudoku_board(mut self, board: Board) -> Self {
        self
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
    fn test_constraint_matrix_convertion() {
        let valid_cells: [u8; 81] = [
            7, 0, 6, 5, 8, 0, 0, 0, 0, 2, 4, 1, 0, 0, 0, 0, 0, 8, 8, 3, 5, 6, 2, 4, 9, 1, 7, 6, 8,
            7, 3, 5, 2, 1, 4, 9, 0, 0, 9, 8, 7, 0, 0, 0, 0, 0, 5, 2, 4, 1, 9, 7, 8, 6, 1, 7, 8, 2,
            4, 3, 6, 9, 5, 5, 6, 0, 0, 9, 8, 2, 0, 0, 0, 0, 0, 7, 6, 5, 8, 3, 1,
        ];
        let board = Board { cells: valid_cells };
        // let cmatrix = ConstraintMatrix::from_board(&board);
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
        // why cant I just node_a.borrow_mut().right = ..  ?

        node_a.borrow_mut().right = Some(node_c.clone());
        node_c.borrow_mut().left = Some(node_a.clone());

        // test the link
        Node::link_right(node_a.clone(), node_b.clone()).expect("Link did not succeed");
    }
    #[test]
    fn test_dl_print() {
        let mut dl = DancingLinks::new();
        println!("After new(): {}", dl);

        dl = dl.init_header_row();
        println!("After init_header_row(): {}", dl);
    }
}
