.name "pierino"
.description "stay alive"

a:	live %-1
    ld %123, r2
    sti r1, %23, %1
    # after execution, ff ff ff ff at byte 36
    ld %0, r2
	zjmp %:a

