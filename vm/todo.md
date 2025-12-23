# Game dynamics

* [ ] Use 2 or more players.

---

# The virtual machine

* [ ] If no parameters are passed it will //print a help message.
* [ ] If one of the `.cor` files is corrupted, the VM should exit with an error code, //print a message on stderr, and do not execute the programs.
* [ ] At the start of the battle the VM will //print a welcome message as shown in the example:

```
For this match the players will be:
Player 1 ([X] bytes): [NAME] ([DESCRIPTION])
Player 2 ([X] bytes): [NAME] ([DESCRIPTION])
...
```

* [ ] At the end of the battle, the VM will write the winner (if any) as shown in the example:

```
cycle [X]: The winner is player [X]: [NAME]!
```

> If nobody executed a valid `live` statement the end message should be `cycle [X]: Nobody wins!`.

* [ ] The players will be loaded into the arena starting from the first byte and will be evenly spaced.
* [ ] The VM must handle a `-d [NB_CYCLES]` flag (dump). If specified, the VM stops execution at `NB_CYCLES` and dumps the arena memory in hexadecimal (32 bytes per row).
* [ ] The last program passed will be the first one executed during the cycle.

* [ ] When a new process is forked, it will be placed at the end of the processes and start execution at the start of the next cycle (it will be first executed on the next cycle).
* [X] The VM assumes the binary is in big-endian.

* [ ] Those are the cases where a file is considered corrupted:
  * Wrong signature code.
  * Declared size of the program not matching the actual size.
  * The size of the program is bigger than the maximum allowed size.
  * The total file size is smaller than the minimum size.

* [ ] The entire execution is deterministic: same inputs → same outputs.

---

# Stop process execution

* [X] Every `CYCLE_TO_DIE` the VM will check every process and kill all that did not successfully execute any `live` instruction.
* [X] To avoid infinite games, `CYCLES_TO_DIE` will be decremented by `CYCLE_DELTA` under certain conditions:

  * During the last life loop, if at least `NBR_LIVE` successfully executed.
  * If it has been `MAX_CHECKS` life loops since the last decrement.
* [ ] Smart players may trick another player into making `live` statements, so a player may still execute `live` after all processes are killed.

---

# Parameters types

* [X] The VM will initialize registers to 0 for each player, except `r1` = `-PLAYER_ID` (first player: r1 = -1).
* [ ] All addresses are relative to the current PC of the process.
----> * [ ] Some instructions truncate addresses using `IDX_MOD` to prevent processes from attacking faraway memory directly (balance purposes).

---

# Your player

* [ ] Provide a basic player able to fight and win against `ameba.s`.
* [ ] A config file provides constants for both Assembler and VM (language-agnostic, easy to translate).

---

# Bonus

* [ ] Create a disassembler: binary → `.s`.
* [ ] Create a visualizer: real-time VM state.
* [ ] Add arithmetic operations in Assembly language.
* [ ] Add simple macro system in Assembly language.

---

# Additional notes

* [ ] use the constants instead of hardcoded values
* [ ] Make use of the instruction file. Work on multiple processes simultaneously (support 2+ files).
* [ ] Read about how modern CPUs do the fetch-execute cycle: [https://corewar-docs.readthedocs.io/en/latest/redcode/parser/](https://corewar-docs.readthedocs.io/en/latest/redcode/parser/)
