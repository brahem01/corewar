; Simple Corewar test program (label-free)
.name "brahem"
.comment "the ultimate warrior"
    live %1         ; Live instruction with direct value
    ld %34, r2        ; Load direct value into register
    st r2, 45         ; Store register into memory (indirect)
    add r2, r3, r4    ; Add two registers into a third
    sub r4, r2, r1    ; Subtract
    and r1, %10, r2   ; AND
    or r2, r3, r4     ; OR
    xor r4, r1, r2    ; XOR

    zjmp %10          ; Jump to offset 10 (instead of %start)
    ldi r2, %5, r3    ; Load with indirect addressing
    sti r3, r2, %2    ; Store with indirect
    fork %20          ; Fork new process (offset 20 instead of %loop)
    lld %10, r1       ; Long load
    lldi r1, %2, r3   ; Long load indirect
    lfork %30         ; Long fork (offset 30 instead of %start)
