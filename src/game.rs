//Here the software will:
//1) Takes the configured board - vector
//2) Each turn has to query for occupied squares
//3) Has to have the ability to output legal moves
use crate::pieces::Piece;
use crate::pieces::PieceType;
use crate::pieces::Colour;

#[derive(Debug)]
pub struct Square {
    //same values as the coordinates for pieces
    x: u8,
    y: u8,
}

//basicly stores the game state - current pieces
#[derive(Debug)]
pub struct BoardState {
    board: Vec<Piece>,
}

impl BoardState {
    pub fn new(board: Vec<Piece>) -> BoardState {
        //in the future check here whether:
        //1) there are two king pieces of different colours
        //2) There are not more than 64 pieces
        //probably something else i am forgetting
        //3) Check if the supplied Vec<Piece> does not assign multiple pieces
        //to one square
        BoardState { board }
    }

    //return all the squares that are occupied -> maybe in the future
    //optimize this so its a 64 bit number - 64 squares, 64 numbers
    //but now a vector will do
    pub fn occupied_squares(&self) -> Vec<Square> {
        let mut occupied: Vec<Square> = Vec::new();
        //iter over all the Piece types in the board
        for piece in &self.board {
            occupied.push(Square {
                x: piece.get_x(),
                y: piece.get_y(),
            });
        }
        occupied
    }

    //calculate all the legal moves in the current position
    fn legal_moves(&self, occupied: Vec<Square>) {
        //here determine the legal moves based on
        //1) The type of the piece - bishop, pawn...
        //2) Occupied squares - what can i capture, where can i can not go
        //3) Colour plays a role in step 2
        todo!();
    }

    //return the current board - pieces, their positions, colours, types
    pub fn get_board(&self) -> &Vec<Piece> {
        &self.board
    }

    //function for default chess configuration
    pub fn populate_default() -> BoardState{
		let mut board: Vec<Piece> = Vec::new();
		//IS THIS STUPID - YES
		//WAS IT FASTER THAN THINKING ABOUT SOME SMART WAY OF
		//DOING THIS FOR THIS ONE AUXILIARY FUNCTION THAT HAS
		//NO IMPACT ON PERFORMANCE? - Y.E.S!

		//push all the major pieces - white
		board.push(Piece::new(PieceType::Rook, b'A', 1, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Rook, b'H', 1, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Knight, b'B', 1, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Knight, b'G', 1, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Bishop, b'C', 1, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Bishop, b'F', 1, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Queen, b'D', 1, Colour::White).unwrap());
		board.push(Piece::new(PieceType::King, b'E', 1, Colour::White).unwrap());

		//push pawns - white
		board.push(Piece::new(PieceType::Pawn, b'A', 2, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'B', 2, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'C', 2, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'D', 2, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'E', 2, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'F', 2, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'G', 2, Colour::White).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'H', 2, Colour::White).unwrap());

		//push all the major pieces - black
		board.push(Piece::new(PieceType::Rook, b'A', 8, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Rook, b'H', 8, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Knight, b'B', 8, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Knight, b'G', 8, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Bishop, b'C', 8, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Bishop, b'F', 8, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Queen, b'D', 8, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::King, b'E', 8, Colour::Black).unwrap());

		//push pawns - black
		board.push(Piece::new(PieceType::Pawn, b'A', 7, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'B', 7, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'C', 7, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'D', 7, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'E', 7, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'F', 7, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'G', 7, Colour::Black).unwrap());
		board.push(Piece::new(PieceType::Pawn, b'H', 7, Colour::Black).unwrap());

		BoardState{ board }	
    	
    }

}
