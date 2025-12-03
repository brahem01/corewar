.name "pierino"
.description "stay alive"

a:	live %-1
    ld 517, r2
    # after this instruciton, r2 should be 290 in hex notation
    ld %0, r2
	zjmp %:a
