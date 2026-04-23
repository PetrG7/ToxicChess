mod gamestate;
use gamestate::GameState;
use gamestate::{Colour, Piece};

fn main() {
    println!("Hello, BitBoard chess!");

    let mut game = GameState::new();
    println!("{:?}", game);

    game.add(Piece::Rook, Colour::Black, 3, 4).unwrap();

    //println!("{:?}", game);

    game.print();
}
