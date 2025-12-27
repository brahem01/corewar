.name "pierino"
.description "stay alive"

a:	live %-1
    and 7, r1, r3
    # after this instruction, r3 should be ffff9000 (in hex notation)
    ld %0, r2
	zjmp %:a
