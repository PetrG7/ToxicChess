mod gamestate;
use gamestate::GameState;

fn main() {
	println!("Hello, BitBoard chess!");

	let game = GameState::new();
	println!("{:?}", game);
}
