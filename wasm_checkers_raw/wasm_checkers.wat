(module
    (memory 1)

    ;; deal with the state of a Box (32 bits)
    ;; Empty box = 0 == 000
    ;; Box with black pawn = 3 == 011
    ;; Box with white pawn = 1 == 001
    ;; Box with white king = 5 == 101
    ;; Box with black king = 7 == 111
    ;; Any other value is an Error

    ;; declaring checking masks
    (global $COLOR_MASK i32 (i32.const 2) )   ;; 010
    (global $CROWNED_MASK i32 (i32.const 4) ) ;; 100
    (global $EMPTY_MASK i32 (i32.const 0))    ;; 000

    ;; Declaring global variables used in turn-taking
    (global $TURN_STATE (mut i32) (i32.const 1)) ;; 1 == black, 0 == white
    (global $BLACK_TURN i32 (i32.const 1))
    (global $WHITE_TURN i32 (i32.const 0))

    ;; get box index
    (func $get_box_index (param $x i32) (param $y i32) (result i32)
        (i32.add
            (local.get $x)
            (i32.mul    (local.get $y)  (i32.const 8))
        )
    )

    ;; get box memory position
    (func $get_box_memory_position (param $x i32) (param $y i32) (result i32)
        (i32.mul
            (call $get_box_index (local.get $x) (local.get $y))
            (i32.const 4)
        )
    )



    ;; Determine if a box contains a crowned piece
    ;; Returns 1 if crowned, returns 0 if not-crowwned OR empty
    (func $is_crowned (param $box_state i32) (result i32)
        (i32.eq
            (i32.and (local.get $box_state) (global.get $CROWNED_MASK))
            (global.get $CROWNED_MASK)
        )
    )

    ;; Determine if a box contains white piece
    ;; returns 1 if white, returns 0 if black OR empty
    (func $is_white (param $box_state i32) (result i32)
        (if (result i32)(call $is_empty (local.get $box_state))
            (then (i32.const 0))
            (else
                (i32.eqz
                    (i32.and (local.get $box_state) (global.get $COLOR_MASK))
                ) 
            )
        )
    )

    ;; Determine if a box contains black piece
    ;; returns 1 if black, returns 0 if white OR empty
    (func $is_black (param $box_state i32) (result i32)
        (i32.eq
            (i32.and (local.get $box_state) (global.get $COLOR_MASK))
            (global.get $COLOR_MASK)
        )
    )

    ;; Determine if box is empty
    (func $is_empty (param $box_state i32) (result i32)
        (i32.eq
            (local.get $box_state)
            (i32.const 0)
        )
    )

    ;; Crown a piece
    ;; This function is immutable
    (func $crown_state (param $box_state i32) (result i32)
        (i32.or
            (local.get $box_state)
            (global.get $CROWNED_MASK)
        )
    )

    ;; Decrown the state of a box
    ;; immutable in nature
    (func $decrown_state (param $box_state i32) (result i32)
        (i32.and
            (local.get $box_state)
            (call $invert_bits (global.get $CROWNED_MASK))
        )
    )

    ;; invert bits of a 32 bit int
    (func $invert_bits (param $mask i32) (result i32)
        (i32.xor
            (i32.const 4294967295) ;; a mask with all bits set to 1
            (local.get $mask)
        )
    )

    ;; set state of a box in linear memory
    (func $set_box_state (param $x i32) (param $y i32) (param $box_state i32)
        (i32.store
            (call $get_box_memory_position (local.get $x) (local.get $y))
            (local.get $box_state)
        )
    )

    ;; Detect if values are within range (inclusive high and low)
    (func $inRange (param $low i32) (param $high i32) (param $value i32) (result i32)
        (i32.and
            (i32.ge_s (get_local $value) (get_local $low))
            (i32.le_s (get_local $value) (get_local $high))
        )
    )

    ;; get state of a box from the linear memory
    (func $get_box_state (param $x i32) (param $y i32) (result i32)
        (if (result i32) 
            (block (result i32)
                (i32.and
                    (call $inRange (i32.const 0) (i32.const 7) (get_local $x)    )
                    (call $inRange (i32.const 0) (i32.const 7) (get_local $y)    )
                )
            )

            (then
                (i32.load (call $get_box_memory_position (local.get $x) (local.get $y)))
            )

            (else  (unreachable))
        )
    )

    
    ;; Turn Taking logic

    ;; Get current turn
    (func $get_turn_state (result i32)
        (global.get $TURN_STATE)
    )

    ;; Set current turn
    (func $set_turn_state (param $state i32)
        (global.set $TURN_STATE (local.get $state))
    )

    ;; Toggle turn state
    (func $togle_turn_state
        (if 
            (i32.eq (i32.const 1) (global.get $TURN_STATE)) ;; if state is black...
            (then (call $set_turn_state( i32.const 0))) ;; set it to white
            (else (call $set_turn_state( i32.const 1))) ;; set it to black
        )
    )
    
    ;; Determine if it is black's turn
    (func $is_blacks_turn (result i32)
        (i32.eq (global.get $TURN_STATE) (global.get $BLACK_TURN))
    )

    ;; Determine if it's whites turn
    (func $is_whites_turn (result i32)
        (i32.eq (global.get $TURN_STATE) (global.get $WHITE_TURN))
    )

    ;; ----------------- Playing rules ---------------------------- ;;

    ;; Crowning Rules
    ;; Should this piece get crowned?
    ;; We crown black pieces in row 0, white pieces in row 7
    (func $should_we_crown (param $x i32) (param $y i32) (result i32)
        (block (result i32)

            ;; black logic
            ;; if black, uncrowned and in row 0... return true
            (if (result i32)
                (i32.and
                    (call $is_black (call $get_box_state (local.get $x) (local.get $y))) ;; is black
                    (i32.eq (local.get $y) (i32.const 0))   ;; is in row 0
                )
                
                (then (i32.const 1)) ;; return true
                (else 
                    ;; white logic
                    ;; if white, uncrowned and in row 7... return true
                    (if (result i32)
                        (i32.and
                            (call $is_white (call $get_box_state (local.get $x) (local.get $y))) ;; is white
                            (i32.eq (local.get $y) (i32.const 7))   ;; is in row 7
                        )
                        
                        (then (i32.const 1)) ;; return true
                        (else (i32.const 0)) ;; return false for all other cases
                    )
                )
            )
        )
    )


    ;; ------------------ Movement Rules ------------------  ;;

    ;; A move is valid if ...
    ;; • The “move distance” from current to target (single axis) is valid
    ;; • The target space is unoccupied
    ;; • The piece being moved belongs to the current player

    ;; checks if the piece moved a valid distance
    ;; unimplemented, for now... both x and y should be more than their originals by 1
    (func $is_valid_distance (param $x_from i32) (param $y_from i32) (param $x_to i32) (param $y_to i32) (result i32)
        (i32.const 1)
    )



    ;; Exports
    (export "get_box_index" (func $get_box_index))
    (export "get_box_memory_position" (func $get_box_memory_position))
    (export "is_crowned" (func $is_crowned))
    (export "is_white" (func $is_white))
    (export "is_black" (func $is_black))
    (export "is_empty" (func $is_empty))
    (export "crown_state" (func $crown_state))
    (export "decrown_state" (func $decrown_state))
)