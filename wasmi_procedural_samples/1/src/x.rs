

//! This wasm module ....   
//! 1. Has no imports
//! 2. Exports a simple function that :
//!     2.1 Takes no arguments
//!     2.2 Returns a simple type



#![crate_type = "cdylib"]




/// Takes in no arguments. It returns PI as an f32
#[no_mangle]
pub extern "C" fn get_pi() -> f32{
    core::f32::consts::PI
}
