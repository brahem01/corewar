.name "pierino"
.description "stay alive"

a:	live %-1
    lldi 2, %10, r3
    # after exec, register r3 is 209ff in hex notation
    ld %0, r2
	zjmp %:a
