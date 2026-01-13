# Corewar

A recreation of the legendary 1984 programming game by D. G. Jones, where programs battle for survival in a virtual machine's memory arena.

## Table of Contents

- [Overview](#overview)
- [Game Mechanics](#game-mechanics)
- [Project Structure](#project-structure)
- [Building](#building)
- [Usage](#usage)
- [Assembly Language](#assembly-language)
- [Instruction Set](#instruction-set)
- [File Format](#file-format)
- [Examples](#examples)
- [Development](#development)

## Overview

Corewar is a programming game where players write programs (called "champions" or "players") in a custom assembly language. These programs are then compiled to bytecode and executed in a virtual machine where they compete for survival. The goal is to be the last program executing a `live` instruction before the game ends.

### Key Concepts

- **Virtual Machine (VM)**: Executes player bytecode in a circular memory arena
- **Assembler**: Compiles `.s` assembly files into `.cor` bytecode files
- **Arena**: The shared memory space where programs battle
- **Processes**: Each program can spawn multiple execution threads
- **Registers**: Each process has 16 private 32-bit registers (r1-r16)
- **Carry Flag**: A boolean flag affected by certain operations

## Game Mechanics

### Victory Conditions

The last player to execute a `live` instruction before all processes die wins the match. The VM periodically checks which processes have executed `live` instructions and kills those that haven't.

### Memory Architecture

- **Circular Memory**: Moving past the last address wraps to address 0
- **Relative Addressing**: All addresses are relative to the current Program Counter (PC)
- **IDX_MOD**: Some instructions apply modulo IDX_MOD to limit memory reach

### Process Lifecycle

1. Each player starts with one process at their spawn position
2. Processes can fork to create copies
3. Processes must execute `live` periodically to survive
4. Dead processes are removed during periodic checks
5. Game ends when no processes remain or CYCLE_TO_DIE reaches zero

### Game Progression

- **CYCLE_TO_DIE**: Time between liveness checks
- **CYCLE_DELTA**: Amount CYCLE_TO_DIE decreases
- **NBR_LIVE**: Threshold of live calls to trigger decrease
- **MAX_CHECKS**: Maximum checks before forced decrease

## Project Structure

```
corewar/
├── src/
│   ├── assembler/     # Assembly compiler
│   ├── vm/            # Virtual machine
│   └── disassembler/  # Bytecode decompiler (bonus)
├── players/           # Champion .s files
├── config/            # Configuration constants
└── Makefile
```

## Building

This project uses Rust and Cargo:

```bash
# Build all components in release mode
make build

# Or use cargo directly
cargo build --release
```

Executables will be in `target/release/`:
- `assembler` - Compiles .s to .cor
- `vm` - Executes .cor files
- `disassembler` - Decompiles .cor to .s (bonus)

## Usage

### Assembler

Compile assembly files to bytecode:

```bash
# Using make
make asm ARGS="player.s"

# Direct execution
./target/release/assembler player.s
```

**Output**: Creates `player.cor` in the same directory

**Error Handling**: Exits with error code and descriptive message if compilation fails

### Virtual Machine

Execute compiled champions:

```bash
# Using make
make run ARGS="player1.cor player2.cor"

# Direct execution
./target/release/vm player1.cor player2.cor

# With memory dump at cycle 100
./target/release/vm -d 100 player1.cor player2.cor
```

**Flags**:
- `-d [NB_CYCLES]`: Dump memory state and exit at specified cycle
- `-v`: Verbose mode showing VM state each cycle (reference implementation)

**Player Limit**: Maximum 4 players per match

### Disassembler (Bonus)

Decompile bytecode back to assembly:

```bash
make disasm ARGS="player.cor"
./target/release/disassembler player.cor
```

## Assembly Language

### File Structure

Every `.s` file must contain:

```assembly
.name "Champion Name"
.description "Champion description"

# Your code here
```

### Syntax Rules

- One instruction per line
- Optional label before instruction
- Instruction and parameters separated by whitespace
- Parameters separated by commas
- Comments use `#`

### Parameter Types

| Type | Format | Description | Size |
|------|--------|-------------|------|
| **Register** | `r1` to `r16` | Process-local storage | 1 byte |
| **Direct** | `%42` | Literal value | 2 or 4 bytes* |
| **Indirect** | `42` | Relative memory address | 2 bytes |

*Direct parameters are 2 bytes for instructions with `Has Idx = true`, otherwise 4 bytes

### Labels

Labels mark positions in code for jumps:

```assembly
loop:           # Label on own line
    live %1
    zjmp %:loop # Reference to label

start: sti r1, %:target, %1  # Label before instruction
```

Labels are replaced with relative byte offsets during compilation.

## Instruction Set

| Instruction | Params | Opcode | Cycles | Description |
|-------------|--------|--------|--------|-------------|
| `live` | 1 | 01 | 10 | Notify VM player is alive |
| `ld` | 2 | 02 | 5 | Load value into register |
| `st` | 2 | 03 | 5 | Store register to memory |
| `add` | 3 | 04 | 10 | Add two registers |
| `sub` | 3 | 05 | 10 | Subtract two registers |
| `and` | 3 | 06 | 6 | Bitwise AND |
| `or` | 3 | 07 | 6 | Bitwise OR |
| `xor` | 3 | 08 | 6 | Bitwise XOR |
| `zjmp` | 1 | 09 | 20 | Jump if carry is true |
| `ldi` | 3 | 10 | 25 | Load indirect |
| `sti` | 3 | 11 | 25 | Store indirect |
| `fork` | 1 | 12 | 800 | Create new process |
| `lld` | 2 | 13 | 10 | Long load (no IDX_MOD) |
| `lldi` | 3 | 14 | 50 | Long load indirect |
| `lfork` | 1 | 15 | 1000 | Long fork |
| `nop` | 0-1 | 16 | 2 | No operation |

### Carry Flag Operations

Instructions that modify the carry flag: `ld`, `add`, `sub`, `and`, `or`, `xor`
- Set to `true` if result is zero
- Set to `false` otherwise

Only `zjmp` reads the carry flag (jumps if true).

## File Format

### .cor Binary Structure

```
[4 bytes]  Magic/Signature (0x00EA83F3)
[128 bytes] Program name (null-padded)
[4 bytes]  Code size in bytes
[2048 bytes] Description (null-padded)
[N bytes]  Bytecode instructions
```

All integers are **big-endian**.

### Example: ameba.cor

```assembly
.name "ameba"
.description "not doing much"

    sti r1, %:hello, %1
    and r1, %0, r1
hello:
    live %1
    zjmp %:hello
```

Compiles to:
```
00 ea 83 f3                    # Magic
61 6d 65 62 61 ...             # "ameba" + padding
00 00 00 17                    # Size: 23 bytes
6e 6f 74 20 64 6f 69 6e 67... # "not doing much" + padding
0b 68 01 00 0f 00 01           # sti r1, %15, %1
06 64 01 00 00 00 00 01        # and r1, %0, r1
01 00 00 00 01                 # live %1
09 ff fb                       # zjmp %-5
```

## Examples

### Basic Self-Replicating Player

```assembly
.name "Replicator"
.description "Spawns copies"

start:
    sti r1, %:live_call, %1
    fork %:start
live_call:
    live %1
    zjmp %:live_call
```

### Aggressive Bomber

```assembly
.name "Bomber"
.description "Overwrites opponent memory"

    sti r1, %:live_stmt, %1
loop:
    st r1, 510        # Write to distant memory
    st r1, 515
    live_stmt: live %1
    zjmp %:loop
```

## Development

### Testing

```bash
# Clean compiled players
make clean

# Assemble a player
make asm ARGS="players/ameba.s"

# Run match
make run ARGS="players/ameba.cor players/other.cor"
```

### Debugging

1. Use `-v` flag on reference VM to see execution details
2. Use `-d N` to dump memory at specific cycle
3. Use `hexdump -C player.cor` to inspect bytecode
4. Test against provided reference players

### Provided Test Players

The project includes several test champions in `players/`:
- `ameba.s` - Basic survival example
- Additional test players for validation

### Configuration

All VM and assembler constants are centralized in the config file:
- `MEM_SIZE`: Arena size
- `IDX_MOD`: Address modulo value
- `MAX_PLAYER_SIZE`: Maximum bytecode size
- `REG_NUMBER`: Number of registers (16)
- `CYCLE_TO_DIE`: Initial liveness check interval
- `CYCLE_DELTA`: Decrease amount
- `NBR_LIVE`: Live threshold
- `MAX_CHECKS`: Check limit

## Requirements

- **No external libraries** for core functionality (parsing, execution)
- **Standard library only** for Assembler and VM
- **No memory leaks**
- **No crashes** under any input
- **Deterministic execution** - same inputs always produce same outputs
- **Big-endian** binary format

## Bonus Features

- ✅ Disassembler (.cor → .s)
- ✅ Real-time visualizer
- ✅ Arithmetic operations in assembly
- ✅ Macro system

*Bonuses only evaluated if core Assembler and VM are perfect*

## Resources

- Reference VM and Assembler binaries provided in `playground/`
- Dockerfile for containerized testing
- Configuration file with all constants

## License

Educational project - implementation of D. G. Jones' 1984 Corewar concept.

---

**Note**: This is a complex systems programming project that provides deep insights into CPU architecture, virtual machines, and Von Neumann computing principles. Take time to understand the execution model before diving into implementation.
