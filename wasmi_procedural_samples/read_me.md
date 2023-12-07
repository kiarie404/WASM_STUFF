This repo contains code samples of working with the wasm modules (using wasmi, WASI and wasm-component).  


The aim of these to show : 
- how to import functions (functions that have basic signatures and less_basic signatures)
- How to import memories
- How to deal with global values
- How to deal with tables
- How to deal with many wasm modules that are inter-twined and working together
- How to build a minimal library API
- How to use WASI in a modular way
- How to implement capability-based security
- How to use the wasm-component model
- How to use wasm in a no-std setup


The files are organized in the order below :    

1. Exporting a simple function from wasm. The function 
    - has no input parameters
    - outputs a basic type( integer or float). It does not return complex types such as vectors or structs
    - The wasm module has no imports
    - Uses wasmi + Rust


2. Exporting a simple function from wasm. The function
   - has no input parameters
   - returns a complex type ie a struct