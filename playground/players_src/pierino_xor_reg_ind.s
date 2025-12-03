.name "pierino"
.description "stay alive"

a:	live %-1
    xor r1, 0, r3
    # after execution, r3 should be fffff72b
    ld %0, r2
	zjmp %:a
