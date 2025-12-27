# 🧩 Corewar — Instruction Implementation TODO

## 🎯 Goal

Implement, test, and validate all Corewar instructions according to the official specification.
Each task should include:

* Opcode decoding
* Parameter validation
* Execution logic
* Carry flag updates (if applicable)
* PC updates (`IDX_MOD` when required)

---

## 🧠 INSTRUCTIONS TODO LIST

### 🔹 `live` (opcode 1)

* [X] Parse 1 direct parameter
* [X] Notify VM that player `-param` is alive
* [X] Increment global `nbr_live` counter
* [X] Mark current process as “alive this cycle”
* [X] No carry modification
* [X] Advance PC by instruction size

---

### 🔹 `ld` (opcode 2)

* [ ] Parse `[Indirect, Direct]`, then `Register`
* [ ] Load value into register
* [ ] Apply `% IDX_MOD` if source is Indirect
* [ ] Set carry to true if value == 0
* [ ] Advance PC accordingly

---

### 🔹 `st` (opcode 3)

* [ ] Parse `Register`, then `[Register, Indirect]`
* [ ] Write register value into destination (arena or register)
* [ ] Apply `% IDX_MOD` if destination is Indirect
* [ ] No carry modification

---

### 🔹 `add` (opcode 4)

* [ ] Parse three `Register` parameters
* [ ] Perform addition: `r3 = r1 + r2`
* [ ] Set carry if result == 0
* [ ] Advance PC

---

### 🔹 `sub` (opcode 5)

* [ ] Parse three `Register` parameters
* [ ] Perform subtraction: `r3 = r1 - r2`
* [ ] Set carry if result == 0
* [ ] Advance PC

---

### 🔹 `and` (opcode 6)

* [ ] Parse `[Register, Indirect, Direct]`, `[Register, Indirect, Direct]`, `Register`
* [ ] Perform bitwise AND
* [ ] Set carry if result == 0
* [ ] Advance PC

---

### 🔹 `or` (opcode 7)

* [ ] Parse `[Register, Indirect, Direct]`, `[Register, Indirect, Direct]`, `Register`
* [ ] Perform bitwise OR
* [ ] Set carry if result == 0
* [ ] Advance PC

---

### 🔹 `xor` (opcode 8)

* [ ] Parse `[Register, Indirect, Direct]`, `[Register, Indirect, Direct]`, `Register`
* [ ] Perform bitwise XOR
* [ ] Set carry if result == 0
* [ ] Advance PC

---

### 🔹 `zjmp` (opcode 9)

* [ ] Parse `Direct` parameter
* [ ] If carry == true, jump `(PC + param % IDX_MOD)`
* [ ] Otherwise, advance PC normally
* [ ] Does not modify carry

---

### 🔹 `ldi` (opcode 10)

* [ ] Parse `[Register, Indirect, Direct]`, `[Register, Direct]`, `Register`
* [ ] Compute address `(param1 + param2) % IDX_MOD`
* [ ] Load value from arena into register
* [ ] No carry modification
* [ ] Advance PC

---

### 🔹 `sti` (opcode 11)

* [ ] Parse `Register`, `[Register, Indirect, Direct]`, `[Register, Direct]`
* [ ] Compute address `(param2 + param3) % IDX_MOD`
* [ ] Store register value at computed address
* [ ] No carry modification
* [ ] Advance PC

---

### 🔹 `fork` (opcode 12)

* [ ] Parse 1 `Direct` parameter
* [ ] Clone current process (deep copy)
* [ ] Set new PC = `(current_PC + param % IDX_MOD)`
* [ ] No carry modification

---

### 🔹 `lld` (opcode 13)

* [ ] Parse `[Indirect, Direct]`, then `Register`
* [ ] Load value into register (NO `% IDX_MOD`)
* [ ] Set carry if result == 0
* [ ] Advance PC

---

### 🔹 `lldi` (opcode 14)

* [ ] Parse `[Register, Indirect, Direct]`, `[Register, Direct]`
* [ ] Compute address `(param1 + param2)` (NO `% IDX_MOD`)
* [ ] Load value into register
* [ ] No carry modification
* [ ] Advance PC

---

### 🔹 `lfork` (opcode 15)

* [ ] Parse `Direct` parameter
* [ ] Clone process (deep copy)
* [ ] Set new PC = `(current_PC + param)` (NO `% IDX_MOD`)
* [ ] No carry modification

---

### 🔹 `nop` (opcode 16)

* [ ] Parse optional `Register` parameter
* [ ] Do nothing
* [ ] Advance PC normally

---

## 🧾 GENERAL TASKS

* [ ] Implement **parameter parsing** (`pcode` decoding)
* [ ] Handle **IDX_MOD** truncation correctly
* [ ] Validate **register ranges (1–REG_NUMBER)**
* [ ] Handle **carry flag** logic consistently
* [ ] Implement **process duplication** (for fork & lfork)
* [ ] Implement **PC movement & wrapping** (circular arena)
* [ ] Add **unit tests** for all instructions

