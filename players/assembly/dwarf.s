.name "dwarf"
.description "bomber that kills ameba"

        sti r1, %:live, %1
        and r1, %0, r1

live:   live %1
        ld %2, r2
        ld %4, r3
        fork %:live

bomb:   sti r1, %:bomb, r2
        add r2, r3, r2
        zjmp %:bomb

