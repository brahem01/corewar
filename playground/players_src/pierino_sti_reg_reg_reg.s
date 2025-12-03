.name "pierino"
.description "stay alive"

a:	live %-1
    ld %123, r2
    sti r1, r2, r3
    # after execution, ff ff ff ff at byte 135
    ld %0, r2
	zjmp %:a

