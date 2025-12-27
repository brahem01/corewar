.name "pierino"
.description "stay alive"

a:	live %-1
    and 6, -4, r3
    # after this instruction, r3 should be 302 (hex notation)
    ld %0, r2
	zjmp %:a
