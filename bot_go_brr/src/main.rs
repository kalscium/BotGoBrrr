#![no_std]
#![no_main]

safe_vex::entry! {
    initialize => bot_go_brr::initialize::initialize();
    opcontrol => bot_go_brr::opcontrol::opcontrol();
    autonomous => bot_go_brr::autonomous::autonomous();
}
