use alloc::{boxed::Box, vec::Vec};
use safe_vex::{bot::Bot, context::Context, maybe::Maybe, motor::Motor, port::PortManager, vex_rt::peripherals::Peripherals};
use crate::{append_slice, bytecode::{execute, ByteCode}, config, controls, drive_train::DriveTrain};
#[cfg(record)]
use crate::record::Record;

/// The robot
pub struct Robot {
    #[cfg(record)]
    record: Record,

    /// The drive-train of the robot
    drive_train: DriveTrain,

    /// The conveyor-belt motor of the robot
    belt: Maybe<Motor>,

    /// The bytecode stack (placed in the struct to avoid reallocating)
    bytecode: Vec<ByteCode>,

    /// The previous joystick info
    last_joystick: Option<(i8, i8)>,
}

impl Bot for Robot {
    const TICK_SPEED: u64 = 50;

    #[inline]
    fn new(_: &Peripherals, port_manager: &mut PortManager) -> Self {
        let drive_train = DriveTrain::new(port_manager);

        Self {
            #[cfg(record)]
            record: Record::new(),
            
            drive_train,
            belt: Maybe::new(Box::new(|| unsafe { Motor::new(config::drive::ARM.port, config::drive::GEAR_RATIO, config::drive::UNIT, config::drive::ARM.reverse) }.ok())),

            // load the autonomous bytecode
            bytecode: config::COMP_AUTO.to_vec(),
            last_joystick: None,
        }
    }

    #[inline]
    fn opcontrol(&mut self, context: Context) -> bool {      
        // get drive-inst
        let (drive_inst, joystick) = controls::gen_drive_inst(
            &self.drive_train,
            self.last_joystick.take().unwrap_or((0, 0)), &context.controller );

        // append to bytecode stack
        append_slice(&mut self.bytecode, &drive_inst);

        // execute inst on bytecode stack
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt);

        // append to record
        #[cfg(record)]
        self.record.append(&drive_inst);

        // update internal state
        self.last_joystick = Some(joystick);
        
        false
    }

    #[inline]
    fn autonomous(&mut self, _: Context) -> bool {
        // execute the autonomous bytecode
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt);
        
        false
    }
}
