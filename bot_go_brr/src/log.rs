//! Functions for logging safely on the robot brain

use alloc::format;
use logic::{log::{Level, Log}, warn};
use safe_vex::{error::PROSErr, fs::{self, FileWrite}, io::println};
use crate::config;

/// An open filestream to the log file
pub struct LogFile(Option<FileWrite>);

/// Flushes all the logical logs and logs them
pub fn logic_flush(logfile: &mut LogFile) {
    // get the logs from the logfile
    let logs = logic::log::LOGGER.lock().flush();

    // iterate through the logs and log them
    for log in logs {
        log_stdout(&log); // log to stdout

        // log errors during logging
        if let Err(err) = log_logfile(&log, logfile) {
            warn!("`PROSErr` encountered while writing to logfile: {err:?}");
        }
    }
}

/// Logs a message to the stdout
pub fn log_stdout(log: &Log) {
    // only log if important enough
    if log.level < config::log::STDOUT_MIN {
        return;
    }

    // generates a prefix based upon log level
    let prefix = match log.level {
        Level::Debug   => "\x1b[35;1mdebug\x1b[0m",
        Level::Info    => "\x1b[34;1minfo\x1b[0m",
        Level::Warning => "\x1b[33;1mwarning\x1b[0m",
    };

    // prints the formatted log
    println!(
        "\x1b[33m{}:{}:{} {prefix} {}",
        log.file,
        log.line,
        log.column,
        log.msg,
    );
}

/// Initialises a new logfile
pub fn logfile_init(path: &str) -> LogFile {
    // check if the sd card is even inserted
    if !fs::is_available() {
        warn!("sd card unavailable");
        return LogFile(None);
    }

    // try to create a new logfile and return it
    let file = FileWrite::create(path);
    match file {
        Ok(file) => LogFile(Some(file)),
        Err(err) => {
            warn!("encountered `PROSErr` while creating logfile: {err:?}");
            LogFile(None)
        }
    }
}

/// Logs a message to the logfile (may fail)
pub fn log_logfile(log: &Log, logfile: &mut LogFile) -> Result<(), PROSErr> {
    // only log if there's an open logfile
    let Some(ref mut file) = logfile.0
        else { return Ok(()) };

    // only log if important enough
    if log.level < config::log::LOGFILE_MIN {
        return Ok(());
    }

    // generate a prefix based upon log level
    let prefix = match log.level {
        Level::Debug => "debug",
        Level::Info  => "info",
        Level::Warning => "warning",
    };

    // format the log
    let formatted = format!(
        "[{}:{}:{}] {prefix} -> {}\n",
        log.file,
        log.line,
        log.column,
        log.msg,
    );

    // try to write to the logfile
    file.write(&formatted)?;

    Ok(())
}
