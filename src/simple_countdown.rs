use std::cmp::min;
use std::time;
use crate::common;

const MAX_TARGET: u64 = 24*60*60-1;

pub struct SimpleCountdown {
    start_time: time::Instant,
    pause_start_time: time::Instant,
    is_paused: bool,
    total_paused_time: u64,
    target: u64,
}

impl SimpleCountdown {
    pub fn new() -> SimpleCountdown {
        SimpleCountdown {
            start_time: time::Instant::now(),
            pause_start_time: time::Instant::now(),
            is_paused: false,
            total_paused_time: 0,
            target: 0
        }
    }

    pub fn add_time_to_target(&mut self, increment: i32){
        self.target = min(self.target.saturating_add_signed(increment as i64), MAX_TARGET);
    }
    pub fn get_display(&self) ->String{
        //let seconds_elapsed = if self.is_paused{
        //    (self.pause_start_time-self.start_time).as_secs()-self.total_paused_time
        //} else {
        //    (time::Instant::now()-self.start_time).as_secs()-self.total_paused_time
        //};
        common::stringify_seconds(self.target)
    }
}