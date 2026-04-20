//structs to help storing all the pieces in a "board"
//also implementation of methods to gather game state - legal moves,
//occupied fields, etc...
use crate::pieces::Pieces;

#[derive(Debug)]
pub struct Board {
    pub pieces: Vec<Pieces>,
}

//functions and methods for the board struct
impl Board {
    //construct empty board - basically init
    pub fn new() -> Self {
        Self { pieces: Vec::new() }
    }
    //add piece to board
    pub fn add<T: Into<Pieces>>(&mut self, piece: T) {
        self.pieces.push(piece.into());
    }
}
