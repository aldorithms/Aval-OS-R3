; Set up the segment registers
xor ax, ax
mov ds, ax
mov es, ax
mov ss, ax
mov sp, 0x7c00

; Load the kernel from disk
mov bx, 0x8000
mov dh, 0x00
mov dl, 0x00
mov cx, 0x0002
mov ah, 0x02
int 0x13

; Jump to the kernel entry point
jmp 0x8000:0x0000
