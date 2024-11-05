//! Platform independant logging

use core::mem::MaybeUninit;
use alloc::{boxed::Box, string::String};
use spin::Mutex;

/// The size of the log buffer
const LOG_BUFFER_SIZE: usize = 32;

/// Different levels of logging
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warning = 2,
}

/// The logger of the program (should only be initialised by this module)
#[derive(Debug)]
pub struct Logger {
    len: usize,
    buffer: [MaybeUninit<Log>; LOG_BUFFER_SIZE],
}

/// The global logic logger
pub static LOGGER: Mutex<Logger> = Mutex::new(Logger {
    len: 0,
    buffer: [const { MaybeUninit::uninit() }; LOG_BUFFER_SIZE],
});

/// A single log from the robot logic
#[derive(Debug, Clone)]
pub struct Log {
    pub level: Level,
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
    pub msg: String,
}

impl Logger {
    /// Returns the internal log buffer and flushes it (cleans it)
    pub fn flush(&mut self) -> Box<[Log]> {
        let buffer = self.buffer[..self.len]
            .iter()
            .map(|x| unsafe {
                x.assume_init_ref().clone()
            })
            .collect::<Box<[_]>>();
        self.len = 0;

        buffer
    }

    /// Pushes a log onto the logger if there's space (returns the log if there is no space)
    pub fn log(&mut self, log: Log) -> Result<(), Log> {
        // if no space then return the log
        if self.len == LOG_BUFFER_SIZE {
            return Err(log);
        }

        // otherwise insert the log and increment the counter
        self.buffer[self.len] = MaybeUninit::new(log);
        self.len += 1;
        Ok(())
    }
}

/// Logs a warning to the logic logger
#[macro_export]
macro_rules! warn {
    ($format:literal $(,$args:expr)* $(,)?) => {{
        let _ = $crate::log::LOGGER
            .lock()
            .log($crate::log::Log {
                level: $crate::log::Level::Warning,
                file: core::file!(),
                line: core::line!(),
                column: core::column!(),
                msg: alloc::format!($format, $($args),*),
            });
    }}
}

/// Logs information to the logic logger
#[macro_export]
macro_rules! info {
    ($format:literal $(,$args:expr)* $(,)?) => {{
        let _ = $crate::log::LOGGER
            .lock()
            .log($crate::log::Log {
                level: $crate::log::Level::Info,
                file: core::file!(),
                line: core::line!(),
                column: core::column!(),
                msg: alloc::format!($format, $($args),*),
            });
    }}
}

/// Logs debug information to the logic logger
#[macro_export]
macro_rules! debug {
    ($format:literal $(,$args:expr)* $(,)?) => {{
        let _ = $crate::log::LOGGER
            .lock()
            .log($crate::log::Log {
                level: $crate::log::Level::Debug,
                file: core::file!(),
                line: core::line!(),
                column: core::column!(),
                msg: alloc::format!($format, $($args),*),
            });
    }}
}

