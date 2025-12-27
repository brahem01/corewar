.name "zjmp_test hahahahaha"
.description "tests forward and backward zjmp"

# Start with some initial instructions
ld %5, r2        # r2 = 5
ld %10, r3       # r3 = 10
and r2, r3, r4   # r3 = r1 & r2 = 0b101 & 0b1010 = 0
                 # Carry will be set to true because result is 0

# This zjmp should jump because carry is true
zjmp %10           # Jump forward 4 bytes (skips next instruction)

ld %99, r4        # Should be skipped

# Backward jump
zjmp %-7          # Jump back 6 bytes (to previous 'ld %10, r2')

# Some more instructions to see PC wrapping
add r2, r3, r5    # r5 = 5 + 10 = 15
sub r5, r3, r6    # r6 = 15 - 10 = 5

