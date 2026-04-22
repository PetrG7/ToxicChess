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


//implementing structures to add pieces
impl GameState{
	//here it is probably best to use 1-8 for coords instead of 0-7, because its easiers
	pub fn add(piece_type: &'static str, colour: &'static str, x: u8, y: u8){
		todo!();
	}

	//init function
	pub fn new() -> Self {
		GameState::default()
	}

	
}
