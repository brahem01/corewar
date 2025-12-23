.name "pierino"
.description "stay alive"

a:	live %-1
    or -2, r1, r3
    # after this instruction, r3 should be ffffffff in hex notation
    ld %0, r2
	zjmp %:a
