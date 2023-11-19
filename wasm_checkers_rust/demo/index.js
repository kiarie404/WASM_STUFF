
fetch('./wasm_checkers_rust.wasm').then(response =>
    response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes, {
    env: {
        notify_piece_moved: (fX, fY, tX, tY) => {
            console.log("A piece moved from (" + fX + "," + fY +
                ") to (" + tX + "," + tY + ")");
        },
        notify_piece_crowned: (x, y) => {
            console.log("A piece was crowned at (" + x + "," + y + ")");
        }
    },
}
)).then(results => {
    instance = results.instance;
    console.log("At start, current turn is " + instance.exports.get_current_turn());
    let piece = instance.exports.get_piece(0, 7);
    console.log("Piece at 0,7 is " + piece);
    let res = instance.exports.move_piece(0, 5, 1, 4); // B
    console.log("First move result: " + res);
    console.log("Turn after move: " + instance.exports.get_current_turn());
    let bad = instance.exports.move_piece(1, 4, 2, 3); // illegal move
    console.log("Illegal move result: " + bad);
    console.log("Turn after illegal move: " + instance.exports.get_current_turn());
}).catch(console.error);