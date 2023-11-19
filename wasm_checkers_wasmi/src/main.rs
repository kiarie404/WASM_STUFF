
use wasmi::{Engine};
use anyhow::{Result, anyhow};
use std::fs;



fn main() -> Result<(), anyhow::Error>{

    // The intepreter itself (Compiles a wat/wasm file to Machine code and facilitates its execution by communicating with the host OS)
    // Begins the Engine with default configs on things like : float_point support, number and height of stacks, support for proposed/unofficial wasm features
    // These configs get represented as a struct called Config  
    let engine = Engine::default();

    // Now we need to validate the raw wasm file.  
    // A validated wasm file is refered to as a Module
    // The Module::new function expects the wasm file to be a byte stream, so we will first turn our wasm file into a byte stream
    let mut byte_stream = Vec::new();
    {
        use std::io::Read; 
        let mut wasm_file = fs::File::open("/src/x.wasm")?;
        wasm_file.read_to_end(&mut byte_stream);
    }


    // Now that we have a readable byte stream, we can easily validate it using the Module::new() function
    let module = wasmi::Module::new(&engine, byte_stream.as_slice())?;


    unimplemented!()
}