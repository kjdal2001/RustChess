enum SquareColor {
    Black,
    White,
}

impl SquareColor {
    fn to_string( &self ) -> String {
        match *self {
            SquareColor::Black => {
                "Black".to_string()
            }
            SquareColor::White => {
                "White".to_string()
            }
        }
    }
}

enum Cell {
    Empty,
}

impl Cell {
    fn to_string( &self ) -> String {
        match *self {
            Cell::Empty => {
                "Empty".to_string()
            }
        }
    }
}

struct Square {
    color: SquareColor,
    cell: Cell,
}

impl Square {
    fn new( color: SquareColor ) -> Square {
        Square {
            color: color,
            cell: Cell::Empty,
        }
    }

    fn to_string( &self ) -> String {
        format!( "This is a square {} {}", self.color.to_string(), self.cell.to_string() ).to_string()
    }
}

pub struct Board {
    squares: [ [ Square; 2 ]; 1 ],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [ [ Square::new( SquareColor::Black ), Square::new( SquareColor::White ) ] ],
        }
    }

    pub fn print( &self ) {
        for row in self.squares.iter() {
            for squ in row.iter() {
                //let x: u8 = squ;
                println!( "{}", squ.to_string() );
            }
        }
    }
}
