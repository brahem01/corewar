.name "pierino"
.description "stay alive"

a:	live %-1
    st r1, r2
    # after execution, r2 should be ffffffff in hex notation
    ld %0, r2
	zjmp %:a

