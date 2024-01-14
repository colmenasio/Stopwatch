mod simple_timer;

use simple_timer::SimpleTimer;
use std::cell::RefCell;
use std::rc::Rc;
use std::time;
use slint::{Timer, TimerMode};

slint::include_modules!();

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {

    let main_window = Main_Window::new().unwrap();
    let main_simple_timer = Rc::new(RefCell::new(SimpleTimer::new()));


    let weak = main_window.as_weak();
    let simple_timer = main_simple_timer.clone();
    main_window.global::<Logic>().on_press_l_button(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<Logic>();
        let mut borrowed_timer = simple_timer.borrow_mut();
        if ! logic_module.get_is_running() {
            borrowed_timer.start_timer();
            logic_module.set_is_running(true);
            logic_module.set_time_str("00:00:00".into());
        } else if logic_module.get_is_running() {
            borrowed_timer.change_pause_state();
            logic_module.set_time_str(borrowed_timer.get_timer_display().into());
            logic_module.set_is_paused(!logic_module.get_is_paused());
        };
    });


    let weak = main_window.as_weak();
    main_window.global::<Logic>().on_press_r_button(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<Logic>();
        if logic_module.get_is_running() {
            logic_module.set_is_running(false);
            logic_module.set_is_paused(false);
        };
    });


    let weak = main_window.as_weak();
    let simple_timer = main_simple_timer.clone();
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, time::Duration::from_secs(1), move || {
        let app = weak.unwrap();
        let logic_module = app.global::<Logic>();
        let borrowed_timer = simple_timer.borrow();
        if ! logic_module.get_is_running() {
            return
        }
        logic_module.set_time_str(borrowed_timer.get_timer_display().into())
    });

    main_window.run().unwrap();
}
