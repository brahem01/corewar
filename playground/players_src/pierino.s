.name "pierino"
.description "stay alive"

    sti r1, %:a, %1
a:	live %23
    ld %0, r2
	zjmp %:a
