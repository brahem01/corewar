.name "pierino"
.description "stay alive"

a:	live %-1
    ld %2, r2
    ld %3, r3
    sub r2, r3, r4
    # after execution, r4 should be ffffffff
    ld %0, r2
	zjmp %:a
