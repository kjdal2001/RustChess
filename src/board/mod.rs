use ansi_term::Colour;
use ansi_term::Colour::*;

enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    fn get_value( &self ) -> Colour {
        match *self {
            PieceColor::White => {
                Purple
            }
            PieceColor::Black => {
                Black
            }
        }
    }
}

enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    fn text( &self ) -> &str {
        match *self {
            PieceType::Pawn => {
                "P"
            }
            PieceType::Rook => {
                "R"
            }
            PieceType::Knight => {
                "N"
            }
            PieceType::Bishop => {
                "B"
            }
            PieceType::Queen => {
                "Q"
            }
            PieceType::King => {
                "K"
            }
        }
    }
}

struct Piece {
    piece_type: PieceType,
    color: PieceColor,
}

impl Piece {
    fn new( piece_type: PieceType, color: PieceColor ) -> Piece {
        Piece {
            piece_type: piece_type,
            color: color,
        }
    }
}

enum SquareColor {
    Black,
    White,
}

impl SquareColor {
    fn get_value( &self ) -> Colour {
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

struct Square {
    color: SquareColor,
    piece: Option< Piece >,
}

impl Square {
    fn new( color: SquareColor, piece: Option< Piece > ) -> Square {
        Square {
            color: color,
            piece: piece,
        }
    }

    fn to_string( &self ) -> String {
        let background_color = self.color.get_value();

        match self.piece {
            Some(ref p) => {
                let text_color = p.color.get_value();

                format!( "{}{}{}",
                         text_color.on( background_color ).paint( " " ),
                         text_color.on( background_color ).paint( &p.piece_type.text() ),
                         text_color.on( background_color ).paint( " " ) )
            }
            None => {
                format!("{}{}{}",
                         White.on( background_color ).paint( " " ),
                         White.on( background_color ).paint( " " ),
                         White.on( background_color ).paint( " " ) )
            }
        }
    }
}

pub struct Board {
    squares: [ [ Square; 8 ]; 8 ],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [ [ Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Rook,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Knight,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Bishop,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Queen,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::King,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Bishop,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Knight,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Rook,
                                                        PieceColor::White ) ) ) ],
                       [ Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::White ) ) ) ],
                       [ Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ) ],
                       [ Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ) ],
                       [ Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ) ],
                       [ Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ),
                         Square::new( SquareColor::White, None ),
                         Square::new( SquareColor::Black, None ) ],
                       [ Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Pawn,
                                                        PieceColor::Black ) ) ) ],
                       [ Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Rook,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Knight,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Bishop,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Queen,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::King,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Bishop,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::White,
                                      Some( Piece::new( PieceType::Knight,
                                                        PieceColor::Black ) ) ),
                         Square::new( SquareColor::Black,
                                      Some( Piece::new( PieceType::Rook,
                                                        PieceColor::Black ) ) ) ] ],
        }
    }

    pub fn print( &self ) {
        for (i, row) in self.squares.iter().enumerate().rev() {
            let res : String = row.iter().fold( String::new(), |res, s| {
                res + &s.to_string()
            } );
            println!( "{} {}", i, res );
        }
        println!( "   A  B  C  D  E  F  G  H " );
    }
}
