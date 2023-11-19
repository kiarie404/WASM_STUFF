(module
 (type $i32_=>_none (func (param i32)))
 (memory $0 16)
 (table $0 1 1 funcref)
 (global $__stack_pointer (mut i32) (i32.const 1048576))
 (global $global$1 i32 (i32.const 1048576))
 (global $global$2 i32 (i32.const 1048576))
 (export "memory" (memory $0))
 (export "get_user" (func $get_user))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $get_user (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 f32)
  (local $7 i32)
  (local.set $1
   (global.get $__stack_pointer)
  )
  (local.set $2
   (i32.const 16)
  )
  (local.set $3
   (i32.sub
    (local.get $1)
    (local.get $2)
   )
  )
  (local.set $4
   (i32.const 1086534451)
  )
  (i32.store offset=8
   (local.get $3)
   (local.get $4)
  )
  (local.set $5
   (i32.const 1)
  )
  (i32.store8 offset=12
   (local.get $3)
   (local.get $5)
  )
  (local.set $6
   (f32.load offset=8
    (local.get $3)
   )
  )
  (local.set $7
   (i32.load8_u offset=12
    (local.get $3)
   )
  )
  (i32.store8 offset=4
   (local.get $0)
   (local.get $7)
  )
  (f32.store
   (local.get $0)
   (local.get $6)
  )
  (return)
 )
 ;; custom section ".debug_abbrev", size 4148
 ;; custom section ".debug_info", size 419854
 ;; custom section ".debug_ranges", size 171832
 ;; custom section ".debug_str", size 689291
 ;; custom section ".debug_pubnames", size 258385
 ;; custom section ".debug_pubtypes", size 468
 ;; custom section ".debug_line", size 286055
 ;; custom section "producers", size 75
)
