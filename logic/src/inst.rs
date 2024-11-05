//! Autonomous instructions for the robot

use packed_struct::prelude::*;

/// The size (in bytes) of a single packed auton instruction
///
/// Make sure to update the value in the Inst struct too
pub const INST_SIZE: usize = 5;

/// A single autonomous intruction
#[derive(Debug, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="msb0")]
#[packed_struct(endian="msb")]
#[packed_struct(size_bytes=5)]
pub struct Inst {
    /// Required angle for the instruction to execute
    #[packed_field(bits="0..9")]
    req_angle: Integer<i16, packed_bits::Bits::<9>>,

    /// If the belt should be spinning
    #[packed_field(bits="9")]
    act_belt_active: bool,

    /// If the belt should be spinning 'upwards'
    ///
    /// *Ignored if the active bool is false*
    #[packed_field(bit="10")]
    act_belt_up: bool,

    /// If the solenoid should be active or not
    #[packed_field(bit="11")]
    act_solenoid_active: bool,

    /// The 'thrust' *(exp y value)* of the robot
    #[packed_field(bit="12..27")]
    act_thrust: Integer<i16, packed_bits::Bits::<15>>,

    /// How many cycles this instruction lasts for
    #[packed_field(bit="27..40")]
    cycles: Integer<i16, packed_bits::Bits<13>>
}
