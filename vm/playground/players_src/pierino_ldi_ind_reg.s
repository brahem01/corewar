.name "pierino"
.description "stay alive"

a:	live %-1
    ld %10, r2
    ldi -4, r2, r3
    # after this instruction, r3 should be 209 in hex notation
    ld %0, r2
	zjmp %:a
