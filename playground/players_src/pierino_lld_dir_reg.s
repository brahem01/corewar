.name "pierino"
.description "stay alive"

a:	live %-1
    lld %1234, r2
    # afet exec r2 is equal to 4d2 in hex
    ld %0, r2
	zjmp %:a
