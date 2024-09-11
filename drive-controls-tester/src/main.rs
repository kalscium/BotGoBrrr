use std::io::Write;
use drive_controls_tester::controls::{self, DrivingState, JoyStick};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    // setup
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    // state variables
    let mut driving_state = DrivingState::Neutral;
    let mut joystick = JoyStick { x: 0, y: 0 };
    let mut precise = false;
    let mut reversed = false;
    
    // main simulation loop
    loop {
        // user inputs
        for key_event in std::io::stdin().keys() {
            if let Ok(key_event) = &key_event {
                match key_event {
                    Key::Char('k') => joystick.y = (joystick.y as i16 + 8).clamp(-127, 127) as i8,
                    Key::Char('h') => joystick.x = (joystick.x as i16 - 8).clamp(-127, 127) as i8,
                    Key::Char('j') => joystick.y = (joystick.y as i16 - 8).clamp(-127, 127) as i8,
                    Key::Char('l') => joystick.x = (joystick.x as i16 + 8).clamp(-127, 127) as i8,

                    Key::Up => joystick.y = (joystick.y as i16 + 1).clamp(-127, 127) as i8,
                    Key::Left => joystick.x = (joystick.x as i16 - 1).clamp(-127, 127) as i8,
                    Key::Down => joystick.y = (joystick.y as i16 - 1).clamp(-127, 127) as i8,
                    Key::Right => joystick.x = (joystick.x as i16 + 1).clamp(-127, 127) as i8,

                    Key::Esc => std::process::exit(0),

                    Key::Char('p') => precise = !precise,
                    Key::Char('r') => reversed = !reversed,

                    _ => (),
                } break
            }
        }

        // get voltages
        let (ldr, rdr) = controls::gen_drive_inst(joystick, reversed, precise);

        // clear screen
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        print!("{joystick:?}{}", termion::cursor::Goto(1, 2));
        print!("precise: {precise:?}{}", termion::cursor::Goto(1, 3));
        print!("reversed: {reversed:?}{}", termion::cursor::Goto(1, 4));
        print!("driving_state: {driving_state:?}{}", termion::cursor::Goto(1, 5));
        print!("ldr: {ldr:?}{}", termion::cursor::Goto(1, 6));
        print!("rdr: {rdr:?}{}", termion::cursor::Goto(1, 7));

        stdout.flush().unwrap();
    }
}
