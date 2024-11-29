//! Common actions and functions used during autonomous

use logic::info;
use safe_vex::rtos;

/// Waits for the specified milliseconds
pub fn wait(ms: u32) {
    info!("routine: waiting {ms}ms");
    rtos::task_delay(ms);
}
