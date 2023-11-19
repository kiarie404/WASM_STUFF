use lazy_static::*;
use mut_static::MutStatic;

mod board;
mod game;

use game::GameEngine;
use board::{ PieceColor, GamePiece, Move};


// ---------------- defining imports --------------- //

// These are imports from the host or other modules that will run in tandem with THIS module 
extern "C" {
    fn notify_piece_moved(fx: i32, fy: i32, tx: i32, ty: i32);
    fn notify_piece_crowned (tx: i32, ty: i32);
}

// ------ CREATING THE API ------------------ //

// making the Game-Engine state global and mutable
lazy_static!{
    static ref GAME_ENGINE : MutStatic<GameEngine> =  MutStatic::from(GameEngine::new()) ;
}


// Exported functions
#[no_mangle]
pub extern "C" fn get_piece(x: i32, y: i32) -> GamePiece {
    // Get read access to the global mutable static... and lock it
    let engine = GAME_ENGINE.read().unwrap();
    
    // get piece at Co-ordicate
    let piece = engine.board[x as usize][y as usize];
    match piece {
       Some(state) => state.into(),
       None => GamePiece { color: PieceColor::Black, crowned: true } 
    }

}


#[no_mangle]
pub extern "C" fn get_current_turn() -> i32{
    let engine = GAME_ENGINE.read().unwrap();
    let current_turn = engine.current_turn.clone();
    match current_turn {
        PieceColor::Black => 1,
        PieceColor::White => 0
    }
}


/// Mutates the game state to implement a move of a piece from one cordinate to another  
/// Returns a 1 if move made changes. returns a 0 if move did not make the necassary changes    
/// It also notifies the host that a move has just happened. It also notifies the host when crowning happens
#[no_mangle]
pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
    let mut engine = GAME_ENGINE.write().unwrap();
    let mv = Move::new((fx as usize, fy as usize), (tx as usize, ty as usize));
    let res = engine.move_piece(&mv);

    match res {
        Some(mr) => {
            unsafe { notify_piece_moved(fx, fy, tx, ty); }
            if mr.crowned == true { unsafe {notify_piece_crowned(tx, ty); }}
            return 1;
        },

        None => 0 
    }
            
}
