.name    "number to big"
.description "number to big"

l1: ld 9999999999999,  r2
	ld	%:l1 , r2
live:	zjmp	%:l1
	xor	r4, r4, r4
	zjmp	%:l1
