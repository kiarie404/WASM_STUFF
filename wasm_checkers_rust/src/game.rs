use crate::board::{Coordinate, GamePiece, Move, PieceColor};

/// A struct that maintains the state of the board boxes, current turn and valid move counts (to determine if there is a draw)
pub struct GameEngine{
    pub board: [[Option<GamePiece>; 8]; 8],
    pub current_turn: PieceColor,
    pub move_count: usize,
}

/// Struct showing the Move details and whether the move resulted in a crowning or not
pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

// ---------------- Implementation block for the game-engine------------------ //
// ---------------- the impl blocks have been divided into parts acconding to their functionalit ------//

impl GameEngine {

    /// initializes the entire game state : assigns initial value to each box, sets turn, zeroes out the move count
    pub fn new() -> GameEngine{

        // Create an unpipulated empty board with the first turn set to black
        let mut temp_engine = GameEngine{
            board : [[None; 8]; 8],
            current_turn : PieceColor::Black,
            move_count : 0
        };

        // place the pawns, the game-pieces.--- this is like initializing the pieces
        temp_engine.initialize_pieces();
        return temp_engine;      
    }

    fn initialize_pieces(&mut self) {
            let white_x_coordinate_values = [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6];
            let white_y_coordinate_values = [5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7];
            let black_x_coordinate_values = [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7];
            let black_y_coordinate_values = [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2];

            white_x_coordinate_values.iter()
                                     .zip(white_y_coordinate_values.iter())
                                     .map(|(x,y)| (*x as usize, *y as usize))
                                     .for_each(|(x, y)|{
                                        self.board[x][y] = Some(GamePiece::generate_new_uncrowned(PieceColor::White));
                                     });

           black_x_coordinate_values.iter()
                                     .zip(black_y_coordinate_values.iter())
                                     .map(|(x,y)| (*x as usize, *y as usize))
                                     .for_each(|(x, y)|{
                                        self.board[x][y] = Some(GamePiece::generate_new_uncrowned(PieceColor::Black));
                                     });
    }
}



// ----------------------------  This block validates and implements movements  ---------------------------  //

impl GameEngine{

    /// This is the main function that changes the state of the Game-engine to imitating movement  
    /// It takes in a reference to Move, checks if the Move is valid, mutates the board state and returns a Option<MoveResult>
    /// It returns None if there was no change in board state
    /// It returns MoveResult if there was a change in board state. The MoveResult describes the move that caused changes : (who made it? Which co-ordinates? Was a piece taken? Was a piece crowned? )
    pub fn move_piece(&mut self, mv: &Move) -> Option<MoveResult>{
        // check if suggested move is on the board , if move is invalid, return None
        let legal_moves = self.get_legal_moves_on_board();

        // Check if move is one of the valid moves
        if !legal_moves.contains(mv){
            return None;
        }

        // make board state changes
          // The legal move could be a simple step or an attack-jump...
          // If it is a jump-attack, we remove the opponents piece.


        // Return a properly filled MoveResult that describes all board-state changes

        unimplemented!()

    }

    pub fn get_legal_moves_on_board(&self) -> Vec<Move>{
        let mut moves = Vec::new();
        for column in 0..8{
            for row in 0..8{
                // if you find a piece...
                if let Some(piece) = self.board[column][row]{
                    //...and that piece happens to belong to the Current player..
                    if piece.color == self.current_turn {
                        let loc = Coordinate::new(column, row);
                        let mut valid_moves = self.get_valid_moves_from_single_box(&loc);
                        moves.append(&mut valid_moves);
                    }
                }
            }
        }

        return moves;
    }

    fn get_valid_moves_from_single_box(&self, loc: &Coordinate) -> Vec<Move>{
        // confirm that the box coordinate passed is within the gameboard
        if !loc.is_within_board(){ return Vec::new(); }
        // if the box is empty, return an empty set of moves
        else if None == self.board[loc.x][loc.y] {   return Vec::new(); }
        // just return all jumps and steps... we will deal with this another time
        else{
            let jumps= loc.get_all_coordinate_jumps();
            let steps = loc.get_all_coordinate_steps();
            let valid_moves = jumps.chain(steps)
                                              .map(|point|{Move { from: *loc, to: point }})
                                              .collect::<Vec<Move>>();
            return valid_moves;
        }

    }
    


}