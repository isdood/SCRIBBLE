.section .text
.global format_string

format_string:
    # Input: rdi = string pointer
    push %rbp
    mov %rsp, %rbp

    # Count string length
    mov %rdi, %rsi
    xor %rcx, %rcx
.count_loop:
    cmpb $0, (%rsi)
    je .count_done
    inc %rcx
    inc %rsi
    jmp .count_loop
.count_done:

    # Allocate buffer
    mov %rcx, %r12       # Save length
    lea (%rcx, %rcx, 1), %rdi  # Double for worst case
    add $16, %rdi        # Extra space
    mov $9, %rax         # sys_mmap
    xor %rsi, %rsi
    mov $3, %rdx         # PROT_READ | PROT_WRITE
    mov $0x22, %r10      # MAP_PRIVATE | MAP_ANONYMOUS
    mov $-1, %r8
    xor %r9, %r9
    syscall

    # Format string
    mov %rax, %rdi       # Destination buffer
    mov 8(%rbp), %rsi    # Source string
    xor %rcx, %rcx       # Counter
.format_loop:
    movb (%rsi), %al
    testb %al, %al
    jz .format_done

    cmpw $0x5f5f, (%rsi)  # Check for "__"
    je .replace_stars

    movb %al, (%rdi)
    inc %rsi
    inc %rdi
    jmp .format_loop

.replace_stars:
    movw $0x2a2a, (%rdi)  # Write "**"
    add $2, %rsi
    add $2, %rdi
    jmp .format_loop

.format_done:
    movb $0, (%rdi)      # Null terminate

    # Return formatted string
    mov %rax, %rax
    leave
    ret
