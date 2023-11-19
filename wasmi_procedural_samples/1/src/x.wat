(module
 (type $none_=>_f32 (func (result f32)))
 (memory $0 16)
 (table $0 1 1 funcref)
 (global $__stack_pointer (mut i32) (i32.const 1048576))
 (global $global$1 i32 (i32.const 1048576))
 (global $global$2 i32 (i32.const 1048576))
 (export "memory" (memory $0))
 (export "get_pi" (func $get_pi))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $get_pi (result f32)
  (local $0 f32)
  (local.set $0
   (f32.const 3.1415927410125732)
  )
  (return
   (local.get $0)
  )
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
