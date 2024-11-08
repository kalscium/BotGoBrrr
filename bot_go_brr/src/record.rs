//! Functions recording autonomous routines

use alloc::format;
use logic::{inst::{Inst, INST_SIZE}, packed_struct::PackedStructSlice, warn};
use safe_vex::{error::PROSErr, fs::{self, FileWrite}};
use crate::drive;

/// A file where the autonomous routine gets recorded to
pub struct Record(Option<FileWrite>);

impl Record {
    /// Creates a new record file at the specified path
    pub fn new(path: &str) -> Result<Self, PROSErr> {
        // check if the sd card is even inserted
        if !fs::is_available() {
            warn!("sd card unavailable");
            return Ok(Record(None));
        }

        // create a new record file and write the initial `[` (rust syntax for an array)
        let mut file = FileWrite::create(path)?;
        file.write("[\n")?;

        // return self
        Ok(Record(Some(file)))
    }

    /// Creates a new record file at the specified path and ignores (but reports) any errors that occur
    pub fn new_ignore(path: &str) -> Self {
        match Self::new(path) {
            Ok(this) => this,
            Err(err) => {
                warn!("`PROSErr` encountered while creating a new record file: {err:?}");
                Record(None)
            },
        }
    }

    /// Writes another autonomous instruction to the auton routine recordfile
    pub fn record(&mut self, thrust: i32, belt_inst: Option<bool>, solenoid_inst: bool) {
        // try get the record file, otherwise do nothing
        let Some(ref mut file) = self.0
        else {
            return;
        };

        // get the yaw
        let yaw = drive::get_yaw();

        // create the inst
        let inst = Inst {
            req_angle: (yaw as i16).into(),
            thrust: (thrust as i16).into(),
            act_belt_active: belt_inst.is_some(),
            act_belt_up: belt_inst.unwrap_or(false),
            act_solenoid_active: solenoid_inst,
        };

        // pack the inst
        let mut packed = [0u8; INST_SIZE];
        inst.pack_to_slice(&mut packed).unwrap();

        // format the inst to a string
        let formatted = format!(
            "{:04x?}, // {:?}\n",
            packed,
            inst,
        );

        // write it to the file and report errors
        if let Err(err)  = file.write(&formatted) {
            warn!("`PROSErr` encountered while writing to record file: {err:?}");
        };
    }
}

impl Drop for Record {
    fn drop(&mut self) {
        if let Some(ref mut file) = self.0 {
            // write the closing `]`
            let _ = file.write("]\n");
        }
    }
}
