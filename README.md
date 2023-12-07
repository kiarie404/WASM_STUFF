# WASM_STUFF

This repo contains wasm sample code that I may reference in the future.  

## WHY?
wasm is a viable choice for packaging software in a portable way, simple way without a lot of boiler-plate-technical clutter.  

It facilitates software that runs on any device OR on top of other system-software in a standard and constant way.   
I see this as a basis for writing durable, bare software. Let's cut the bloat out whistle maintaining portability and security.  


In software programming, abstraction is everything... right? Well, wasm looks like a stable abstraction that I can use for the next couple of decades without worrying about maintenance... and keeping up with the shiny tools... this is just a standard abstraction.  



So I begin here, understanding wasm on embedded devices...     

### Runtimes 

There are 3 wasm-runtimes that are used in this repo. 
  1. Wasmtime
  2. Wasmi
  3. Wasm-micro-runtime
  
Both are written in Rust. Both have support for WASI.	Both try their best to adhere to the official specifications. They are in active development, they are not dead.  

#### 1. Wasmtime


##### Why wasmtime?	
1. **Good Hands** - Wasmtime is built by the Bytecode Alliance, a non-profit organization dedicated to creating secure new software foundations, building on standards such as WebAssembly and WebAssembly System Interface (WASI). They are in the frontlines developing ;standards such as Component-model, wasm-deelopment tools, runtimes, niche libraries... 


2. **Support for wasm-component model** - Currently(Dec, 2023), wasmtime is among the very few runtimes that have began to implement the wasm-component model

3. **Ergonomics** - wasmtime has many awesome tools in their repo. Wasmtime runs on many major Oses. They make it easy for developers to treat wasm-modules just like normal dependencies. From importation, API interaction, execution to debugging.  

4. **Support for AoT, intepretation and JIT compilation**

##### Why NOT wasmtime?
1. **Difficult Baremetal Support** - to run wasmtime on a no-std platform, you have to tweak it yourself. That in itself is hard task. Wasmtime is in active development, so you have to continuously try to keep your no-std_wasmtime_clone up to date by yourself.  


#### 2. Wasmi

##### Why wasmi?  
1. **Support for No-std** 
2. Compliance to wasm official specs. It has similarities to wasmtime, so this smoothens the learning curve. 
3. WASI-compliant

#### Why not wasmi?
1. No support for the component model. (this is a big deal)
2. Not as bleeding-edge as wasmtime in terms of features supported. But it supports core specs... so we can deal with it


#### 3. Wasm-micro-runtime

This runtime has all the right curves... but it is written in C. 


This runtime is lightweight, well-thought-out and performant... making it a good choice for embedded systems.  
It has features that are at par with wasmtime. It has both standard and a few proposed features.  
It partially implements the component model.  
It is under the maintenance of Bytecode Alliance, so there is assurance that it is in safe hands.  
The tooling and documentation around it is great.

**BUT IT IS WRITTEN IN C!!**  
This is very sad, but completely understandable. It would have been perfect if it were written by another language like Rust.  
C is awesome. A necessary tool in the embedded world. Sad but tolerable.  


