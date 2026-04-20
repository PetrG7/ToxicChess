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
    //pub fn spawn(&self, number: u8, letter: u8, colour: bool, has_moved: bool) ->
}

//implementing functions and methods for pawn
impl Pawn {
    //spawn at location
    pub fn spawn(number: u8, letter: u8, colour: bool, has_moved: bool) -> Pawn {
    //here prevent spawning on illegal squares - out of bounds, or where other pieces are
        Pawn {
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
    pub fn spawn(number: u8, letter: u8, colour: bool) -> Knight {
        Knight {
            number,
            letter,
            colour,
        }
    }
}

//implementing functions and methods for bishop
impl Bishop {
    //spawn at location
    pub fn spawn(number: u8, letter: u8, colour: bool) -> Bishop {
        Bishop {
            number,
            letter,
            colour,
        }
    }
}

//implementing functions and methods for rook
impl Rook {
    //spawn at location
    pub fn spawn(number: u8, letter: u8, colour: bool, has_moved: bool) -> Rook {
        Rook {
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
    pub fn spawn(number: u8, letter: u8, colour: bool) -> Queen {
        Queen {
            number,
            letter,
            colour,
        }
    }
}

//implementing functions and methods for king
impl King {
    //spawn at location
    pub fn spawn(number: u8, letter: u8, colour: bool, has_moved: bool) -> King {
        King {
            number,
            letter,
            colour,
            has_moved,
        }
    }
}
