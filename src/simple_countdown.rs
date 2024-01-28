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
            target: 0,
        }
    }

    pub fn add_time_to_target(&mut self, increment: i32){
        self.target = min(self.target.saturating_add_signed(increment as i64), MAX_TARGET);
    }

    pub fn start_timer(& mut self){
        self.start_time = time::Instant::now();
        self.is_paused = false;
        self.total_paused_time = 0;
    }

    pub fn change_pause_state(& mut self){
        if self.is_paused {
            self.unpause_timer();
        } else {
            self.pause_timer();
        }
    }

    fn pause_timer(& mut self){
        self.pause_start_time = time::Instant::now();
        self.is_paused = true;
    }

    fn unpause_timer(& mut self){
        self.total_paused_time += self.pause_start_time.elapsed().as_secs();
        self.is_paused = false;
    }

    fn get_elapsed_seconds(& self) -> u64{
        if self.is_paused {
            (self.pause_start_time-self.start_time).as_secs()-self.total_paused_time
        } else {
            (time::Instant::now()-self.start_time).as_secs()-self.total_paused_time
        }
    }

    pub fn is_finished(&self) -> bool {
        self.get_elapsed_seconds() >= self.target
    }

    pub fn get_display(&self) -> String{
        let seconds_elapsed = self.get_elapsed_seconds();
        if seconds_elapsed >= self.target {
            "00:00:00".to_string()
        } else {
            common::stringify_seconds(self.target-seconds_elapsed)
        }
    }

    pub fn get_target_display(&self) -> String{
        common::stringify_seconds(self.target)
    }
}