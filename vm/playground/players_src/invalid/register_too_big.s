.name    "register too big"
.description "register too big"

l1:	live	%1
	ld	%:l1 , r2
l3:	ldi	%:l2, r17, r4
	sti	r4, %:l2, r2
	add	r2, r3, r2
live:	zjmp	%:l1
	xor	r4, r4, r4
	zjmp	%:l3
