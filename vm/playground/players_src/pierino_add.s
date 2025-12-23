.name "pierino"
.description "stay alive"

a:	live %-1
    ld %2, r2
    ld %3, r3
    add r2, r3, r4
    # after this instruction, r4 is 5
    ld %0, r3
	zjmp %:a
