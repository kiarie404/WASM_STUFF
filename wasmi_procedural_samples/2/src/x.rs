

//! This wasm module ....   
//! 1. Has no imports
//! 2. Exports a simple function that :
//!     2.1 Takes no arguments
//!     2.2 Returns a complex type (ie something that is not f32, f64, i32, i64)



#![crate_type = "cdylib"]


#[no_mangle]
pub extern "C" fn get_user() -> User{
    User { id: 6.1, active_status: true }
}


#[repr(C)]  // use this to make the struct FFI friendly. C structs do not rearrange themselves during compiler optimization
pub struct User{
    pub id : f32,
    pub active_status : bool
}
