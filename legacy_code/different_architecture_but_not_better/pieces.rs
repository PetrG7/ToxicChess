//holds the pieces structs and the functions implemented for these pieces
//is that the most effective way? NO, but i want to learn before chasing effectiveness
#[derive(Debug)]
pub struct Pawn {
    number: u8,
    letter: u8,   //holds a classic ascii char
    colour: bool, //lets say 1 for white and 0 for black
    has_moved: bool,
}

#[derive(Debug)]
pub struct Knight {
    number: u8,
    letter: u8,
    colour: bool, //lets say 1 for white and 0 for black
}

#[derive(Debug)]
pub struct Bishop {
    number: u8,
    letter: u8,
    colour: bool, //lets say 1 for white and 0 for black
}

#[derive(Debug)]
pub struct Rook {
    number: u8,
    letter: u8,
    colour: bool, //lets say 1 for white and 0 for black
    has_moved: bool,
}

#[derive(Debug)]
pub struct Queen {
    number: u8,
    letter: u8,
    colour: bool, //lets say 1 for white and 0 for black
}

#[derive(Debug)]
pub struct King {
    number: u8,
    letter: u8,
    colour: bool, //lets say 1 for white and 0 for black
    has_moved: bool,
}

//enum for all pieces, so its possible to use in a vec
#[derive(Debug)]
pub enum Pieces {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

//implementing functions that are for all pieces
impl Pieces {
    //handle getting the numbers - for position
    pub fn number(&self) -> u8 {
        match self {
            Pieces::Pawn(pawn) => pawn.number,
            Pieces::Knight(knight) => knight.number,
            Pieces::Bishop(bishop) => bishop.number,
            Pieces::Rook(rook) => rook.number,
            Pieces::Queen(queen) => queen.number,
            Pieces::King(king) => king.number,
        }
    }
    //handle getting the letters - for position
    pub fn letter(&self) -> u8 {
        match self {
            Pieces::Pawn(pawn) => pawn.letter,
            Pieces::Knight(knight) => knight.letter,
            Pieces::Bishop(bishop) => bishop.letter,
            Pieces::Rook(rook) => rook.letter,
            Pieces::Queen(queen) => queen.letter,
            Pieces::King(king) => king.letter,
        }
    }
    //handle getting the piece type - bishop, pawn, rook...
    pub fn piece_type(&self) -> &'static str{
    	match self {
    	    Pieces::Pawn(_) => "Pawn",
    	    Pieces::Knight(_) => "Knight",
    	    Pieces::Bishop(_) => "Bishop",
    	    Pieces::Rook(_) => "Rook",
    	    Pieces::Queen(_) => "Queen",
    	    Pieces::King(_) => "King",
    	}
    }
    //handle getting the piece colour - white/black
    pub fn piece_colour(&self) -> &'static str{
		if self.colour {
			
		}
    }
}

//implementing converting from the individual pieces to the Pieces struct
// reason -> passing it later to the board struct and i would have to type out
//the piece type two times, which looks strange
impl From<Pawn> for Pieces {
    fn from(pawn: Pawn) -> Self {
        Pieces::Pawn(pawn)
    }
}

impl From<Knight> for Pieces {
    fn from(knight: Knight) -> Self {
        Pieces::Knight(knight)
    }
}

impl From<Bishop> for Pieces {
    fn from(bishop: Bishop) -> Self {
        Pieces::Bishop(bishop)
    }
}

impl From<Rook> for Pieces {
    fn from(rook: Rook) -> Self {
        Pieces::Rook(rook)
    }
}

impl From<Queen> for Pieces {
    fn from(queen: Queen) -> Self {
        Pieces::Queen(queen)
    }
}

impl From<King> for Pieces {
    fn from(king: King) -> Self {
        Pieces::King(king)
    }
}

//implementing functions and methods for pawn
impl Pawn {
    //spawn at location
    pub fn new(number: u8, letter: u8, colour: bool, has_moved: bool) -> Self {
        //here prevent spawning on illegal squares - out of bounds, or where other pieces are
        Self {
            number,
            letter,
            colour,
            has_moved,
        }
    }
}

//implementing functions and methods for knight
impl Knight {
    //spawn at location
    pub fn new(number: u8, letter: u8, colour: bool) -> Self {
        Self {
            number,
            letter,
            colour,
        }
    }
}

//implementing functions and methods for bishop
impl Bishop {
    //spawn at location
    pub fn new(number: u8, letter: u8, colour: bool) -> Self {
        Self {
            number,
            letter,
            colour,
        }
    }
}

//implementing functions and methods for rook
impl Rook {
    //spawn at location
    pub fn new(number: u8, letter: u8, colour: bool, has_moved: bool) -> Self {
        Self {
            number,
            letter,
            colour,
            has_moved,
        }
    }
}

//implementing functions and methods for queen
impl Queen {
    //spawn at location
    pub fn new(number: u8, letter: u8, colour: bool) -> Self {
        Self {
            number,
            letter,
            colour,
        }
    }
}

//implementing functions and methods for king
impl King {
    //spawn at location
    pub fn new(number: u8, letter: u8, colour: bool, has_moved: bool) -> Self {
        Self {
            number,
            letter,
            colour,
            has_moved,
        }
    }
}
