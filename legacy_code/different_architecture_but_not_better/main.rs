mod board;
mod pieces;
use board::Board;
use pieces::{Bishop, King, Knight, Pawn, Pieces, Queen, Rook};

fn main() {
    println!("Hello, world");

    //construct board
    let mut board = Board::new();

    //create a piece
    let pawn = Pawn::new(4, b'A', false, false);

    board.add(pawn);

    let rook = Rook::new(1, b'A', false, false);

    board.add(rook);

    let king = King::new(2, b'A', false, false);

    board.add(king);

    
    for piece in &board.pieces {
        println!("Piece type: {:?}, Piece letter: {:?}, Piece number: {:?}.", piece.piece_type(), piece.letter() as char, piece.number());
    }
    

    //println!("{:?}", board);
}
