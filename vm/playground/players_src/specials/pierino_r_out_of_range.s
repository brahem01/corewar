.name "pierino"
.description "register out of range!"

    sti r1, %:a, %1
a:	live %23
    ld %0, r223
	zjmp %:a
