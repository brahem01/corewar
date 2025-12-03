.name "pierino"
.description "stay alive"

a:	live %-1
    ld %10, r2
    lldi -5, r2, r3
    # after exec, register r3 is 209 in hex notation
    ld %0, r2
	zjmp %:a
