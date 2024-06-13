//! For recording the bytecode instructions executed by the robot and prints them to the screen

use alloc::{vec, vec::Vec};
use safe_vex::vex_rt::io::println;
use crate::bytecode::ByteCode;

/// A struct that holds the recorded bytecode instructions
pub struct Record(Vec<(Vec<ByteCode>, u32)>);

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
        Self(vec![(Vec::new(), 0)])
    }

    /// Adds a tick cycle instruction to the record
    #[inline]
    pub fn cycle(&mut self) {
        let last = self.0.last_mut();

        // if there is already a cycle instruction just increment it by one
        if let Some((_, c)) = last {
            *c += 1;
            return;
        }

        // if not, then just push a new one and update cycle bounds
        self.0.push((Vec::new(), 0));
    }

    /// Appends new instructions to the record
    #[inline]
    pub fn append(&mut self, bytecode: &[ByteCode]) {
        // get the insts of the last cycle
        let default = (Vec::new(), 0);
        let (slice, _) = &self.0.get(self.0.len().saturating_sub(2)).unwrap_or(&default);
        
        // if there is a change between cycles, then push the change onto the bytecode stack
        let insts: Vec<&ByteCode> = bytecode
            .iter()
            .filter(|x| !slice.contains(x))
            .collect();

        // push the changes onto the stack
        for inst in insts {
            println!("\x1b[36;1mexecuted\x1b[0m {inst}");
            self.0.last_mut().unwrap().0.push(*inst);
        }
    }

    /// Flushes the record (removes all insts) and prints it to the `stdout`
    #[inline]
    pub fn flush(&mut self) {
        // header
        println!("\x1b[34;1m<<< \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m>>>\x1b[0m");

        // contents
        for cycle in self.0.iter() {
            for inst in cycle.0.iter() {
                println!("{inst}");
            } println!("{}", ByteCode::Cycle(cycle.1));
        }

        // footer
        println!("\x1b[34;1m=== \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m===\x1b[0m");

        // flush self
        *self = Self::new();
    }
}
