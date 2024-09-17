use alloc::{vec::Vec, vec};
use safe_vex::{bot::Bot, context::Context, port::PortManager, vex_rt::peripherals::Peripherals};
use crate::{append_slice, bytecode::{execute, ByteCode}, config, controls, drive_train::DriveTrain, reverse_in_place};
#[cfg(feature = "record")]
use crate::record::Record;

/// The robot
pub struct Robot {
    #[cfg(feature = "record")]
    record: Record,

    /// The drive-train of the robot
    drive_train: DriveTrain, /// The conveyor-belt motor of the robot

    /// The bytecode stack (placed in the struct to avoid reallocating)
    bytecode: Vec<ByteCode>,
}
 impl Bot for Robot {
    const TICK_SPEED: u64 = 50;
    // const TICK_SPEED: u64 = 1000; // for testing purposes only

    #[inline]
    fn new(_: &Peripherals, port_manager: &mut PortManager) -> Self {
        Self {
            #[cfg(feature = "record")]
            record: Record::new(),
            drive_train: DriveTrain::new(port_manager),
            

            // load the autonomous bytecode
            #[cfg(feature = "full-autonomous")]
            bytecode: reverse_in_place(config::autonomous::FULL_AUTO.to_vec()),
            #[cfg(not(feature = "full-autonomous"))]
            bytecode: reverse_in_place(config::autonomous::MATCH_AUTO.to_vec()),
        } }

    fn opcontrol(&mut self, context: Context) -> bool {      
        // clear old instructions
        self.bytecode.clear();
        execute(&mut vec![
            ByteCode::LeftDrive { voltage: 0 },
            ByteCode::RightDrive { voltage: 0 },
        ], &mut self.drive_train);
        
        // get drive-inst
        let drive_inst = controls::gen_drive_inst(&context.controller);

        // append instructions to bytecode stack & execute them
        append_slice(&mut self.bytecode, &drive_inst);
        execute(&mut self.bytecode, &mut self.drive_train);

        // append to record
        #[cfg(feature = "record")]
        {
            self.record.append(&drive_inst);
            self.record.cycle();
        }

        // check if record is flushed
        #[cfg(feature = "record")]
        if context.controller.y {
            self.record.flush();
        }
        
        false
    }

    fn autonomous(&mut self, _: Context) -> bool {
        // check if there is any instructions left
        if self.bytecode.is_empty() { return true };
        
        // execute the autonomous bytecode
        execute(&mut self.bytecode, &mut self.drive_train);
        
        false
    }
}
