#[no_mangle]
pub extern "C" fn get_user() -> User{
    return User { id: 6.1, active_status: true };
}


#[repr(C)]  // use this to make the struct FFI friendly. C structs do not rearrange themselves during compiler optimization
struct User{
    id : f32,
    active_status : bool
}
