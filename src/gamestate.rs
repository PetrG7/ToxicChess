use std::fmt; // for implementing display

//calculate index of u64 bit from coords
fn get_index(x: u8, y: u8) -> u8 {
    //(x + 1) * (y + 1) <- incorrect
    y * 8 + x //<- correct
}

//helper function to calculate whether a bit at a certain index is set - it is 1
fn is_bit_set(number: u64, index: u8) -> bool {
    (number & (1u64 << index)) != 0
}

//structure to hold all the necesarry bitboards
#[derive(Default, Debug)]
pub struct GameState {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
    //and for all the pieces
    white_pieces: u64,
    black_pieces: u64,
}

//helping enum for adding pieces
pub enum Colour {
    Black,
    White,
}

//helping enum for adding pieces
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

//impl display for debbuging purpouses
impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Black => write!(f, "Black"),
            Self::White => write!(f, "White"),
        }
    }
}

//impl display for debbugging purpouses
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Pawn => write!(f, "pawn"),
            Self::Knight => write!(f, "knight"),
            Self::Bishop => write!(f, "bishop"),
            Self::Rook => write!(f, "rook"),
            Self::Queen => write!(f, "queen"),
            Self::King => write!(f, "king"),
        }
    }
}

//implementing structures to add pieces
impl GameState {
    //function to add pieces to the board
    pub fn add(&mut self, piece: Piece, colour: Colour, x: u8, y: u8) -> Result<(), &'static str> {
        //check whether x and y are possible values - colour is safe, because it is an enum, piece_type will be handled by matching
        if x > 7 || y > 7 {
            //don't know whether it is better to handle the error here, or later with this function returning option.
            return Err("You tried placing a piece out of bounds. That is illegal!");
        }

        //gather all pieces bitboard (all occupied squares)
        let full_bitboard = self.white_pieces | self.black_pieces;
        //calculate the index
        let index = get_index(x, y);

        //check whether the square already has a piece on it
        if is_bit_set(full_bitboard, index) {
            return Err(
                "There is already a piece assigned to square with your provided coordinates. That is illegal!",
            );
        } else {
            //make the piece mask
            let mask = 1u64 << index;
            //have to use reference for colour here, because i can't consume it
            //and I need to use it later
            match (piece, &colour) {
                (Piece::Pawn, Colour::White) => self.white_pawns |= mask,
                (Piece::Knight, Colour::White) => self.white_knights |= mask,
                (Piece::Bishop, Colour::White) => self.white_bishops |= mask,
                (Piece::Rook, Colour::White) => self.white_rooks |= mask,
                (Piece::Queen, Colour::White) => self.white_queens |= mask,
                (Piece::King, Colour::White) => self.white_king |= mask,
                (Piece::Pawn, Colour::Black) => self.black_pawns |= mask,
                (Piece::Knight, Colour::Black) => self.black_knights |= mask,
                (Piece::Bishop, Colour::Black) => self.black_bishops |= mask,
                (Piece::Rook, Colour::Black) => self.black_rooks |= mask,
                (Piece::Queen, Colour::Black) => self.black_queens |= mask,
                (Piece::King, Colour::Black) => self.black_king |= mask,
            }
            //also include the piece in the full colour bitboard
            match colour {
                Colour::White => self.white_pieces |= mask,
                Colour::Black => self.black_pieces |= mask,
            }
        }
        Ok(())
    }

    //init function
    pub fn new() -> Self {
        GameState::default()
    }

    //init function with default pieces - how it would look, if it were a start of the game
    pub fn new_default() -> Self {
        //it would be easier to express these numbers using bitwise arithmetic
        //hovewer i want to do it like this, so its visible
        Self {
            //white pieces
            white_pawns: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
            white_knights:
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            white_bishops:
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            white_rooks: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            white_queens: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            white_king: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,
            //black pieces
            black_pawns: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            black_knights:
                0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_bishops:
                0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_rooks: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_queens: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            black_king: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            //full map
            white_pieces: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_11111111,
            black_pieces: 0b11111111_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
        }
    }

	//function to move piece -> change the board
    pub fn move_piece(&mut self, current_x: u8, current_y: u8, target_x: u8, target_y: u8){
    	todo!()
    }

    //print current gamestate - for debug purpouses
    pub fn display(&self) {
        //construct full bitmap
        let full_bitboard = self.white_pieces | self.black_pieces;

        println!(" --- --- --- --- --- --- --- --- ");

        // Iterate through ranks (rows) from top (8) to bottom (1)
        for rank in (0..8).rev() {
            print!("|");

            // Iterate through files (columns) from left (A) to right (H)
            for file in 0..8 {
                // Calculate the 1D bitboard index from our 2D coordinates
                let i = rank * 8 + file;

                //if there is a piece at that location
                if is_bit_set(full_bitboard, i) {
                    //check for white colour
                    if is_bit_set(self.white_pieces, i) {
                        if is_bit_set(self.white_pawns, i) {
                            print!(" ♟ |");
                        } else if is_bit_set(self.white_knights, i) {
                            print!(" ♞ |");
                        } else if is_bit_set(self.white_bishops, i) {
                            print!(" ♝ |");
                        } else if is_bit_set(self.white_rooks, i) {
                            print!(" ♜ |");
                        } else if is_bit_set(self.white_queens, i) {
                            print!(" ♛ |");
                        } else {
                            print!(" ♚ |");
                        }
                    }
                    //if the piece is black
                    else {
                        if is_bit_set(self.black_pawns, i) {
                            print!(" ♙ |");
                        } else if is_bit_set(self.black_knights, i) {
                            print!(" ♘ |");
                        } else if is_bit_set(self.black_bishops, i) {
                            print!(" ♗ |");
                        } else if is_bit_set(self.black_rooks, i) {
                            print!(" ♖ |");
                        } else if is_bit_set(self.black_queens, i) {
                            print!(" ♕ |");
                        } else {
                            print!(" ♔ |");
                        }
                    }
                }
                //if there is not a bit at that location
                else {
                    print!("   |");
                }
            }

            // Printing the row ends. We print the rank number, then drop to the next line for the divider.
            println!(" {}", rank + 1);
            println!(" --- --- --- --- --- --- --- --- ");
        }

        // Print the file letters at the very bottom
        println!("  A   B   C   D   E   F   G   H  ");
    }
}
