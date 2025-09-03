; Simple Corewar test program

start:          ; Label definition
    live %1     ; Live instruction with direct value
    ld %34, r2  ; Load direct value into register
    st r2, 45   ; Store register into memory (indirect)
    add r2, r3, r4 ; Add two registers into a third
    sub r4, r2, r1 ; Subtract
    and r1, %10, r2 ; AND
    or r2, r3, r4   ; OR
    xor r4, r1, r2  ; XOR

loop:
    zjmp %start      ; Jump to start if carry
    ldi r2, %5, r3  ; Load with indirect addressing
    sti r3, r2, %2  ; Store with indirect
    fork %loop       ; Fork process
    lld %10, r1      ; Long load
    lldi r1, %2, r3  ; Long load indirect
    lfork %start     ; Long fork
    nop              ; No operation
