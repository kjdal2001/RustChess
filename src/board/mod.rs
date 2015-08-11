use ansi_term::Colour;
use ansi_term::Colour::*;

enum SquareColor {
    Black,
    White,
}

impl SquareColor {
    fn get_background( &self ) -> Colour {
        match *self {
            SquareColor::Black => {
                Green
            }
            SquareColor::White => {
                Yellow
            }
        }
    }
}

enum Piece {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    fn text( &self ) -> &str {
        match *self {
            Piece::None => {
                " "
            }
            Piece::Pawn => {
                "P"
            }
            Piece::Rook => {
                "R"
            }
            Piece::Knight => {
                "N"
            }
            Piece::Bishop => {
                "B"
            }
            Piece::Queen => {
                "Q"
            }
            Piece::King => {
                "K"
            }
        }
    }
}

struct Square {
    color: SquareColor,
    piece: Piece,
}

impl Square {
    fn new( color: SquareColor, piece: Piece ) -> Square {
        Square {
            color: color,
            piece: piece,
        }
    }

    fn to_string( &self ) -> String {
        let background_color = self.color.get_background();
        let text_color = Red;

        format!( "{}{}{}",
                 text_color.on( background_color ).paint( " " ),
                 text_color.on( background_color ).paint( &self.piece.text() ),
                 text_color.on( background_color ).paint( " " ) )
    }
}

pub struct Board {
    squares: [ [ Square; 8 ]; 8 ],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [ [ Square::new( SquareColor::Black, Piece::Rook ),
                         Square::new( SquareColor::White, Piece::Knight ),
                         Square::new( SquareColor::Black, Piece::Bishop ),
                         Square::new( SquareColor::White, Piece::Queen ),
                         Square::new( SquareColor::Black, Piece::King ),
                         Square::new( SquareColor::White, Piece::Bishop ),
                         Square::new( SquareColor::Black, Piece::Knight ),
                         Square::new( SquareColor::White, Piece::Rook ) ],
                       [ Square::new( SquareColor::White, Piece::Pawn ),
                         Square::new( SquareColor::Black, Piece::Pawn ),
                         Square::new( SquareColor::White, Piece::Pawn ),
                         Square::new( SquareColor::Black, Piece::Pawn ),
                         Square::new( SquareColor::White, Piece::Pawn ),
                         Square::new( SquareColor::Black, Piece::Pawn ),
                         Square::new( SquareColor::White, Piece::Pawn ),
                         Square::new( SquareColor::Black, Piece::Pawn ) ],
                       [ Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ) ],
                       [ Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ) ],
                       [ Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ) ],
                       [ Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ),
                         Square::new( SquareColor::White, Piece::None ),
                         Square::new( SquareColor::Black, Piece::None ) ],
                       [ Square::new( SquareColor::Black, Piece::Pawn ),
                         Square::new( SquareColor::White, Piece::Pawn ),
                         Square::new( SquareColor::Black, Piece::Pawn ),
                         Square::new( SquareColor::White, Piece::Pawn ),
                         Square::new( SquareColor::Black, Piece::Pawn ),
                         Square::new( SquareColor::White, Piece::Pawn ),
                         Square::new( SquareColor::Black, Piece::Pawn ),
                         Square::new( SquareColor::White, Piece::Pawn ) ],
                       [ Square::new( SquareColor::White, Piece::Rook ),
                         Square::new( SquareColor::Black, Piece::Knight ),
                         Square::new( SquareColor::White, Piece::Bishop ),
                         Square::new( SquareColor::Black, Piece::Queen ),
                         Square::new( SquareColor::White, Piece::King ),
                         Square::new( SquareColor::Black, Piece::Bishop ),
                         Square::new( SquareColor::White, Piece::Knight ),
                         Square::new( SquareColor::Black, Piece::Rook ) ] ],
        }
    }

    pub fn print( &self ) {
        for row in self.squares.iter().rev() {
            let res : String = row.iter().fold( String::new(), |res, s| {
                res + &s.to_string()
            } );
            println!( "{}", res );
            /*for square in row.iter() {
                println!( "{}", square.to_string() );
            }*/
        }
    }
}
