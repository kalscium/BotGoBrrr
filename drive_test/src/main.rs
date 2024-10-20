use std::{thread::sleep, time::Duration};

use gilrs::{Axis, Event, Gilrs};

fn main() {
    // controller manager object
    let mut gilrs = Gilrs::new().unwrap();

    // the current active controller
    let mut active_controller = None;

    // main loop
    loop {
        // Examine new controller events and update the active_controller variable
        while let Some(Event { id, .. }) = gilrs.next_event() {
            active_controller = Some(id);
        }

        // if there isn't a controller connected then just continue
        let active_controller = match active_controller {
            Some(id) => id,
            None => {
                println!("warning: no controller events detected");
                continue
            },
        };

        // get the cached gamepad state
        let gamepad = gilrs.gamepad(active_controller);

        // get the x and y values and make them 'normal' (for nintendo switch controller (change this if you have a diff controller))
        let jx = -gamepad.value(Axis::LeftStickY);
        let jy = -gamepad.value(Axis::LeftStickX);

        // get the voltage values
        let jxv = drive_controls::exp_daniel(jx);
        let jyv = drive_controls::exp_daniel(jy);

        // course correct
        let yaw = 45.0;
        let (jxvc, jyvc) = drive_controls::course_correct(jxv, jyv, yaw);

        // get the final left and right drive voltages
        let (ldr, rdr) = drive_controls::arcade(jxvc as i32, jyvc as i32);

        // clear the screen and print the values
        clearscreen::clear().unwrap();
        println!("\njoystick: ({jx}, {jy})");
        println!("joyvolts: ({jxv}, {jyv})\n");
        println!("yaw: {yaw}");
        println!("correctd: ({jxvc}, {jyvc})\n");
        println!("driveval: ({ldr}, {rdr})");

        // to not burn out my cpu
        sleep(Duration::from_millis(50))
    }
}
