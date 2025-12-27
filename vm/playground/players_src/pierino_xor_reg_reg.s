.name "pierino"
.description "stay alive"

a:	live %-1
    ld %16, r2
    xor r1, r2, r3
    # after execution, r3 should be ffffffef
    ld %0, r2
	zjmp %:a
