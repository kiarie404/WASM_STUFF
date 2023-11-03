fetch('./target/wasm32-unknown-unknown/debug/wasm_checkers_rust.wasm').then(response =>
    response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes)).then(results => {
    instance = results.instance;
    // document.getElementById("container").innerText = instance.exports.add_one(41);

    console.debug("I am in");
    var x = 5;
    var added = instance.exports.increment(x);
    console.debug(x, " was incremented to ", added);

}).catch(console.error);