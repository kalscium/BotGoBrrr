//! For recording the bytecode instructions executed by the robot and prints them to the screen

use alloc::vec::Vec;
use safe_vex::vex_rt::io::println;
use crate::bytecode::ByteCode;

/// A struct that holds the recorded bytecode instructions
pub struct Record {
    recorded: Vec<(Vec<ByteCode>, u32)>,    
    last: Vec<ByteCode>,
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
            last: Vec::new(),
            current: Vec::new(),
            cycle: 0,
        }
    }

    /// Adds a tick cycle instruction to the record
    #[inline]
    pub fn cycle(&mut self) {
        // find unique instructions
        let diff = self.current
            .iter()
            .filter(|x| !self.last.contains(x))
            .copied()
            .collect::<Vec<_>>();
        
        // if no instructions are appended then just increase the cycle instead
        if diff.is_empty() {
            self.cycle += 1;
            return;
        }

        // start a new cycle
        self.last = core::mem::take(&mut self.current);
        self.recorded.push((
            diff,
            self.cycle,
        ));
        self.cycle = 0;
    }

    /// Appends new instructions to the current record
    #[inline]
    pub fn append(&mut self, bytecode: &[ByteCode]) {
        self.current.extend_from_slice(bytecode);
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
