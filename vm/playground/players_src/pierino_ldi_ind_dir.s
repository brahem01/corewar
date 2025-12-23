.name "pierino"
.description "stay alive"

a:	live %-1
    ldi -4, %1, r3
    # after this instruction, r3 should be ae4fffc in hex notation
    ld %0, r2
	zjmp %:a
