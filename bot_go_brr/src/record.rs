//! For recording the bytecode instructions executed by the robot and prints them to the screen

use core::ops::Range;
use alloc::{vec, vec::Vec};
use safe_vex::vex_rt::io::println;
use crate::bytecode::ByteCode;

/// A struct that holds the recorded bytecode instructions
pub struct Record {
    cycle_bounds: Range<usize>,
    bytecode: Vec<ByteCode>,
}

impl Default for Record {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Record {
    /// Creates a new bytecode record
    #[inline]
    pub fn new() -> Self {
        Self {
            bytecode: vec![ByteCode::Cycle(0)], // to fix offset errors
            cycle_bounds: 0..1,
        }
    }

    /// Adds a tick cycle instruction to the record
    #[inline]
    pub fn cycle(&mut self) {
        let last = self.bytecode.last_mut();

        // if there is already a cycle instruction just increment it by one
        if let Some(ByteCode::Cycle(c)) = last {
            *c += 1;
            return;
        }

        // if not, then just push a new one and update cycle bounds
        self.cycle_bounds.start = self.cycle_bounds.end+1;
        self.cycle_bounds.end = self.bytecode.len();
        self.bytecode.push(ByteCode::Cycle(0));
    }

    /// Appends new instructions to the record
    #[inline]
    pub fn append(&mut self, bytecode: &[ByteCode]) {
        // get the insts of the last cycle
        let slice = &self.bytecode[self.cycle_bounds.clone()];
        
        // if there is a change between cycles, then push the change onto the bytecode stack
        let insts: Vec<&ByteCode> = bytecode
            .iter()
            .filter(|x| !slice.contains(x))
            .collect();

        // push the changes onto the stack
        for inst in insts {
            println!("\x1b[36;1mexecuted\x1b[0m {inst}");
            self.bytecode.push(*inst);
        }
    }

    /// Flushes the record (removes all insts) and prints it to the `stdout`
    #[inline]
    pub fn flush(&mut self) {
        // header
        println!("\x1b[34;1m<<< \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m>>>\x1b[0m");

        // contents
        for inst in &self.bytecode {
            println!("{inst}");
        }

        // footer
        println!("\x1b[34;1m=== \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m===\x1b[0m");

        // flush self
        *self = Self::new();
    }
}
