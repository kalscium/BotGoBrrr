use alloc::{boxed::Box, vec::Vec, vec};
use safe_vex::{bot::Bot, context::Context, maybe::Maybe, motor::Motor, port::PortManager, vex_rt::peripherals::Peripherals};
use crate::{append_slice, bytecode::{execute, ByteCode}, config, controls, drive_train::DriveTrain, reverse_in_place};
#[cfg(feature = "record")]
use crate::record::Record;

/// The robot
pub struct Robot {
    #[cfg(feature = "record")]
    record: Record,

    /// The drive-train of the robot
    drive_train: DriveTrain, /// The conveyor-belt motor of the robot
    belt: Maybe<Motor>,
    /// The motor of the robot's goal scorer (for once the conveyor places the donut on the goal)
    inserter: Maybe<Motor>,
    /// The motor on the robot that grabs the goal
    graber: Maybe<Motor>,

    /// The bytecode stack (placed in the struct to avoid reallocating)
    bytecode: Vec<ByteCode>,
}
 impl Bot for Robot { const TICK_SPEED: u64 = 50;
    // const TICK_SPEED: u64 = 1000; // for testing purposes only

    #[inline]
    fn new(_: &Peripherals, port_manager: &mut PortManager) -> Self {
        let drive_train = DriveTrain::new(port_manager);

        Self {
            #[cfg(feature = "record")]
            record: Record::new(),
            drive_train,

            belt: Maybe::new(Box::new(|| unsafe { Motor::new(config::drive::BELT.port, config::drive::GEAR_RATIO, config::drive::UNIT, config::drive::BELT.reverse) }.ok())),
            inserter: Maybe::new(Box::new(|| unsafe { Motor::new(config::drive::INSERTER.port, config::drive::GEAR_RATIO, config::drive::UNIT, config::drive::INSERTER.reverse) }.ok())),
            graber: Maybe::new(Box::new(|| unsafe { Motor::new(config::drive::GRABER.port, config::drive::GEAR_RATIO, config::drive::UNIT, config::drive::GRABER.reverse) }.ok())),

            // load the autonomous bytecode
            #[cfg(feature = "full-autonomous")]
            bytecode: reverse_in_place(config::autonomous::FULL_AUTO.to_vec()),
            #[cfg(not(feature = "full-autonomous"))]
            bytecode: reverse_in_place(config::autonomous::MATCH_AUTO.to_vec()),
        }
    }

    #[inline]
    fn opcontrol(&mut self, context: Context) -> bool {      
        // clear old instructions
        self.bytecode.clear();
        execute(&mut vec![
            ByteCode::LeftDrive { voltage: 0 },
            ByteCode::RightDrive { voltage: 0 },
            ByteCode::Belt { voltage: 0 },
            ByteCode::Inserter { voltage: 0 },
            ByteCode::Graber { voltage: 0 }
        ], &mut self.drive_train, &mut self.belt, &mut self.inserter, &mut self.graber);
        
        // get drive-inst
        let drive_inst = controls::gen_drive_inst(&context.controller);

        // get the conveyor belt instruction
        let belt_inst = match (context.controller.x, context.controller.b) {
            (true, false) => ByteCode::Belt { voltage: config::drive::BELT_VOLTAGE },
            (false, true) => ByteCode::Belt { voltage: -config::drive::BELT_VOLTAGE },
            (_, _) => ByteCode::Belt { voltage: 0 },
        };

        // get the graber instruction
        let graber_inst = match (context.controller.r1, context.controller.r2) {
            (true, false) => ByteCode::Graber { voltage: config::drive::GRABER_VOLTAGE_UP },
            (false, true) => ByteCode::Graber { voltage: -config::drive::GRABER_VOLTAGE_DOWN },
            (_, _) => ByteCode::Graber { voltage: 0 },
        };

        // get the inserter instruction
        let inserter_inst = ByteCode::Inserter {
            voltage: (12000.0 * context.controller.right_stick.y as f64 / 127.0) as i32,
        };

        // append instructions to bytecode stack
        append_slice(&mut self.bytecode, &drive_inst);
        self.bytecode.push(belt_inst);
        self.bytecode.push(inserter_inst);
        self.bytecode.push(graber_inst);

        // execute bytecode inst on bytecode stack
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt, &mut self.inserter, &mut self.graber);

        // append to record
        #[cfg(feature = "record")]
        {
            self.record.append(&drive_inst);
            self.record.append(&[belt_inst, inserter_inst, graber_inst]);
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
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt, &mut self.inserter, &mut self.graber);
        
        false
    }
}
