.name "pierino"
.description "stay alive"

a:	live %-1
    ld %16, r2
    or r3, r2, r3
    # after this instruction, r3 should be 10 in hex notation
    ld %0, r2
	zjmp %:a
