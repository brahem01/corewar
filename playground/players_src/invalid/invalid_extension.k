.name    "invalid extension"
.description "invalid extension"

l1: ld 9,  r2
	ld	%:l1 , r2
live:	zjmp	%:l1
	xor	r4, r4, r4
	zjmp	%:l1
