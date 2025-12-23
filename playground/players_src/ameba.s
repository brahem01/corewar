.name "ameba"
.description "not doing much"

        sti r1,%:hello,%1
        and r1,%0,r1

hello:  live %1
        zjmp %:hello
