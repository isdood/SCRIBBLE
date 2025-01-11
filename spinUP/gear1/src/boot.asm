BITS 16
ORG 0x7c00

start:
    ; Setup segments
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7c00

    ; Build disk address packet
    mov ah, 0x42           ; Extended read function
    mov dl, 0x80          ; Drive number (first hard disk)
    mov si, dap           ; Point to DAP
    int 0x13              ; Call BIOS
    jc error              ; Jump if error

    ; Jump to loaded code
    jmp 0:0x7e00         ; Far jump to second stage

error:
    hlt
    jmp error

dap:
    db 0x10              ; DAP size (16 bytes)
    db 0                 ; Reserved
    dw 63               ; Number of sectors
    dw 0                ; Transfer buffer offset
    dw 0x07e0           ; Transfer buffer segment
    dq 1                ; Starting LBA

times 510-($-$$) db 0    ; Pad to 510 bytes
dw 0xaa55               ; Boot signature
