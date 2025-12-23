# Corewar VM (Rust)

I’m working on this project: [Corewar description](https://github.com/01-edu/public/tree/master/subjects/corewar).

The goal is to build a **Virtual Machine** and an **Assembler** for the Corewar game.

* The **Assembler** takes assembly code (`.s`) and turns it into binary programs (`.cor`).
* The **Virtual Machine** loads those `.cor` files into a shared circular memory (the arena) and runs them cycle by cycle.

> Note: I’m only working on the **VM** — the assembler is being implemented by a peer [Assembler](https://github.com/brahem01/corewar/tree/main/assembler).

---

## Game dynamics

* Each program (player) starts with a single process.
* A process has its own **registers**, a **program counter (PC)**, and a **carry flag**.
* Players can execute instructions to:

  * move through memory,
  * copy or modify data,
  * create new processes,
  * and call `live` to prove they’re still alive.

The VM periodically checks which processes executed `live`. If a process hasn’t, it dies. The last player to say it’s alive wins the match.

---

## Implementation (Rust)

I’m building the VM step by step:

* memory management with a circular arena
* processes with registers and program counters
* instruction set decoding and execution
* cycle scheduling and process life/death checks
* loading players and handling game flow

For now, I’m only using the Rust **standard library**, so there are no external dependencies to worry about.

---

## Testing

The project includes a **playground** with a reference assembler, VM, and test players.
You can run the project with:

```sh
make run
```

This will launch the VM with a simple example setup.

---

## Progress

You can track what’s completed and what’s pending in the [`todo.md`](./todo.md) file.
