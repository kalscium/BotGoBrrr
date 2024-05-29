//! For recording the bytecode instructions executed by the robot and prints them to the screen

use alloc::vec::Vec;
use safe_vex::vex_rt::io::println;
use crate::bytecode::ByteCode;

/// A struct that holds the recorded bytecode instructions
pub struct Record(Vec<ByteCode>);

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
        Self(Vec::new())
    }

    /// Adds a tick cycle instruction to the record
    #[inline]
    pub fn cycle(&mut self) {
        let last = self.0.last_mut();

        // if there is already a cycle instruction just increment it by one
        if let Some(ByteCode::Cycle(c)) = last {
            *c += 1;
            return;
        }

        // if not, then just push a new one
        self.0.push(ByteCode::Cycle(0));
    }

    /// Appends new instructions to the record
    #[inline]
    pub fn append(&mut self, bytecode: &[ByteCode]) {
        // get the insts of the last cycle
        let mut bounds = 0..0;
        for (i, inst) in self.0.iter().enumerate().rev() {
            if let ByteCode::Cycle(_) = inst {
                if bounds.end == 0 {
                    bounds.end = i;
                } else {
                    bounds.start = i+1;
                } }
        }
        
        // if there is a change between cycles, then push the change onto the bytecode stack
        let slice = &self.0[bounds];
        let insts: Vec<&ByteCode> = bytecode
            .iter()
            .filter(|x| !slice.contains(x))
            .collect();

        // push the changes onto the stack
        for inst in insts {
            println!("\x1b[36;1mexecuted\x1b[0m {inst}");
            self.0.push(*inst);
        }
    }

    /// Flushes the record (removes all insts) and prints it to the `stdout`
    #[inline]
    pub fn flush(&mut self) {
        // header
        println!("\x1b[34;1m<<< \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m>>>\x1b[0m");

        // contents
        for inst in &self.0 {
            println!("{inst}");
        }

        // footer
        println!("\x1b[34;1m=== \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m===\x1b[0m");

        // flush self
        *self = Self::new();
    }
}
