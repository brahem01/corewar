.name "pierino"
.description "stay alive"

a:	live %-1
    xor -2, -1, r3
    # after execution, r3 should be f7
    ld %0, r2
	zjmp %:a
