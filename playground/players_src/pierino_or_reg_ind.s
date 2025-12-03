.name "pierino"
.description "stay alive"

a:	live %-1
    or r1, -2, r3
    # after this isntruction, r3 should be ffffffff in hex notation 
    ld %0, r2
	zjmp %:a
