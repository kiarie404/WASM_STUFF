//! This program illustrates scenario 1 as described in the read_me of the repo


// ------- Feautures Used ------------- //
#![feature(error_in_core)]


// core usages
use core::error::Error;
use core::{fmt, fmt::Display};

// foreign and std
use std::{fs};
use wasmi::{Engine, Linker, TypedFunc};



fn main() -> Result<(),  Box<dyn Error>>{

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
        let mut wasm_file = fs::File::open("x.wasm").expect("Cannot find x.wasm file");
        wasm_file.read_to_end(&mut byte_stream)?;
    }


    // Now that we have a readable byte stream, we can easily validate it using the Module::new() function\
    let byte_stream_slice = byte_stream.as_slice();


    // We then create a Module. If we dont manage to create a module, we return an Error
    let module;
    let module_creation_result = wasmi::Module::new(&engine, byte_stream_slice);
    match module_creation_result {
        Ok(val) => module = val,
        // the value in the Err(val) below is a wasmi::Error
        // wasmi::error does not implement the Error trait, so we enclose wasmi::Error in our own struct that implements the Error trait 
        Err(val) => {
            let temp = Box::new(WasmiError{source: val});
            return  Err(temp);
        }
    }


    // Now that we have a module, it is time to instantiate it
        // The linker is what assists us in the instantiation process; it satisfies imports and returns an unstarted module
        let linker = Linker::new(&engine);

        // we also need to build the Context of the wasm module : (store + HostState). THe stack will be built implicitly.
        // Now, in wasmi, Context is implemented as a trait (AsContext). The store struct implements the AsContext trait.... So where does the HostState Go?
        // The HostState it declared as part of the store. ie when you create the store, you need to declare it AND pass to it the HostState
        // There can only be one Store per execution context

        // so we declare the HostState type
        type HostState = Option<i32>; // you can choose whichever datastructure you want, I chose this for the sake of simplicity. I wont be using the Hoststate in this example
        let shared_state: HostState = None; // And I Chose None just to make sure no garbage values are passed in this example... 

        // Next we declare the Store. The store automatically implements the AsContext and AsContextMut trait
        let mut store = wasmi::Store::new(&engine, shared_state);

        // So now that we have context, we can instantiate our module
        let module_instance_pre_result = linker.instantiate(&mut store, &module);

        // Now we have an InstancePre. A partially instantiated Instance where the start function has not yet been executed.
        // In our case, our module does not and SHOULD NOT have a start function. In other cases, we might. But for this case... WE DONT need pne that has the start function
        let module_instance;
        match module_instance_pre_result {
            Ok(module_instance_pre) => module_instance = module_instance_pre.ensure_no_start(&mut store).expect("The wasm file has a start function... BAD"),
            Err(val) => {return Err(Box::new(WasmiError{source: val}));}
        }


        // Now we have a running instance that we can invoke functions from and interact with the other ojects it exports

        // But first, we get the function reference of the function that wasm module exported
        
        // prints all exports... testing purposes
        for element in module_instance.exports(&store){
            println!("{:?}", element);
        }

        // get_PI function
         // specify the parameters and results of the Typed Func. Each Param or Result should implement the WasmType trait
        let imported_get_pi_func: TypedFunc<(), wasmi::core::F32> = module_instance.get_typed_func(&store, "get_pi")
                                                                                   .expect("the wasm module does not export the get_pi function");

        // use the get_pi function
        let pi = imported_get_pi_func.call(&mut store, ()).expect("unable to get pi")
                                                                           .to_float();

        // confirm that you have got pi
        println!("The value of pi = {}", pi);
    
    


    Ok(())
    // unimplemented!()
}




#[derive(Debug)]
struct WasmiError {   source : wasmi::Error }

impl Display for WasmiError{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        self.source.fmt(f)
    }
}

impl Error for WasmiError{ }


