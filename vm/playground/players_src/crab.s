.name "walker"
.description "moving around"

st r1,r8         # size: 4 bytes, writes the player_id in r8

# write a pit
# - add %0,%0,r1
# 00000110 10100100 [00000000 00000000 00000000 00000000] [00000000 00000000 00000000 00000000] 00000001 -> 11 bytes
# |        |        |                                     |                                     | arg3: reg1
# |        |        |                                     | arg2: 0
# |        |        | arg1: 0
# |        | pcode: direct,direct,register
# | add opcode

# - zjmp %-11
# 00000001 00001001 11111111 11110101 -> 1 + 3 bytes
# |        |        | arg1: direct with has_idx: -11
# |        | zjmp opcode
# | arg3 of add, reg 1

ld %111411200,r2     # size: 5 bytes, load first 4 bytes of add instruction
ld %1,r3             # size: 5 bytes, load last 4 bytes of add instruction
ld %17432565,r4        # size: 5 bytes, load zjmp + 1 byte left padding

# write a live 
sti r3,%12,%0         # size: 7 bytes, write the opcode of live + 3 bytes left padding
sti r8,%9,%0          # size: 7 bytes, write the last 4 bytes of live

nop r1
nop r1

sti r2,%28,%0
sti r16,%25,%0
sti r16,%22,%0
sti r4,%17,%0
