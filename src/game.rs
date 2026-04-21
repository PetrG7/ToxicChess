//Here the software will:
//1) Takes the configured board - vector
//2) Each turn has to query for occupied squares
//3) Has to have the ability to output legal moves
use crate::pieces::Colour;
use crate::pieces::Piece;
use crate::pieces::PieceType;

#[derive(Debug, PartialEq)]
pub struct Square {
    //same values as the coordinates for pieces
    x: u8,
    y: u8,
}

//basicly stores the game state - current pieces
#[derive(Debug)]
pub struct BoardState {
    board: Vec<Piece>,
    //spencifies whose turn it is -> for calculating legal moves
    turn: Colour,
}

impl BoardState {
	//starting_colour meaning which colour starts
    pub fn new(board: Vec<Piece>, starting_colour: Colour) -> Result<Self, &'static str> {
        //in the future check here whether:
        //1) there are two king pieces of different colours - DONE
        //2) There are not more than 64 pieces - DONE
        //probably something else i am forgetting
        //3) Check if the supplied Vec<Piece> does not assign multiple pieces
        //to one square - DONE

        //validating the provided pieces
        if board.len() > 64 {
            return Err("A board vector with more than 64 pieces was provided.");
        }

        let mut black_king = false;
        let mut white_king = false;

		//have to do it like this, so the pieces don't get compared agains the same piece
        for (index, piece) in board.iter().enumerate() {
            //check the piece againts all other pieces
            for (other_index, other_piece) in board.iter().enumerate() {
                if (piece.get_y() == other_piece.get_y()) && (piece.get_x() == other_piece.get_x()) && (index != other_index)
                {
                    return Err("Two pieces were assigned the same square");
                }
            }
            //check for two kings of the opposite colour
            if piece.get_piece_type() == &PieceType::King {
                match piece.get_piece_colour() {
                    Colour::Black => {
                        //if there is no black king
                        if !black_king {
                            //there is a black king now
                            black_king = true;
                        }
                        // if there is a black king -> error
                        else {
                            return Err("Two black kings were provided.");
                        }
                    }
                    Colour::White => {
                    	//same check as with the black king
                        if !white_king {
                            white_king = true;
                        } else {
                            return Err("Two white kings were provided.");
                        }
                    }
                }
            }
        }

        //check whether there are kings
        if !black_king || !white_king {
        	return Err("You have to provide one king piece of each colour.");
        }

