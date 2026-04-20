//for all valid letters
use crate::globals::LETTERS;

//enum for piece colour
#[derive(Debug, PartialEq)]
pub enum Colour {
    Black,
    White,
}
//enum for piece type
#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

//stucture that will hold information about a individual piece
#[derive(Debug)]
pub struct Piece {
    //type - bishop, pawn...
    piece_type: PieceType,
    //x coord- using u8, because in rust that is how you do
    //8 bit chars, because the type char is 32 bit
    x: u8,
    //y coord - number on the side
    y: u8,
    //colour - black or white
    colour: Colour,
    //will have to put this here, it will not apply for all pieces, but its seems like the easiest after trying out a different appproach
    has_moved: bool,
}

//functions for Piece struct
impl Piece {
    pub fn new(
        piece_type: PieceType,
        x: u8,
        y: u8,
        colour: Colour,
        has_moved: bool,
    ) -> Option<Self> {
        //check whether values are valid - haha i forgot this is rust, i don't have
        //to check - hohoho
        //check whether x and y are valid
        if y <= 8 && LETTERS.contains(&x) {
            Some(Self {
                piece_type,
                x,
                y,
                colour,
                has_moved,
            })
        } else {
            None
        }
    }
    //implement a function for fetching the x and y coords, since I want to keep
    //my struct fields private
    pub fn get_x(&self) -> u8 {
        //it copies the value before returning so its fine
        self.x
    }
    pub fn get_y(&self) -> u8 {
        //it copies the value before returning so its fine i guess
        self.y
    }

    //for getting piece type - debbuging drawing
    pub fn get_piece_type(&self) -> &PieceType {
        &self.piece_type
    }
    //for getting piece colour
    pub fn get_piece_colour(&self) -> &Colour {
        &self.colour
    }
}
