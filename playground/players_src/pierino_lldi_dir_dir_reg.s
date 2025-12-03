.name "pierino"
.description "stay alive"

a:	live %-1
    lldi %1, %10, r3
    # after exec, register r3 is 209 in hex notation
    ld %0, r2
	zjmp %:a
