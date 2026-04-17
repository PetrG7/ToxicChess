//imports for piece handling
mod pieces;
use pieces::Colour;
use pieces::Piece;
use pieces::PieceType;
mod globals;
use globals::LETTERS;
mod game;
use game::BoardState;
mod debug; // for drawing the board - debugging purpouses
use debug::draw;

fn main() {
    println!("Hello, chess!");

    //here lies my logic for chessboard initialization
    //esentially it will be a vector of pieces, also there will be another vector with occupied fields
    //so that i can easily calculate legal moves
    let mut board: Vec<Piece> = Vec::new();

    //"push" my "pieces" onto the "chessboard"
    //here i put a white bishop on the coords A3 (3 = 3, 1 = A)
    board.push(Piece::new(PieceType::Bishop, b'H', 5, Colour::White).unwrap());
    //white pawns
    //the reason for the goofy double borrow is some rust stuff
    //if there was no & before letter i would have to deref in the board.push call
    for &letter in &LETTERS {
        //println!("{}", letter);
        board.push(Piece::new(PieceType::Pawn, letter, 3, Colour::White).unwrap());
    }
    //init GameState
    let gamestate = BoardState::new(board);
    draw(gamestate);

    let gamestate = BoardState::populate_default();
    draw(gamestate);
}
