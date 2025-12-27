.name "pierino"
.description "stay alive"

a:	live %-1
    or 6, 7, r3
    # after this instruction, r3 should be 392 in hex notation
    ld %0, r2
	zjmp %:a
