use crate::board::{Coordinate, GamePiece, Move, PieceColor};

/// A struct that maintains the state of the board boxes, current turn and valid move counts (to determine if there is a draw)
pub struct GameEngine{
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: usize,
}

/// Struct showing the Move details and whether the move resulted in a crowning or not
pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

// ---------------- Implementation block ------------------ //

impl GameEngine {

    /// initializes the entire game state : assigns initial value to each box, sets turn, zeroes out the move count
    fn new() -> GameEngine{

        // Create an unpipulated empty board with the first turn set to black
        let mut temp_engine = GameEngine{
            board : [[None; 8]; 8],
            current_turn : PieceColor::Black,
            move_count : 0
        };

        // place the pawns, the game-pieces.
        

    }
}