https://www.felixcloutier.com/x86/mov

- [ ] MOV r/m64, r64
    - [x] MOV r64, r64
    - [ ] MOV m64, r64

Current goal is to prove this naked function
```rust
pub unsafe extern "sysv64" fn switch_to_task_inner(current_task: *mut Task, next_task: *mut Task) {
    core::arch::naked_asm!(
        "mov [rdi+48], r15",
        "mov r15, [rsi+48]",
        "mov [rdi+40], r14",
        "mov r14, [rsi+40]",
        "mov [rdi+32], r13",
        "mov r13, [rsi+32]",
        "mov [rdi+24], r12",
        "mov r12, [rsi+24]",
        "mov [rdi+16], rbx",
        // "mov rbx, [rsi+16]",
        "mov [rdi+8], rbp", // store rbp
        "mov rbp, [rsi+8]", //load rbp
        "mov [rdi], rsp",   // store rsp in task struct
        "mov rsp, [rsi]",   // load rsp from the next task
        "mov rbx, cr3",     // change virtual address spaces
        "mov [rdi+56], rbx",
        "mov rbx, [rsi+16]",
        "push rbx",
        "mov rbx, [rsi+56]",
        "mov cr3, rbx",
        "pop rbx",
        "ret",
    );
}
```
