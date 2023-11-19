(module
    
    (import "env" "print_num" (func $print_num (param i32)))
    (memory 1)

    (func $add_show (param $x i32) (param $y i32)
        (call $print_num 
            (i32.add
                (local.get $x)
                (local.get $y)
            )
        )
    )

    (export "add_show" (func $add_show))
)