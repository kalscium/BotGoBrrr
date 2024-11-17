//! Functions recording autonomous routines

use alloc::format;
use logic::{debug, inst::{Inst, INST_SIZE}, packed_struct::PackedStructSlice, warn};
use safe_vex::{error::PROSErr, fs::{self, FileWrite}};
use crate::{config, drive};

/// A file where the autonomous routine gets recorded to
pub struct Record {
    file: Option<FileWrite>,
    current: Option<Inst>,
    prev_angle_delta: f32,
}

impl Record {
    /// Creates a new record file at the specified path
    pub fn new(path: &str) -> Result<Self, PROSErr> {
        // check if the sd card is even inserted
        if !fs::is_available() {
            warn!("sd card unavailable");
            return Ok(Record {
                file: None,
                current: None,
                prev_angle_delta: 0.,
            });
        }

        // create a new record file and return the record
        let file = FileWrite::create(path)?;
        Ok(Record {
            file: Some(file),
            current: None,
            prev_angle_delta: 0.,
        })
    }

    /// Creates a new record file at the specified path and ignores (but reports) any errors that occur
    pub fn new_ignore(path: &str) -> Self {
        match Self::new(path) {
            Ok(this) => this,
            Err(err) => {
                warn!("`PROSErr` encountered while creating a new record file: {err:?}");
                Record {
                    file: None,
                    current: None,
                    prev_angle_delta: 0.,
                }
            },
        }
    }

    /// Writes another autonomous instruction to the auton routine recordfile
    pub fn record(
        &mut self,
        thrust: i32,
        belt_inst: Option<bool>,
        doinker_inst: Option<bool>,
        solenoid_inst: bool,
    ) {
        // try get the record file, otherwise do nothing
        let Some(ref mut file) = self.file
        else {
            return;
        };

        // get the yaw
        let yaw = drive::get_yaw();

        // create a new inst
        let new_inst = Inst {
            req_angle: (yaw as i16).into(),
            act_belt_active: belt_inst.is_some(),
            act_belt_up: belt_inst.unwrap_or(false),
            act_doinker_active: doinker_inst.is_some(),
            act_doinker_up: doinker_inst.unwrap_or(false),
            act_solenoid_active: solenoid_inst,
            thrust: (thrust as i16).into(),
        };

        // get the 'current' inst
        let Some(ref mut current) = self.current
        else {
            // otherwise set the current inst to the new inst and return
            debug!("first recorded instruction encountered");
            self.current = Some(new_inst);
            return;
        };

        // find the deltas between the current and the new inst
        let angle_delta = logic::drive::low_angle_diff(yaw, i16::from(current.req_angle) as f32);

        // get if the robot is turning the same direction or moving the same direction
        let turn_same = self.prev_angle_delta.is_sign_positive() == angle_delta.is_sign_positive();

        // update the 'previous' deltas
        self.prev_angle_delta = angle_delta;

        // check if the new instruction is moving in the same direction as the 'current' one, is larger than the angle precision (not the same angle) and also has the same actions
        if
            // must be turning the same direction
            turn_same
            // must be turning
            && maths::absf(angle_delta) > config::auton::ANGLE_PRECISION
            // thrust, belt and doinker must no be active
            && (i16::from(new_inst.thrust) == 0 && !new_inst.act_belt_active && !new_inst.act_doinker_active)
            // make sure the solenoid doesn't change
            && new_inst.act_solenoid_active == current.act_solenoid_active
            // must have the same actions
            && (new_inst.thrust, new_inst.act_belt_active, new_inst.act_belt_up, new_inst.act_solenoid_active) == (current.thrust, current.act_belt_active, current.act_belt_up, current.act_solenoid_active)
        {
            // update the current inst's required angle to the new one and return
            debug!("recorded (and compressed) a similar looking instruction");
            current.req_angle = new_inst.req_angle;
            return;
        }

        // pack the 'current' inst
        let mut packed = [0u8; INST_SIZE];
        current.pack_to_slice(&mut packed).unwrap();

        // format the inst to a string
        let formatted = format!("{packed:?}, // {current:?}\n");

        // write it to the file and report errors
        if let Err(err) = file.write(&formatted) {
            warn!("`PROSErr` encountered while writing to record file: {err:?}");
        };

        // update the current inst
        debug!("recorded a different instruction");
        *current = new_inst;
    }
}
