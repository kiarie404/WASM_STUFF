// Abstractions

// Enum that contains the only possible piece colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece{
    color : PieceColor,
    crowned : bool
}

// the Co-ordinate
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate{
    x : usize,
    y : usize
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}


// --------- Struct implementations --------------- //

impl GamePiece{
    /// function receives a color and returns an uncrowned Gamepiece
    fn generate_new_uncrowned(color: PieceColor) -> GamePiece{
        return GamePiece { color: color, crowned: false };
    }

    /// Function immutably consumes an uncrowned gamePiece and returns a crowned piece
    fn generate_new_crowned(uncrowned_piece : GamePiece) -> GamePiece{
        return GamePiece { color: uncrowned_piece.color, crowned: true };
    }
}

impl Coordinate{
    /// Checks whether the co-ordinate is within the 8x8 board and returns true if its within the 8x8 bounds 
    fn is_within_board(&self) -> bool{
        return (self.x <= 7) && (self.y <= 7); 
    }

    /// Jumps are done when attacking.  
    /// This function returns possible attack Coordinates from the current co-ordinate
    fn get_all_jumps(&self) -> impl Iterator<Item = Coordinate>{
        let mut jumps = Vec::new();

        if self.y >= 2 { jumps.push(Coordinate{x: self.x + 2, y: self.y -2});    }
        jumps.push(Coordinate{x: self.x + 2, y: self.y + 2});

        if self.x >= 2 && self.y >= 2 { jumps.push(Coordinate { x: self.x - 2, y: self.y -2 });}
        if self.x >= 2 { jumps.push(Coordinate{x: self.x - 2, y: self.y + 2});}

        return jumps.into_iter();
    }


    /// moves are different from jumps. moves are not meant for attacks. 
    /// This function returns an iterator with the moves
    fn get_all_moves(&self) -> impl Iterator<Item = Coordinate>{
        let mut moves = Vec::new();

        if self.x >= 1 { moves.push(Coordinate{x: self.x - 1, y: self.y + 1});}
        moves.push(Coordinate { x: self.x + 1, y: self.y + 1 });

        if self.y >= 1 {    moves.push(Coordinate { x: self.x + 1, y: self.y - 1 }); }
        if self.x >= 1 && self.y >= 1 { moves.push(Coordinate { x: self.x - 1, y: self.y - 1 });    }

        return moves.into_iter();
    }
}

impl Move{
    fn new ( from: (usize, usize), to: (usize, usize)) -> Move{
        return Move{ 
                    from: Coordinate { x: from.0, y: from.1 },
                    to  : Coordinate { x: to.0, y: to.1 }
                };
    }
}
