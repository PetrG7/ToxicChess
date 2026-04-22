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
    let mut board: Vec<Piece> = vec![];

    //"push" my "pieces" onto the "chessboard"
    //here i put a white bishop on the coords A3 (3 = 3, 1 = A)
    board.push(Piece::new(PieceType::Bishop, b'H', 5, Colour::White, false).unwrap());

    board.push(Piece::new(PieceType::King, b'D', 1, Colour::White, false).unwrap());

    board.push(Piece::new(PieceType::King, b'D', 5, Colour::Black, false).unwrap());

    board.push(Piece::new(PieceType::Rook, b'H', 3, Colour::Black, false).unwrap());

    board.push(Piece::new(PieceType::Knight, b'C', 5, Colour::Black, false).unwrap());

    board.push(Piece::new(PieceType::Rook, b'E', 4, Colour::Black, false).unwrap());

	board.push(Piece::new(PieceType::Rook, b'E', 6, Colour::Black, false).unwrap());
/*
    for &letter in &LETTERS {
        //println!("{}", letter);
        board.push(Piece::new(PieceType::Pawn, letter, 2, Colour::White, false).unwrap());
    }

    for &letter in &LETTERS {
        //println!("{}", letter);
        board.push(Piece::new(PieceType::Pawn, letter, 7, Colour::Black, false).unwrap());
    }
*/

    //testing the legal moves func
    match BoardState::new(board, Colour::White) {
        Ok(state) => {
            let gamestate = state;
            draw(&gamestate);
            let legal = gamestate.legal_moves();
            println!("{:?}", legal);
        }
        Err(e) => println!("{}", e),
    }

    let gamestate_default = BoardState::populate_default();
    draw(&gamestate_default);
    //make a move

    //testing occupied squares function
    let occupied = gamestate_default.occupied_squares(&Colour::White);
    // println!("{:?}", occupied);
}
