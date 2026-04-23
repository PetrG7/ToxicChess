use std::fmt; // for implementing display

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

impl fmt::Display for Colour {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Colour::Black => write!(f, "Black"),
			Colour::White => write!(f, "White"),
		}
	}
}


//implementing structures to add pieces
impl GameState{
	//here it is probably best to use 1-8 for coords instead of 0-7, because its easiers
	pub fn add(piece_type: &'static str, colour: Colour, x: u8, y: u8){
		//check whether x and y are possible values - colour is safe, because it is an enum, piece_type will be handled by matching
		if x > 7 || y > 7 {
			//don't know whether it is better to handle the error here, or later with this function returning option.
			println!("Invalid piece coordinates for: {} at coords x: {}, y: {}, colour is {}. Piece was not added.", piece_type, x, y, colour);
		}
		//convert input to lowercase
		match piece_type.to_lowercase().as_str() {
			"pawn" => {},
			"knight" => {},
			"bishop" => {},
			"rook" => {},
			"queen" => {},
			"king" => {},
			_ => {},
		}
		
		todo!();
	}

	//init function
	pub fn new() -> Self {
		GameState::default()
	}
	
	//function to add pawn
	fn add_pawn(&mut self, x: u8, y: u8, colour: Colour) {
		match colour {
			Colour::White => {
				//check whether there is not a piece at that certain index
				if Self::is_bit_set(self.white_pieces, Self::get_index(x, y)) {
					//there is already a piece there
					println!("These is already a pawn at x: {}, y: {}, the pawn has not been added.", x, y);
				}
				else{
					self.white_pawns |= 1u64 << Self::get_index(x, y);
				}
			}
			Colour::Black => {

				
			}
		}
	}

	//calculate index of u64 bit from coords
	fn get_index(x: u8, y: u8) -> u8 {
		(x+1)*(y+1)
	}

	//helper function to calculate whether a bit at a certain index is set - it is 1
	fn is_bit_set(number: u64, index: u8) -> bool {
		(number & (1u64 << index)) != 0
	}
	
}
