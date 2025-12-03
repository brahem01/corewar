.name "single_live"
.description "just executes live once"

live %1
ld -8, r2
ld %2, r3
ld %10, r4
st r4, r5
#st r5, 11
add r4, r5, r6
sub r3, r6, r7


# --- Bitwise operations ---
and r4, r5, r8  # r8 = r4 & r5 = 10 & 10 = 10
or  r2, r3, r9  # r9 = r2 | r3 = -8 | 2 = -6  
xor r6, r7, r10 # r10 = r6  r7 = 20 ^ -18 = -6

