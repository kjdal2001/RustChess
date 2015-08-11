pub enum PlayerColor {
    White,
    Black,
}

impl PlayerColor {
    fn to_string( &self ) -> String {
        match *self {
            PlayerColor::White => {
                "White".to_string()
            }
            PlayerColor::Black => {
                "Black".to_string()
            }
        }
    }
}

pub struct Player {
    color: PlayerColor,
}

impl Player {
    pub fn new( color: PlayerColor ) -> Player {
        Player {
            color: color
        }
    }

    pub fn to_string( &self ) -> String {
        format!( "{}", self.color.to_string() ).to_string()
    }
}
