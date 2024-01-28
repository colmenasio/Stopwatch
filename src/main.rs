#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod simple_timer;
mod simple_countdown;
mod common;

use simple_timer::SimpleTimer;
use std::cell::RefCell;
use std::rc::Rc;
use std::time;
use slint::{Timer, TimerMode};
use crate::simple_countdown::SimpleCountdown;

slint::include_modules!();

fn main() {

    let main_window = Main_Window::new().unwrap();
    let main_simple_timer = Rc::new(RefCell::new(SimpleTimer::new()));
    let main_simple_countdown = Rc::new(RefCell::new(SimpleCountdown::new()));

    // TIMER LOGIC CALLBACKS //

    // left button
    let weak = main_window.as_weak();
    let simple_timer = main_simple_timer.clone();

    main_window.global::<TLogic>().on_press_l_button(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<TLogic>();
        let mut borrowed_timer = simple_timer.borrow_mut();

        if ! logic_module.get_is_running() {
            borrowed_timer.start_timer();
            logic_module.set_is_running(true);
            logic_module.set_time_str("00:00:00".into());
        } else {
            borrowed_timer.change_pause_state();
            logic_module.set_is_paused(!logic_module.get_is_paused());
        };
    });

    // right button
    let weak = main_window.as_weak();

    main_window.global::<TLogic>().on_press_r_button(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<TLogic>();

        if logic_module.get_is_running() {
            logic_module.set_is_running(false);
            logic_module.set_is_paused(false);
        };
    });

    // do a time display update
    let weak = main_window.as_weak();
    let simple_timer = main_simple_timer.clone();

    main_window.global::<TLogic>().on_update_time(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<TLogic>();
        let borrowed_timer = simple_timer.borrow_mut();

        logic_module.set_time_str(borrowed_timer.get_display().into())
    });

    // COUNTDOWN CALLBACKS //

    // l button
    let weak = main_window.as_weak();
    let simple_countdown = main_simple_countdown.clone();

    main_window.global::<CLogic>().on_press_l_button(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<CLogic>();
        let mut borrowed_countdown = simple_countdown.borrow_mut();

        if ! logic_module.get_is_running() {
            borrowed_countdown.start_timer();
            logic_module.set_is_running(true);
            logic_module.set_time_str("00:00:00".into());
        } else {
            borrowed_countdown.change_pause_state();
            logic_module.set_is_paused(!logic_module.get_is_paused());
        };
    });

    // r button
    let weak = main_window.as_weak();

    main_window.global::<CLogic>().on_press_r_button(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<CLogic>();

        if logic_module.get_is_running() {
            logic_module.set_is_running(false);
            logic_module.set_is_paused(false);
            logic_module.set_is_finished(false);

        };
    });


    // add or substract time to the target time when inputing with the +- buttons
    let simple_countdown = main_simple_countdown.clone();

    main_window.global::<CLogic>().on_add_time_to_target(move |increment| {
        let mut borrowed_countdown = simple_countdown.borrow_mut();

        borrowed_countdown.add_time_to_target(increment)
    });

    // do a time display update

    let weak = main_window.as_weak();
    let simple_countdown = main_simple_countdown.clone();

    main_window.global::<CLogic>().on_update_time(move || {
        let app = weak.unwrap();
        let logic_module = app.global::<CLogic>();
        let borrowed_countdown = simple_countdown.borrow();

        // TODO add logic to determine which thing to update
        logic_module.set_time_str(borrowed_countdown.get_display().into());
        logic_module.set_inactive_str(borrowed_countdown.get_target_display().into());
        if logic_module.get_is_running() && borrowed_countdown.is_finished() {
            logic_module.set_is_finished(true);
        }
    });

    // MAIN APP LOOP //

    let weak = main_window.as_weak();
    let timer = Timer::default();

    timer.start(TimerMode::Repeated, time::Duration::from_secs(1), move || {
        let app = weak.unwrap();
        app.global::<MainLogic>().invoke_update_all();
    });

    // RUN THE APP //

    main_window.run().unwrap();
}
