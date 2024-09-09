use alloc::{boxed::Box, format, {vec::Vec, vec}};
use safe_vex::{adi::new_adi_digital_output, bot::Bot, context::Context, maybe::Maybe, motor::Motor, port::PortManager, vex_rt::{adi::AdiDigitalOutput, peripherals::Peripherals}};
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
    intake: Maybe<Motor>, /// The pneumatics solenoid for the goal grabber
    solenoid: Maybe<AdiDigitalOutput>,

    /// If the solenoid is active or not
    solenoid_active: bool,
    /// The last tick that the solenoid was active
    solenoid_tick: u16,

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

            belt: Maybe::new(Box::new(|| unsafe { Motor::new(config::drive::BELT.port, config::drive::GEAR_RATIO, config::drive::UNIT, config::drive::BELT.reverse) }.ok())),
            intake: Maybe::new(Box::new(|| unsafe { Motor::new(config::drive::INTAKE.port, config::drive::GEAR_RATIO, config::drive::UNIT, config::drive::INTAKE.reverse) }.ok())),
            solenoid: Maybe::new(Box::new(|| unsafe { new_adi_digital_output(config::SOLENOID_PORT) }.ok())),

            // solenoid fields
            solenoid_active: false,
            solenoid_tick: 0,

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
            ByteCode::Belt { voltage: 0 },
            ByteCode::Intake { voltage: 0 },
            ByteCode::Solenoid(false),
        ], &mut self.drive_train, &mut self.belt, &mut self.intake, &mut self.solenoid);

        // update the controller screen and also haptics
        let screen = &mut context.peripherals.master_controller.screen;
        screen.clear();
        screen.print(0, 0, &format!("tick: {};", context.tick));
        screen.print(1, 0, &format!("solanoid: {};", self.solenoid_active));
        
        // get drive-inst
        let drive_inst = controls::gen_drive_inst(&context.controller);

        // get the conveyor belt instruction
        let belt_inst = match (context.controller.r2, context.controller.r1) {
            (true, false) => ByteCode::Belt { voltage: config::drive::BELT_VOLTAGE },
            (false, true) => ByteCode::Belt { voltage: -config::drive::BELT_VOLTAGE },
            (_, _) => ByteCode::Belt { voltage: 0 },
        };

        // get the intake instruction
        let intake_inst = ByteCode::Intake {
            voltage: (12000.0 * context.controller.right_stick.y as f64 / 127.0) as i32,
        };

        // get the solenoid instruction
        let solenoid_inst = ByteCode::Solenoid(
            if context.controller.x && context.tick - self.solenoid_tick >= config::SOLENOID_DELAY { // make sure the button is held down and only every 2 ticks
                self.solenoid_tick = context.tick;
                self.solenoid_active = !self.solenoid_active;

                // update haptics
                if self.solenoid_active {
                    screen.rumble(".");
                }
                
                self.solenoid_active
            } else { self.solenoid_active }
        );

        // append instructions to bytecode stack
        append_slice(&mut self.bytecode, &drive_inst);
        self.bytecode.push(belt_inst);
        self.bytecode.push(intake_inst);
        self.bytecode.push(solenoid_inst);

        // execute bytecode inst on bytecode stack
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt, &mut self.intake, &mut self.solenoid);

        // append to record
        #[cfg(feature = "record")]
        {
            self.record.append(&drive_inst);
            self.record.append(&[belt_inst, intake_inst, solenoid_inst]);
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
        execute(&mut self.bytecode, &mut self.drive_train, &mut self.belt, &mut self.intake, &mut self.solenoid);
        
        false
    }
}
