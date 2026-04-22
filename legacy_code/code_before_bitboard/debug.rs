//purely for debbuing needs, since printing the results with Debug is
//unreadable
use crate::game::BoardState;
use crate::pieces::{Colour, Piece, PieceType};

//function that draws the board into terminal using symbols
//not scalable, so terminal has to be big enough
pub fn draw(state: &BoardState) {
    //get the board
    let current = state.get_board();
    //nested loop - could be done much better, but its only for debugging
    // -> no performance needed
    //using print for better control over newlines
    println!(" --- --- --- --- --- --- --- --- ");
    for y in (1..=8).rev() {
        print!("|");
        //the char values - 72+1
        for x in 65..=72 {
            // - 8 + 1
            //check whether there is a piece at these coords
            let is_piece = current
                .iter()
                .find(|piece| piece.get_x() == x && piece.get_y() == y);
            match is_piece {
                Some(piece) => {
                    let unicode = piecetype_to_unicode(piece);
                    print!(" {} |", char::from_u32(unicode).unwrap());
                }
                None => print!("   |"), //empty space
            }
        }
        //number of row
        println!(" {}", y);
        println!(" --- --- --- --- --- --- --- --- ");
    }
    println!("  A   B   C   D   E   F   G   H  ");
}

//is a reference, because i am iterating, thus it is a ref.
fn piecetype_to_unicode(piece: &Piece) -> u32 {
    let res = match piece.get_piece_type() {
        PieceType::Pawn => 0x2659,
        PieceType::Knight => 0x2658,
        PieceType::Bishop => 0x2657,
        PieceType::Rook => 0x2656,
        PieceType::Queen => 0x2655,
        PieceType::King => 0x2654,
    };
    //now check for colour -> if black i have to add +6
    match piece.get_piece_colour() {
        Colour::Black => res,
        Colour::White => res + 6,
    }
}
