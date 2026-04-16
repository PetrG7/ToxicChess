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
    //y coord - on a chessboard it is a letter but here i switched it for number
    y: u8,
    //colour - black or white
    colour: Colour,
}

//functions for Piece struct
impl Piece {
    pub fn new(piece_type: PieceType, x: u8, y: u8, colour: Colour) -> Option<Piece> {
        //check whether values are valid - haha i forgot this is rust, i don't have
        //to check - hohoho
        //actually i have to check whether x and y are not more than 8
        if x <= 8 && y <= 8 {
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
