use std::arch::naked_asm;

/// 💀DANGEROUS💀
///
/// Rotates stack entries from higher addresses to lower addresses,
/// without modifying any registers.
///
/// Example:
/// Given a function with the `stdcall` calling convention:
///
/// ```rust
/// extern "stdcall" fn foo(arg1: i32, arg2: i32)
/// ```
///
/// Stack layout on entry:
///
/// | Name      | Address |
/// |-----------|---------|
/// | arg2      | esp + 8 |
/// | arg1      | esp + 4 |
/// | ret addr  | esp     |
///
/// Calling `stack_rotate(3)` results in:
///
/// | Name      | Address |
/// |-----------|---------|
/// | ret addr  | esp + 8 |
/// | arg2      | esp + 4 |
/// | arg1      | esp     |
///
/// Note: The `counts` parameter should specify the original number of stack
/// entries to rotate, without accounting for the stack changes caused by
/// pushing the `counts` argument itself.
#[unsafe(naked)]
pub unsafe extern "stdcall" fn stack_rotate(counts: u32) {
    naked_asm!(include_str!("asm/stack_rotate.asm"))
}
