# Bytecode
---
The JIT compiled bytecode that the robot executes.

# Ascii Representation
---
- The bytecode instructions have an ascii representation for easy sending through the stdout and input into macros.
- Each bytecode instruction is delimeted by a semi-colon.

**Bytecode ascii mapping:**
```
// Cycle
c <u32>;

// LeftDrive
ld <i32>;

// RightDrive
rd <i32>;

// Belt
b <i32>;

// Intake
i <i32>;
```
