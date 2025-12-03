.name "pierino_test"
.description "stay alive and test zjmp"

a:  live %-1

    ld %2, r2       # r2 = 2
    ld %3, r3       # r3 = 3
    add r2, r3, r4  # r4 = 5, carry = false

    sub r2, r2, r5  # r5 = 2 - 2 = 0 -> carry becomes true
    and r4, r4, r6  # r6 = 5 & 5 = 5 -> carry = false
    or r5, r5, r7   # r7 = 0 | 0 = 0 -> carry becomes true

    zjmp %:a        # Will jump back because carry is now true

