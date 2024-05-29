use alloc::{boxed::Box, vec::Vec};
use safe_vex::{bot::Bot, context::Context, maybe::Maybe, motor::Motor, port::PortManager, vex_rt::peripherals::Peripherals};
use crate::{append_slice, bytecode::{execute, ByteCode}, config, controls, drive_train::DriveTrain};
#[cfg(feature = "record")]
use crate::record::Record;

/// The robot
pub struct Robot {
    #[cfg(feature = "record")]
    record: Record,

    /// The drive-train of the robot
    drive_train: DriveTrain,

    /// The conveyor-belt motor of the robot
    belt: Maybe<Motor>,

    /// The bytecode stack (placed in the struct to avoid reallocating)
    bytecode: Vec<ByteCode>,
}

impl Bot for Robot {
    const TICK_SPEED: u64 = 50;

    #[inline]
    fn new(_: &Peripherals, port_manager: &mut PortManager) -> Self {
        let drive_train = DriveTrain::new(port_manager);

        Self {
            #[cfg(feature = "record")]
            record: Record::new(),
            
            drive_train,
            belt: Maybe::new(Box::new(|| unsafe { Motor::new(config::drive::BELT.port, config::drive::GEAR_RATIO, config::drive::UNIT, config::drive::BELT.reverse) }.ok())),

            // load the autonomous bytecode
            #[cfg(feature = "full-autonomous")]
            bytecode: config::autonomous::FULL_AUTO.to_vec(),
            #[cfg(not(feature = "full-autonomous"))]
            bytecode: config::autonomous::MATCH_AUTO.to_vec(),
        }
    }

    #[inline]
    fn opcontrol(&mut self, context: Context) -> bool {      
        // get drive-inst
        let drive_inst = controls::gen_drive_inst(&context.controller);

        // get belt bytecode inst and push it to the bytecode
        let belt_inst = match (context.controller.x, context.controller.b) {
            (true, _) => ByteCode::Belt { voltage: config::drive::BELT_VOLTAGE },
            (_, true) => ByteCode::Belt { voltage: -config::drive::BELT_VOLTAGE },
            (_, _) => ByteCode::Belt { voltage: 0 },
        };

        // append instructions to bytecode stack
        append_slice(&mut self.bytecode, &drive_inst);
        self.bytecode.push(belt_inst);

        // execute bytecode inst on bytecode stack
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt);

        // append to record
        #[cfg(feature = "record")]
        {
            self.record.append(&drive_inst);
            self.record.append(&[belt_inst]);
            self.record.cycle();
        }

        // check if record is flushed
        #[cfg(feature = "record")]
        if context.controller.y {
            self.record.flush();
        }
        
        false
    }

    #[inline]
    fn autonomous(&mut self, _: Context) -> bool {
        // check if there is any instructions left
        if self.bytecode.is_empty() { return true };
        
        // execute the autonomous bytecode
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt);
        
        false
    }
}