        //Self { board }
        return Ok(Self { board, turn: starting_colour});
    }

    //return all the squares that are occupied -> maybe in the future
    //optimize this so its a 64 bit number - 64 squares, 64 numbers
    //but now a vector will do
    //also decided to use the colour variable, which is admittedly a bit inneficient
    //but also makes my job much easier :)
    pub fn occupied_squares(&self, colour: &Colour) -> Vec<Square> {
        let mut occupied: Vec<Square> = Vec::new();
        //iter over all the Piece types in the board
        for piece in &self.board {
        	if piece.get_piece_colour() == colour{
        		occupied.push(Square {
        		    x: piece.get_x(),
        		    y: piece.get_y(),
        		});
        	}
        }
        occupied
    }

    //calculate all the legal moves in the current position, for the current turn
    //NOTE - FIX THIS, IT IS NOT SUPPOSED TO RETURN THE PIECE TYPE BUT THE PIECE, THE PIECE TYPE TELLS ME NOTHING
    //SUMMARY OF THINGS TO FIX HERE - RETURN PIECE, NOT PIECE TYPE AND ADD THE MODIFIER BASED ON THE COLOURS - BLACK PAWNS GO -1 on Y axis, WHITE GO +1 on Y axis
    pub fn legal_moves(&self) -> Vec<(&PieceType, Square)> {
        // -> Vec<(Piece, Square)>{
        //here determine the legal moves based on
        //1) The type of the piece - bishop, pawn...
        //2) Occupied squares - what can i capture, where can i can not go
        //3) Colour plays a role in step 2

		//initialization of the variable, so the compiler does not scream at me - fix later hihi
        let mut occupied = self.occupied_squares(&Colour::White);

        //get all the occupied squares for the colour that is not in turn
        match self.turn {
        	Colour::White => {occupied = self.occupied_squares(&Colour::Black);},
        	Colour::Black => {occupied = self.occupied_squares(&Colour::White);},
        }

        let mut legal_moves: Vec<(&PieceType, Square)> = vec![];

        //iterate over the pieces
        for piece in &self.board {
            //different rules, for different pieces
            let piece_type = piece.get_piece_type();

            //println!("{:?}", piece_type);


           	//NOTE -> HAVE TO ADD COLOUR MODIFIERS -> BECAUSE WHITE PAWNS CAPTURE UP, WHEREAS BLACK PAWNS CAPTURE DOWN FOR EXAMPLE

            match piece_type {
                //now that we know the piece, each has to abide by their own rules to determine the possible move squares
                PieceType::Pawn => {
                	//pawns can go up two, if they have not been moved
                	if !piece.has_moved() {
                		//if the pawn has not moved yet
                		//it can go up 1
                		legal_moves.push((piece_type, Square { x: piece.get_x(), y: piece.get_y() + 1}));
						//it can go up 2
						legal_moves.push((piece_type, Square { x: piece.get_x(), y: piece.get_y() + 2}));
						//scan occupied squares by the opposite colour, so it can see whether it can capture
						if occupied.contains(&Square {x: piece.get_x() - 1, y: piece.get_y() + 1}){
							//pawn can capture to the left
							legal_moves.push((piece_type, Square { x: piece.get_x() - 1, y: piece.get_y() + 1}));
						}
						if occupied.contains(&Square {x: piece.get_x() + 1, y: piece.get_y() + 1}){
							//pawn can capture to the left
							legal_moves.push((piece_type, Square { x: piece.get_x() + 1, y: piece.get_y() + 1}));
						}						
						//also this is where i will implement en pessent, but that is very difficcult
						//so for now i'll skip it
                	}
                }
                PieceType::Knight => {}
                PieceType::Bishop => {}
                PieceType::Rook => {}
                PieceType::Queen => {}
                PieceType::King => {}
            }
        }

        legal_moves
    }

    //return the current board - pieces, their positions, colours, types
    pub fn get_board(&self) -> &Vec<Piece> {
        &self.board
    }

    //function for default chess configuration
    pub fn populate_default() -> BoardState {
        let mut board: Vec<Piece> = Vec::new();
        //IS THIS STUPID - YES
        //WAS IT FASTER THAN THINKING ABOUT SOME SMART WAY OF
        //DOING THIS FOR THIS ONE AUXILIARY FUNCTION THAT HAS
        //NO IMPACT ON PERFORMANCE? - Y.E.S!

        //push all the major pieces - white
        board.push(Piece::new(PieceType::Rook, b'A', 1, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Rook, b'H', 1, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Knight, b'B', 1, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Knight, b'G', 1, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Bishop, b'C', 1, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Bishop, b'F', 1, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Queen, b'D', 1, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::King, b'E', 1, Colour::White, false).unwrap());

        //push pawns - white
        board.push(Piece::new(PieceType::Pawn, b'A', 2, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'B', 2, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'C', 2, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'D', 2, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'E', 2, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'F', 2, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'G', 2, Colour::White, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'H', 2, Colour::White, false).unwrap());

        //push all the major pieces - black
        board.push(Piece::new(PieceType::Rook, b'A', 8, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Rook, b'H', 8, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Knight, b'B', 8, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Knight, b'G', 8, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Bishop, b'C', 8, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Bishop, b'F', 8, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Queen, b'D', 8, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::King, b'E', 8, Colour::Black, false).unwrap());

        //push pawns - black
        board.push(Piece::new(PieceType::Pawn, b'A', 7, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'B', 7, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'C', 7, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'D', 7, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'E', 7, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'F', 7, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'G', 7, Colour::Black, false).unwrap());
        board.push(Piece::new(PieceType::Pawn, b'H', 7, Colour::Black, false).unwrap());

		//by default, white starts ofc
        BoardState { board, turn: Colour::White}
    }
}
