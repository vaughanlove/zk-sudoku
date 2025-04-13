# zk-sudoku

generate a zk proof of a completed sudoku board. Currently has examples using openvm's and succinct's zkVMs.

Core solving logic in `sudoku` using an implementation of Knuth's Dancing Links algorithm.

`X_proof` is X's zkVM being used to generate proofs. I needed about 30GB of ram to generate proofs using openvm's zkVM and 14GB for succinct's zkVM. Don't recommend using swap memory, it significantly slows down computation.

Working on optimizing the algorithm and profiling the different zkVMs to optimize for cycles. Also need to add setup instructions.

