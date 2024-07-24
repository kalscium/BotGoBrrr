//! For recording the bytecode instructions executed by the robot and prints them to the screen

use alloc::vec::Vec;
use safe_vex::vex_rt::io::println;
use crate::bytecode::ByteCode;

/// A struct that holds the recorded bytecode instructions
pub struct Record {
    recorded: Vec<(Vec<ByteCode>, u32)>,    
    current: Vec<ByteCode>,
    cycle: u32,
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
            recorded: Vec::new(),
            current: Vec::new(),
            cycle: 0,
        }
    }

    /// Adds a tick cycle instruction to the record
    #[inline]
    pub fn cycle(&mut self) {
        // if no instructions are appended then just increase the cycle instead
        if self.current.is_empty() {
            self.cycle += 1;
            return;
        }

        // start a new cycle
        self.recorded.push((
            core::mem::take(&mut self.current),
            core::mem::replace(&mut self.cycle, 0),
        ));
    }

    /// Appends new instructions to the record
    #[inline]
    pub fn append(&mut self, bytecode: &[ByteCode]) {
        // get the insts of the last cycle
        let default = Vec::new();
        let slice = self.recorded.last().map(|x| &x.0).unwrap_or(&default);
        
        // if there is a change between cycles, then push the change onto the bytecode stack
        let insts: Vec<&ByteCode> = bytecode
            .iter()
            .filter(|x| !slice.contains(x))
            .collect();

        // push the changes onto the stack
        for inst in insts {
            // println!("\x1b[36;1mexecuted\x1b[0m {inst}");
            self.current.push(*inst);
        }
    }

    /// Flushes the record (removes all insts) and prints it to the `stdout`
    #[inline]
    pub fn flush(&mut self) {
        // header
        println!("\x1b[34;1m<<< \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m>>>\x1b[0m");

        // contents
        for (insts, cycle) in self.recorded.iter() {
            println!("{:?};", ByteCode::Cycle(*cycle));
            for inst in insts {
                println!("{inst:?};");
            }
        }

        // footer
        println!("\x1b[34;1m=== \x1b[33mEXECUTED BYTECODE INSTRUCTIONS \x1b[34;1m===\x1b[0m");

        // flush self
        *self = Self::new();
    }
}
