//imports for piece handling
mod pieces;
use pieces::Colour;
use pieces::Piece;
use pieces::PieceType;

fn main() {
    println!("Hello, chess!");
    //here lies my logic for chessboard initialization
    //esentially it will be a vector of pieces, also there will be another vector with occupied fields
    //so that i can easily calculate legal moves
    let mut board: Vec<Piece> = Vec::new();
    //"push" my "pieces" onto the "chessboard"
    //here i put a white bishop on the coords A3 (3 = 3, 1 = A)
	board.push(Piece::new(PieceType::Bishop, 3, 1, Colour::White).unwrap());
	//white pawns
	for letter in 1..8{
		board.push(Piece::new(PieceType::Pawn, 3, letter, Colour::White).unwrap());
	}
	//debug print board
	println!("{:?}", board);
	
    
}
