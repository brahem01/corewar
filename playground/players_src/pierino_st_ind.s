.name "pierino"
.description "stay alive"

a:	live %-1
    st r1, 16
    # after this instruction, ff ff ff ff at byte 21
    ld %0, r2
	zjmp %:a
