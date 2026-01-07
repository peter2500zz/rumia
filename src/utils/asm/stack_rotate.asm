
stack_rotate:
    // 序言
    push ebp
    mov ebp, esp
    // ebp: caller ebp
    // ebp + 4: ret addr
    // ebp + 8: counts: u32
    // ebp + 0xC: 🤤

    // 保存状态
    pushad
    pushfd

    // i = counts;
    mov ecx, [ebp+8]

    // if (i == 0) return;
    test ecx, ecx
    jz rotate_stack_end

    // stack_temp_value = 0;
    xor edx, edx

    // do {{
rotate_stack:
        // stack_offset = (i - 1) << 2;
        mov eax, ecx
        dec eax
        shl eax, 2

        // tmp = *(ebp + 0xC + stack_offset);
        mov ebx, [ebp+eax+0xC]
        // *(ebp + 0xC + stack_offset) = stack_temp_value;
        mov [ebp+eax+0xC], edx
        // stack_temp_value = tmp;
        mov edx, ebx

        // i--;
    // }} while (i != 0);
    dec ecx
    jnz rotate_stack

    // top_stack_offset = (counts - 1) << 2;
    mov eax, [ebp+8]
    dec eax
    shl eax, 2

    // *(ebp + 0xC + top_stack_offset) = stack_temp_value;
    mov [ebp+eax+0xC], edx

    // return;
rotate_stack_end:
    // 还原状态
    popfd
    popad

    // 尾声
    leave
    ret 4
