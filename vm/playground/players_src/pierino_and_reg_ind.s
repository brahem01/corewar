.name "pierino"
.description "stay alive"

a:	live %-1
    and r1, 1, r3
    # after this instruction, r3 should be 7401 (in hex notation)
    ld %0, r2
	zjmp %:a
