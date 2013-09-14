global upcall_call_shim_on_rust_stack

upcall_call_shim_on_rust_stack:
    push ebp
    mov ebp, esp

    push dword[ebp+8]
    call dword[ebp+12]

    mov esp, ebp
    pop ebp
    ret
