extern crate ansi_term;

mod board;
mod players;

use board::Board;
use players::*;

struct Game {
    board: Board,
    white: Player,
    black: Player,
}

impl Game {
    fn new( white: Player, black: Player) -> Game {
        Game {
            board: Board::new(),
            white: white,
            black: black,
        }
    }

    pub fn run( &self ) {
        println!( "Running!!!" );
        println!( "White: {}", self.white.to_string() );
        println!( "Black: {}", self.black.to_string() );
        self.board.print();
    }
}

fn main()
{
    let white = Player::new( PlayerColor::White );
    let black = Player::new( PlayerColor::Black );

    let g = Game::new( white, black );
    g.run();
}
