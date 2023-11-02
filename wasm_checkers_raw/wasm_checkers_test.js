fetch('./wasm_checkers.wasm').then(response =>
    response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes)).then(results => {
    instance = results.instance;
    // document.getElementById("container").innerText = instance.exports.add_one(41);

    // test index finding function
    console.debug("Testing Index generation from the coordinates passed");
    console.debug("(2,5) = ", instance.exports.get_box_index(2,5));
    console.debug("(3,0) = ", instance.exports.get_box_index(3,0));

    // test memory position finding
    console.debug("Testing memory position generation...");
    console.debug("(0,0) = ", instance.exports.get_box_memory_position(0, 0));
    console.debug("(5,5) = ", instance.exports.get_box_memory_position(5, 5));

    // Testing if box states are okay
    console.debug("Testing box states....");
    console.debug("Black king state: ");
    var black_king = 7;
    console.debug("     crowned : ", instance.exports.is_crowned(black_king));
    console.debug("     black : ", instance.exports.is_black(black_king));
    console.debug("     white : ", instance.exports.is_white(black_king));
    console.debug("     empty : ", instance.exports.is_empty(black_king));

    console.debug("White king state: ");
    var value = 5;
    console.debug("     crowned : ", instance.exports.is_crowned(value));
    console.debug("     black : ", instance.exports.is_black(value));
    console.debug("     white : ", instance.exports.is_white(value));
    console.debug("     empty : ", instance.exports.is_empty(value));

    console.debug("White Pawn state: ");
    var value = 1;
    console.debug("     crowned : ", instance.exports.is_crowned(value));
    console.debug("     black : ", instance.exports.is_black(value));
    console.debug("     white : ", instance.exports.is_white(value));
    console.debug("     empty : ", instance.exports.is_empty(value));

    console.debug("Black Pawn state: ");
    var value =3;
    console.debug("     crowned : ", instance.exports.is_crowned(value));
    console.debug("     black : ", instance.exports.is_black(value));
    console.debug("     white : ", instance.exports.is_white(value));
    console.debug("     empty : ", instance.exports.is_empty(value));

    console.debug("Empty state: ");
    var value = 0;
    console.debug("     crowned : ", instance.exports.is_crowned(value));
    console.debug("     black : ", instance.exports.is_black(value));
    console.debug("     white : ", instance.exports.is_white(value));
    console.debug("     empty : ", instance.exports.is_empty(value));

    console.debug("Trash state: ");
    var value = 15;
    console.debug("     crowned : ", instance.exports.is_crowned(value));
    console.debug("     black : ", instance.exports.is_black(value));
    console.debug("     white : ", instance.exports.is_white(value));
    console.debug("     empty : ", instance.exports.is_empty(value));

    // Testing crowning and decrownning
    console.debug("Testing crowning ");
    console.debug("Passing a 7 yields a 7 as seen : ", instance.exports.crown_state(7));
    console.debug("Passing a 5 yields a 5 as seen : ", instance.exports.crown_state(5));
    console.debug("Passing a 3 yields a 7 as seen : ", instance.exports.crown_state(3));
    console.debug("Passing a 1 yields a 5 as seen : ", instance.exports.crown_state(1));


    console.debug("Testing decrowning ");
    console.debug("Passing a 7 yields a 3 as seen : ", instance.exports.decrown_state(7));
    console.debug("Passing a 5 yields a 1 as seen : ", instance.exports.decrown_state(5));
    console.debug("Passing a 3 yields a 3 as seen : ", instance.exports.decrown_state(3));
    console.debug("Passing a 1 yields a 1 as seen : ", instance.exports.decrown_state(1));

}).catch(console.error);