.name "pierino"
.description "stay alive"

a:	live %-1
    lld -4, r2
    # afer exec r2 is set to ffffffff in hex
    ld %0, r2
	zjmp %:a
