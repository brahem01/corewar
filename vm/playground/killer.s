.name "process_killer"
.description "cuts the jump loop"

start:
    ld %0, r1
bomb:
    st r1, %:hello
    st r1, %:hello+1
    live %1
    zjmp %:bomb

