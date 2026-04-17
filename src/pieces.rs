//for all valid letters
use crate::globals::LETTERS;

//enum for piece colour
#[derive(Debug)]
pub enum Colour {
    Black,
    White,
}
//enum for piece type
#[derive(Debug)]
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
    //x coord - number
    x: u8,
    //y coord - using u8, because in rust that is how you do 
    //8 bit chars, because the type char is 32 bit
    y: u8,
    //colour - black or white
    colour: Colour,
}

//functions for Piece struct
impl Piece {
    pub fn new(piece_type: PieceType, x: u8, y: u8, colour: Colour) -> Option<Piece> {
        //check whether values are valid - haha i forgot this is rust, i don't have
        //to check - hohoho
        //check whether x and y are valid
        if x <= 8 && LETTERS.contains(&y) {
            Some(Piece {
                piece_type,
                x,
                y,
                colour,
            })
        } else {
            None
        }
    }
}
