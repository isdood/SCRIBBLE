section .text
global _start

_start:
    ; Get argc
    pop rax
    
    ; If less than 3 args, jump to exec original binary
    cmp rax, 3
    jl exec_binary
    
    ; Save argc
    push rax
    
    ; Get program name (argv[0])
    pop rdi
    pop rdi
    
    ; Get command (argv[1])
    pop rsi
    
    ; Get module spec (argv[2])
    pop rdx
    
    ; Prepare execve args
    push rdx        ; module
    push rsi        ; command
    push rdi        ; program
    mov rdx, 0      ; envp = NULL
    mov rsi, rsp    ; argv
    lea rdi, [rel binary_path] ; path to real binary
    mov rax, 59     ; syscall number for execve
    syscall
    
exec_binary:
    ; If execve fails or not enough args, run original binary
    lea rdi, [rel binary_path]
    mov rax, 59
    syscall
    
    ; Exit if all fails
    mov rax, 60
    xor rdi, rdi
    syscall

section .data
binary_path: db "./zig-out/bin/seed",0
