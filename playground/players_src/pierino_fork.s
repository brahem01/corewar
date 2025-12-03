.name "pierino"
.description "stay alive"

    fork %:a
    # after exec, two twin process should run
a:	live %-1
    ld %0, r2
	zjmp %:a

