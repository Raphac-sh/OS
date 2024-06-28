bits 16
org 0x7c00

boot:
    mov si, message
    mov ah, 0x0e

.loop:
    lodsb
    cmp al, 0
    je halt
    int 0x10
    jmp .loop

halt:
    hlt

message:
    db "Hello world!", 0

times 510-($-$$) db 0
    dw 0xAA55
