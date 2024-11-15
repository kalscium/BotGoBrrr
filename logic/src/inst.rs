//! Autonomous instructions for the robot

use packed_struct::prelude::*;
use crate::{info, warn};

/// The size (in bytes) of a single packed auton instruction
///
/// Make sure to update the value in the Inst struct too
pub const INST_SIZE: usize = 4;

/// A single autonomous intruction
#[derive(Debug, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="msb0")]
#[packed_struct(endian="msb")]
#[packed_struct(size_bytes=4)]
pub struct Inst {
    /// Required angle for the instruction to execute
    #[packed_field(bits="0..9")]
    pub req_angle: Integer<i16, packed_bits::Bits::<9>>,

    /// Required odom y coordinate (in mm)
    #[packed_field(bits="9..22")]
    pub req_odom_y: Integer<i16, packed_bits::Bits::<9>>,

    /// If the belt should be spinning
    #[packed_field(bits="22")]
    pub act_belt_active: bool,

    /// If the belt should be spinning 'upwards'
    ///
    /// *Ignored if the active bool is false*
    #[packed_field(bit="23")]
    pub act_belt_up: bool,

    /// If the solenoid should be active or not
    #[packed_field(bit="24")]
    pub act_solenoid_active: bool,

    /// If the doinker should be spinning
    #[packed_field(bits="25")]
    pub act_doinker_active: bool,

    /// If the doinker should be spinning 'upwards'
    ///
    /// *Ignored if the active bool is false*
    #[packed_field(bit="26")]
    pub act_doinker_up: bool,
}

/// A stored autonomous routine
pub struct AutonRoutine<'a> {
    /// Internal reference to the packed instructions slice
    insts: &'a [[u8; INST_SIZE]],
    /// The current inst index into the inst slice
    idx: usize,
}

impl<'a> AutonRoutine<'a> {
    /// Creates a new autonomous routine iterator over a slice of packed insts
    pub fn new(insts: &'a [[u8; INST_SIZE]]) -> Self {
        Self {
            insts,
            idx: 0,
        }
    }
}

impl Iterator for AutonRoutine<'_> {
    type Item = Inst;

    fn next(&mut self) -> Option<Self::Item> {
        // get the packed inst
        let Some(packed) =
            self.insts.as_ref().get(self.idx)
        else {
            return None;
        };

        // try unpack the inst
        let Ok(inst) =
            Inst::unpack(packed)
        else {
            warn!("packed inst `{packed:?}` in auton routine at idx `{}` is invalid", self.idx);
            info!("skipping the rest of autonomous");
            return None;
        };

        // update the index and return inst
        self.idx += 1;
        Some(inst)
    }
}
