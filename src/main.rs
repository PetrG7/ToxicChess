mod gamestate;
use gamestate::GameState;
use gamestate::{Colour, Piece};

fn main() {
    println!("Hello, BitBoard chess!");

    let mut game = GameState::new();
    println!("{:?}", game);

	//place a rook at D5
    game.add(Piece::Rook, Colour::Black, 3, 4).unwrap();

	//place a rook at H8
    game.add(Piece::Rook, Colour::Black, 7, 7).unwrap();

	//place a rook at A1
    game.add(Piece::Rook, Colour::Black, 0, 0).unwrap();

    //println!("{:?}", game);

    game.display();

    let game_default = GameState::new_default();

    game_default.display();
}
